use std::collections::{BTreeMap, HashMap, HashSet};

use serde_json::Value;
use sqlx::{FromRow, PgPool, types::Json};

use crate::{
    error::AppError,
    models::exam::{
        AnswerValue, CaseStudy, CaseStudySection, ChoiceSpec, ClientInfoResponse,
        DeleteQuestionNoteResponse, DragDropSpec, ExamAnswerRecord, ExamDetail, ExamQuestion,
        ExamQuestionPayload, ExamSummary, ExamTopic, FlagQuestionRequest, FlagQuestionResponse,
        HotspotSpec, NormalizedExamListParams, PaginatedExamsResponse, QuestionFilter,
        QuestionImage, QuestionNavItem, QuestionNavStatus, QuestionNote, QuestionType,
        ResetExamResponse, ResetQuestionResponse, SubmitAnswerRequest, VoteNoteResponse,
    },
};

macro_rules! with_exam_list_cte {
    ($tail:literal) => {
        concat!(
            r#"
WITH question_counts AS (
    SELECT exam_id, COUNT(*)::bigint AS total_questions
    FROM questions
    GROUP BY exam_id
),
answer_counts AS (
    SELECT exam_id, COUNT(*)::bigint AS answered_count
    FROM client_answers
    WHERE $1::text IS NOT NULL AND client_id = $1::text
    GROUP BY exam_id
),
exam_stats AS (
    SELECT
        e.id,
        e.code,
        e.title,
        e.vendor,
        COALESCE(qc.total_questions, 0)::bigint AS total_questions,
        COALESCE(ac.answered_count, 0)::bigint AS answered_count,
        cep.last_question_index,
        TO_CHAR(e.updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD') AS updated_at,
        e.updated_at AS sort_updated_at
    FROM exams e
    LEFT JOIN question_counts qc ON qc.exam_id = e.id
    LEFT JOIN answer_counts ac ON ac.exam_id = e.id
    LEFT JOIN client_exam_progress cep
        ON cep.exam_id = e.id AND $1::text IS NOT NULL AND cep.client_id = $1::text
),
filtered AS (
    SELECT *
    FROM exam_stats
    WHERE (
        $2::text IS NULL
        OR code ILIKE $2 ESCAPE '\'
        OR title ILIKE $2 ESCAPE '\'
        OR vendor ILIKE $2 ESCAPE '\'
    )
    AND (
        $3::text = 'all'
        OR ($3::text = 'not_started' AND answered_count <= 0)
        OR ($3::text = 'completed' AND total_questions > 0 AND answered_count >= total_questions)
        OR ($3::text = 'in_progress' AND answered_count > 0 AND answered_count < total_questions)
    )
)
"#,
            $tail
        )
    };
}

const COUNT_EXAMS_SQL: &str = with_exam_list_cte!("SELECT COUNT(*)::bigint FROM filtered");
const LIST_EXAMS_SQL: &str = with_exam_list_cte!(
    "SELECT id, code, title, vendor, total_questions, answered_count,
            last_question_index, updated_at
     FROM filtered
     ORDER BY sort_updated_at DESC, id ASC
     LIMIT $4 OFFSET $5"
);

/// Escapes SQL `LIKE`/`ILIKE` metacharacters and wraps the term in `%...%`.
fn build_ilike_pattern(raw: &str) -> String {
    let mut escaped = String::with_capacity(raw.len() + 2);
    escaped.push('%');
    for ch in raw.chars() {
        match ch {
            '\\' | '%' | '_' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    escaped.push('%');
    escaped
}

/// Fetches a paginated, searchable, status-filtered list of exams for the given client.
pub async fn list_exams(
    pool: &PgPool,
    client_id: Option<&str>,
    params: &NormalizedExamListParams,
) -> Result<PaginatedExamsResponse, AppError> {
    let search_pattern = params.search.as_deref().map(build_ilike_pattern);
    let status = params.status.as_str();

    let total: i64 = sqlx::query_scalar(COUNT_EXAMS_SQL)
        .bind(client_id)
        .bind(search_pattern.as_deref())
        .bind(status)
        .fetch_one(pool)
        .await?;

    let total_pages = if total == 0 {
        1
    } else {
        (total + params.limit - 1) / params.limit
    };
    let safe_page = params.page.min(total_pages);
    let offset = (safe_page - 1) * params.limit;

    let items: Vec<ExamSummary> = sqlx::query_as(LIST_EXAMS_SQL)
        .bind(client_id)
        .bind(search_pattern.as_deref())
        .bind(status)
        .bind(params.limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok(PaginatedExamsResponse {
        items,
        total,
        page: safe_page,
        limit: params.limit,
        total_pages,
    })
}

const GET_EXAM_HEADER_SQL: &str = r#"
SELECT
    e.id,
    e.code,
    e.title,
    e.vendor,
    (SELECT COUNT(*)::bigint FROM questions q WHERE q.exam_id = e.id) AS total_questions,
    (
        SELECT COUNT(*)::bigint
        FROM client_answers ca
        WHERE $2::text IS NOT NULL
          AND ca.client_id = $2::text
          AND ca.exam_id = e.id
    ) AS answered_count,
    cep.last_question_index,
    TO_CHAR(e.updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD') AS updated_at,
    e.description,
    e.description_vi
FROM exams e
LEFT JOIN client_exam_progress cep
    ON cep.exam_id = e.id
   AND $2::text IS NOT NULL
   AND cep.client_id = $2::text
WHERE e.id = $1
"#;

const GET_EXAM_TOPICS_SQL: &str = r#"
SELECT
    t.id,
    t.name,
    t.name_vi,
    COUNT(q.id)::bigint AS question_count,
    COUNT(ca.question_id)::bigint AS answered_count,
    MIN(q.question_index) AS start_question_index,
    CASE
        WHEN $3::integer IS NOT NULL
             AND COALESCE(BOOL_OR(q.question_index = $3::integer), FALSE)
        THEN $3::integer
        ELSE NULL
    END AS continue_question_index
FROM topics t
LEFT JOIN questions q
    ON q.exam_id = t.exam_id
   AND q.topic_id = t.id
LEFT JOIN client_answers ca
    ON ca.question_id = q.id
   AND $2::text IS NOT NULL
   AND ca.client_id = $2::text
WHERE t.exam_id = $1
GROUP BY t.exam_id, t.id, t.name, t.name_vi, t.sort_order
ORDER BY t.sort_order ASC, t.id ASC
"#;

#[derive(Debug, FromRow)]
struct ExamHeaderRow {
    id: String,
    code: String,
    title: String,
    vendor: String,
    total_questions: i64,
    answered_count: i64,
    last_question_index: Option<i32>,
    updated_at: String,
    description: String,
    description_vi: Option<String>,
}

/// Fetches full metadata and per-topic progress for a single exam.
pub async fn get_exam_overview(
    pool: &PgPool,
    exam_id: &str,
    client_id: Option<&str>,
) -> Result<ExamDetail, AppError> {
    let header: ExamHeaderRow = sqlx::query_as(GET_EXAM_HEADER_SQL)
        .bind(exam_id)
        .bind(client_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("exam '{exam_id}' not found")))?;

    let topics: Vec<ExamTopic> = sqlx::query_as(GET_EXAM_TOPICS_SQL)
        .bind(exam_id)
        .bind(client_id)
        .bind(header.last_question_index)
        .fetch_all(pool)
        .await?;

    Ok(ExamDetail {
        summary: ExamSummary {
            id: header.id,
            code: header.code,
            title: header.title,
            vendor: header.vendor,
            total_questions: header.total_questions,
            answered_count: header.answered_count,
            last_question_index: header.last_question_index,
            updated_at: header.updated_at,
        },
        description: header.description,
        description_vi: header.description_vi,
        topics,
    })
}

const GET_SCOPED_QUESTIONS_SQL: &str = r#"
SELECT
    q.id,
    q.topic_id,
    q.question_index,
    ca.submitted_answer,
    ca.earned_points,
    ca.max_points AS answer_max_points,
    ca.is_correct,
    (cf.question_id IS NOT NULL) AS is_flagged
FROM questions q
LEFT JOIN client_answers ca
    ON ca.question_id = q.id
   AND $2::text IS NOT NULL
   AND ca.client_id = $2::text
LEFT JOIN client_question_flags cf
    ON cf.question_id = q.id
   AND $2::text IS NOT NULL
   AND cf.client_id = $2::text
WHERE q.exam_id = $1
  AND ($3::text IS NULL OR q.topic_id = $3::text)
  AND ($4::text IS NULL OR q.case_study_id = $4::text)
  AND (
      $5::text = 'all'
      OR ($5::text = 'wrong' AND ca.is_correct = FALSE)
      OR ($5::text = 'flagged' AND cf.question_id IS NOT NULL)
  )
ORDER BY q.question_index ASC
"#;

const COUNT_FLAGGED_QUESTIONS_SQL: &str = r#"
SELECT COUNT(*)::bigint
FROM client_question_flags
WHERE $1::text IS NOT NULL
  AND client_id = $1::text
  AND exam_id = $2
"#;

const GET_QUESTION_DETAIL_SQL: &str = r#"
SELECT
    id,
    topic_id,
    case_study_id,
    question_index,
    question_type,
    prompt,
    prompt_vi,
    spec,
    answer_key,
    max_points,
    explanation,
    explanation_vi,
    keywords,
    keywords_vi,
    images
FROM questions
WHERE id = $1
"#;

const GET_CASE_STUDY_SQL: &str = r#"
SELECT
    id,
    title,
    title_vi,
    sections,
    images
FROM case_studies
WHERE exam_id = $1 AND id = $2
"#;

const UPSERT_LAST_QUESTION_SQL: &str = r#"
INSERT INTO client_exam_progress (client_id, exam_id, last_question_index, updated_at)
VALUES ($1, $2, $3, NOW())
ON CONFLICT (client_id, exam_id) DO UPDATE SET
    last_question_index = EXCLUDED.last_question_index,
    updated_at = NOW()
"#;

#[derive(Debug, FromRow)]
struct ScopedQuestionRow {
    id: String,
    topic_id: String,
    question_index: i32,
    submitted_answer: Option<Json<AnswerValue>>,
    earned_points: Option<i16>,
    answer_max_points: Option<i16>,
    is_correct: Option<bool>,
    is_flagged: bool,
}

#[derive(Debug, FromRow)]
struct QuestionDetailRow {
    id: String,
    topic_id: String,
    case_study_id: Option<String>,
    question_index: i32,
    question_type: QuestionType,
    prompt: String,
    prompt_vi: Option<String>,
    spec: Value,
    answer_key: Option<Json<AnswerValue>>,
    max_points: i16,
    explanation: String,
    explanation_vi: Option<String>,
    keywords: Vec<String>,
    keywords_vi: Vec<String>,
    images: Json<Vec<QuestionImage>>,
}

#[derive(Debug, FromRow)]
struct CaseStudyRow {
    id: String,
    title: String,
    title_vi: Option<String>,
    sections: Json<Vec<CaseStudySection>>,
    images: Json<Vec<QuestionImage>>,
}

/// Loads one practice question (optionally scoped to topic/case_study/filter) and records `last_question_index`.
pub async fn get_exam_question(
    pool: &PgPool,
    exam_id: &str,
    client_id: Option<&str>,
    topic_id: Option<&str>,
    case_study_id: Option<&str>,
    filter: QuestionFilter,
    requested_q: Option<i32>,
) -> Result<ExamQuestionPayload, AppError> {
    let mut exam = get_exam_overview(pool, exam_id, client_id).await?;

    let scoped: Vec<ScopedQuestionRow> = sqlx::query_as(GET_SCOPED_QUESTIONS_SQL)
        .bind(exam_id)
        .bind(client_id)
        .bind(topic_id)
        .bind(case_study_id)
        .bind(filter.as_str())
        .fetch_all(pool)
        .await?;

    if scoped.is_empty() {
        return Err(AppError::NotFound(format!(
            "no matching questions found in exam '{exam_id}'"
        )));
    }

    let selected_pos = if let Some(q) = requested_q {
        scoped.iter().position(|row| row.question_index == q).unwrap_or(0)
    } else if topic_id.is_none()
        && case_study_id.is_none()
        && filter == QuestionFilter::All
        && let Some(last_q) = exam.summary.last_question_index
    {
        scoped
            .iter()
            .position(|row| row.question_index == last_q)
            .unwrap_or(0)
    } else {
        0
    };

    let selected = &scoped[selected_pos];
    let current_index = selected.question_index;

    if let Some(cid) = client_id {
        ensure_client_id(pool, cid).await?;

        sqlx::query(UPSERT_LAST_QUESTION_SQL)
            .bind(cid)
            .bind(exam_id)
            .bind(current_index)
            .execute(pool)
            .await?;

        exam.summary.last_question_index = Some(current_index);
        for topic in &mut exam.topics {
            topic.continue_question_index = if topic.id == selected.topic_id {
                Some(current_index)
            } else {
                None
            };
        }
    }

    let answer = match (
        &selected.submitted_answer,
        selected.earned_points,
        selected.answer_max_points,
        selected.is_correct,
    ) {
        (Some(submitted), Some(earned_points), Some(max_points), Some(is_correct)) => {
            Some(ExamAnswerRecord {
                question_id: selected.id.clone(),
                submitted_answer: submitted.0.clone(),
                earned_points,
                max_points,
                is_correct,
            })
        }
        _ => None,
    };

    let nav_items = scoped
        .iter()
        .map(|row| QuestionNavItem {
            index: row.question_index,
            status: match row.is_correct {
                None => QuestionNavStatus::Unanswered,
                Some(true) => QuestionNavStatus::Correct,
                Some(false) => QuestionNavStatus::Incorrect,
            },
            is_flagged: row.is_flagged,
        })
        .collect();

    let detail: QuestionDetailRow = sqlx::query_as(GET_QUESTION_DETAIL_SQL)
        .bind(&selected.id)
        .fetch_one(pool)
        .await?;

    let case_study = if let Some(cs_id) = detail.case_study_id.as_deref() {
        let cs_row: Option<CaseStudyRow> = sqlx::query_as(GET_CASE_STUDY_SQL)
            .bind(exam_id)
            .bind(cs_id)
            .fetch_optional(pool)
            .await?;
        cs_row.map(|row| CaseStudy {
            id: row.id,
            title: row.title,
            title_vi: row.title_vi,
            sections: row.sections.0,
            images: row.images.0,
        })
    } else {
        None
    };

    let flagged_count: i64 = sqlx::query_scalar(COUNT_FLAGGED_QUESTIONS_SQL)
        .bind(client_id)
        .bind(exam_id)
        .fetch_one(pool)
        .await?;

    let question = ExamQuestion {
        id: detail.id,
        topic_id: detail.topic_id,
        case_study_id: detail.case_study_id,
        index: detail.question_index,
        question_type: detail.question_type,
        prompt: detail.prompt,
        prompt_vi: detail.prompt_vi,
        spec: detail.spec,
        answer_key: detail.answer_key.map(|key| key.0),
        max_points: detail.max_points,
        explanation: detail.explanation,
        explanation_vi: detail.explanation_vi,
        keywords: detail.keywords,
        keywords_vi: detail.keywords_vi,
        images: detail.images.0,
    };

    let answered_count = exam.summary.answered_count;
    let total_in_scope = scoped.len() as i64;

    Ok(ExamQuestionPayload {
        exam,
        total_in_scope,
        current_index,
        question,
        case_study,
        answer,
        is_flagged: selected.is_flagged,
        nav_items,
        answered_count,
        flagged_count,
    })
}

const GET_QUESTION_FOR_GRADING_SQL: &str = r#"
SELECT
    question_index,
    question_type,
    spec,
    answer_key,
    max_points
FROM questions
WHERE exam_id = $1 AND id = $2
"#;

const UPSERT_CLIENT_ANSWER_SQL: &str = r#"
INSERT INTO client_answers (
    client_id, question_id, exam_id, question_index,
    submitted_answer, earned_points, max_points, is_correct, answered_at
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
ON CONFLICT (client_id, question_id) DO UPDATE SET
    question_index = EXCLUDED.question_index,
    submitted_answer = EXCLUDED.submitted_answer,
    earned_points = EXCLUDED.earned_points,
    max_points = EXCLUDED.max_points,
    is_correct = EXCLUDED.is_correct,
    answered_at = NOW()
"#;

const UPSERT_CLIENT_PROGRESS_ON_ANSWER_SQL: &str = r#"
INSERT INTO client_exam_progress (
    client_id, exam_id, last_question_index, updated_at
)
VALUES ($1, $2, $3, NOW())
ON CONFLICT (client_id, exam_id) DO UPDATE SET
    last_question_index = EXCLUDED.last_question_index,
    updated_at = NOW()
"#;

#[derive(Debug, FromRow)]
struct QuestionForGradingRow {
    question_index: i32,
    question_type: QuestionType,
    spec: Value,
    answer_key: Option<Json<AnswerValue>>,
    max_points: i16,
}

/// Validates and grades a submitted answer against `spec` and `answer_key` for all 5 question types.
pub(crate) fn grade_answer(
    question_id: &str,
    question_type: QuestionType,
    spec: &Value,
    answer_key: Option<&AnswerValue>,
    max_points: i16,
    submitted: &AnswerValue,
) -> Result<(AnswerValue, i16, bool), AppError> {
    match question_type {
        QuestionType::Simulation => Err(AppError::BadRequest(format!(
            "question '{question_id}' is a simulation without gradable input"
        ))),
        QuestionType::SingleChoice | QuestionType::MultipleChoice => {
            let AnswerValue::Choice(raw_selected) = submitted else {
                return Err(AppError::BadRequest(format!(
                    "question '{question_id}' expects an array of selected option IDs"
                )));
            };
            let Some(AnswerValue::Choice(expected)) = answer_key else {
                return Err(AppError::Config(format!(
                    "question '{question_id}' is missing a valid choice answer_key"
                )));
            };

            let choice_spec: ChoiceSpec = serde_json::from_value(spec.clone()).map_err(|err| {
                AppError::Config(format!("invalid choice spec on '{question_id}': {err}"))
            })?;
            let valid_ids: HashSet<&str> =
                choice_spec.options.iter().map(|opt| opt.id.as_str()).collect();

            let mut seen = HashSet::with_capacity(raw_selected.len());
            let mut normalized = Vec::with_capacity(raw_selected.len());
            for raw in raw_selected {
                let id = raw.trim();
                if !valid_ids.contains(id) {
                    return Err(AppError::BadRequest(format!(
                        "invalid option id '{id}' for question '{question_id}'"
                    )));
                }
                if seen.insert(id.to_string()) {
                    normalized.push(id.to_string());
                }
            }

            if question_type == QuestionType::SingleChoice {
                if normalized.len() != 1 {
                    return Err(AppError::BadRequest(
                        "single-choice question requires exactly one selected option".to_string(),
                    ));
                }
                let expected_set: HashSet<&str> = expected.iter().map(String::as_str).collect();
                let is_correct = expected_set.contains(normalized[0].as_str());
                let earned = if is_correct { max_points } else { 0 };
                return Ok((AnswerValue::Choice(normalized), earned, is_correct));
            }

            let required_count = choice_spec.required_count.unwrap_or(expected.len());
            if normalized.is_empty() {
                return Err(AppError::BadRequest(
                    "at least one option must be selected".to_string(),
                ));
            }
            if normalized.len() > required_count {
                return Err(AppError::BadRequest(format!(
                    "multiple-choice question allows at most {required_count} selected options"
                )));
            }

            let expected_set: HashSet<&str> = expected.iter().map(String::as_str).collect();
            let selected_set: HashSet<&str> = normalized.iter().map(String::as_str).collect();
            let matches = selected_set.intersection(&expected_set).count();
            let earned = (matches as i16).min(max_points);
            let is_correct = expected_set == selected_set;
            Ok((AnswerValue::Choice(normalized), earned, is_correct))
        }
        QuestionType::Hotspot => {
            let AnswerValue::Mapping(raw_map) = submitted else {
                return Err(AppError::BadRequest(format!(
                    "question '{question_id}' expects an object mapping slot IDs to choice IDs"
                )));
            };
            let Some(AnswerValue::Mapping(expected)) = answer_key else {
                return Err(AppError::Config(format!(
                    "question '{question_id}' is missing a valid mapping answer_key"
                )));
            };

            let hs_spec: HotspotSpec = serde_json::from_value(spec.clone()).map_err(|err| {
                AppError::Config(format!("invalid hotspot spec on '{question_id}': {err}"))
            })?;

            let slot_choices: HashMap<&str, HashSet<&str>> = hs_spec
                .slots
                .iter()
                .map(|slot| {
                    let choices = slot.choices.iter().map(|c| c.id.as_str()).collect();
                    (slot.id.as_str(), choices)
                })
                .collect();

            let mut normalized = BTreeMap::new();
            for (raw_slot, raw_choice) in raw_map {
                let slot_id = raw_slot.trim();
                let choice_id = raw_choice.trim();
                let Some(valid_choices) = slot_choices.get(slot_id) else {
                    return Err(AppError::BadRequest(format!(
                        "invalid slot id '{slot_id}' for question '{question_id}'"
                    )));
                };
                if !valid_choices.contains(choice_id) {
                    return Err(AppError::BadRequest(format!(
                        "invalid choice id '{choice_id}' for slot '{slot_id}'"
                    )));
                }
                normalized.insert(slot_id.to_string(), choice_id.to_string());
            }

            if normalized.len() != hs_spec.slots.len() {
                return Err(AppError::BadRequest(format!(
                    "all {} hotspot slots must be answered",
                    hs_spec.slots.len()
                )));
            }

            let matches = expected
                .iter()
                .filter(|(slot, ans)| normalized.get(*slot) == Some(*ans))
                .count();
            let earned = (matches as i16).min(max_points);
            let is_correct = matches == expected.len() && normalized.len() == expected.len();
            Ok((AnswerValue::Mapping(normalized), earned, is_correct))
        }
        QuestionType::DragDrop => {
            let AnswerValue::Mapping(raw_map) = submitted else {
                return Err(AppError::BadRequest(format!(
                    "question '{question_id}' expects an object mapping target IDs to pool IDs"
                )));
            };
            let Some(AnswerValue::Mapping(expected)) = answer_key else {
                return Err(AppError::Config(format!(
                    "question '{question_id}' is missing a valid mapping answer_key"
                )));
            };

            let dd_spec: DragDropSpec = serde_json::from_value(spec.clone()).map_err(|err| {
                AppError::Config(format!("invalid drag_drop spec on '{question_id}': {err}"))
            })?;

            let valid_targets: HashSet<&str> =
                dd_spec.targets.iter().map(|t| t.id.as_str()).collect();
            let valid_pool: HashSet<&str> = dd_spec.pool.iter().map(|p| p.id.as_str()).collect();

            let mut normalized = BTreeMap::new();
            let mut used_pool = HashSet::new();

            for (raw_target, raw_pool) in raw_map {
                let target_id = raw_target.trim();
                let pool_id = raw_pool.trim();
                if !valid_targets.contains(target_id) {
                    return Err(AppError::BadRequest(format!(
                        "invalid target id '{target_id}' for question '{question_id}'"
                    )));
                }
                if !valid_pool.contains(pool_id) {
                    return Err(AppError::BadRequest(format!(
                        "invalid pool id '{pool_id}' for question '{question_id}'"
                    )));
                }
                if !dd_spec.reusable && !used_pool.insert(pool_id.to_string()) {
                    return Err(AppError::BadRequest(format!(
                        "pool item '{pool_id}' cannot be reused for question '{question_id}'"
                    )));
                }
                normalized.insert(target_id.to_string(), pool_id.to_string());
            }

            if normalized.len() != dd_spec.targets.len() {
                return Err(AppError::BadRequest(format!(
                    "all {} drag-and-drop targets must be filled",
                    dd_spec.targets.len()
                )));
            }

            let matches = expected
                .iter()
                .filter(|(target, ans)| normalized.get(*target) == Some(*ans))
                .count();
            let earned = (matches as i16).min(max_points);
            let is_correct = matches == expected.len() && normalized.len() == expected.len();
            Ok((AnswerValue::Mapping(normalized), earned, is_correct))
        }
    }
}

/// Evaluates a submitted answer, persists partial/full points and progress, and returns the graded record.
pub async fn submit_exam_answer(
    pool: &PgPool,
    exam_id: &str,
    client_id: Option<&str>,
    req: &SubmitAnswerRequest,
) -> Result<ExamAnswerRecord, AppError> {
    let question_id = req.question_id.trim();

    let row: QuestionForGradingRow = sqlx::query_as(GET_QUESTION_FOR_GRADING_SQL)
        .bind(exam_id)
        .bind(question_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "question '{question_id}' not found in exam '{exam_id}'"
            ))
        })?;

    if req.question_index != row.question_index {
        return Err(AppError::BadRequest(format!(
            "questionIndex {} does not match question '{question_id}' (expected {})",
            req.question_index, row.question_index
        )));
    }

    let (normalized_answer, earned_points, is_correct) = grade_answer(
        question_id,
        row.question_type,
        &row.spec,
        row.answer_key.as_ref().map(|k| &k.0),
        row.max_points,
        &req.submitted_answer,
    )?;

    if let Some(cid) = client_id {
        let mut tx = pool.begin().await?;

        sqlx::query(ENSURE_CLIENT_SQL)
            .bind(cid)
            .execute(&mut *tx)
            .await?;

        sqlx::query(UPSERT_CLIENT_ANSWER_SQL)
            .bind(cid)
            .bind(question_id)
            .bind(exam_id)
            .bind(row.question_index)
            .bind(Json(&normalized_answer))
            .bind(earned_points)
            .bind(row.max_points)
            .bind(is_correct)
            .execute(&mut *tx)
            .await?;

        sqlx::query(UPSERT_CLIENT_PROGRESS_ON_ANSWER_SQL)
            .bind(cid)
            .bind(exam_id)
            .bind(row.question_index)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
    }

    Ok(ExamAnswerRecord {
        question_id: question_id.to_string(),
        submitted_answer: normalized_answer,
        earned_points,
        max_points: row.max_points,
        is_correct,
    })
}

const GET_QUESTION_INDEX_SQL: &str = r#"
SELECT question_index
FROM questions
WHERE exam_id = $1 AND id = $2
"#;

const UPSERT_QUESTION_FLAG_SQL: &str = r#"
INSERT INTO client_question_flags (client_id, question_id, exam_id, question_index, flagged_at)
VALUES ($1, $2, $3, $4, NOW())
ON CONFLICT (client_id, question_id) DO NOTHING
"#;

const DELETE_QUESTION_FLAG_SQL: &str = r#"
DELETE FROM client_question_flags
WHERE client_id = $1 AND exam_id = $2 AND question_id = $3
"#;

/// Marks a question for review (`client_question_flags`), independent of whether it has been answered.
pub async fn flag_question(
    pool: &PgPool,
    exam_id: &str,
    client_id: &str,
    req: &FlagQuestionRequest,
) -> Result<FlagQuestionResponse, AppError> {
    let question_id = req.question_id.trim();

    let actual_index: i32 = sqlx::query_scalar(GET_QUESTION_INDEX_SQL)
        .bind(exam_id)
        .bind(question_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "question '{question_id}' not found in exam '{exam_id}'"
            ))
        })?;

    if req.question_index != actual_index {
        return Err(AppError::BadRequest(format!(
            "questionIndex {} does not match question '{question_id}' (expected {actual_index})",
            req.question_index
        )));
    }

    let mut tx = pool.begin().await?;
    sqlx::query(ENSURE_CLIENT_SQL)
        .bind(client_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(UPSERT_QUESTION_FLAG_SQL)
        .bind(client_id)
        .bind(question_id)
        .bind(exam_id)
        .bind(actual_index)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(FlagQuestionResponse {
        question_id: question_id.to_string(),
        is_flagged: true,
    })
}

/// Removes a question's "Mark for review" flag for the given client.
pub async fn unflag_question(
    pool: &PgPool,
    exam_id: &str,
    client_id: &str,
    question_id: &str,
) -> Result<FlagQuestionResponse, AppError> {
    let qid = question_id.trim();
    sqlx::query(DELETE_QUESTION_FLAG_SQL)
        .bind(client_id)
        .bind(exam_id)
        .bind(qid)
        .execute(pool)
        .await?;

    Ok(FlagQuestionResponse {
        question_id: qid.to_string(),
        is_flagged: false,
    })
}

/// Deletes a client's submitted answer for a single question so it returns to unanswered state.
pub async fn reset_question_answer(
    pool: &PgPool,
    exam_id: &str,
    client_id: &str,
    question_id: &str,
) -> Result<ResetQuestionResponse, AppError> {
    let qid = question_id.trim();
    sqlx::query(
        "DELETE FROM client_answers WHERE client_id = $1 AND exam_id = $2 AND question_id = $3",
    )
    .bind(client_id)
    .bind(exam_id)
    .bind(qid)
    .execute(pool)
    .await?;

    Ok(ResetQuestionResponse {
        question_id: qid.to_string(),
        reset: true,
    })
}

/// Clears all submitted answers, progress, and review flags for a client in an exam.
pub async fn reset_exam_progress(
    pool: &PgPool,
    exam_id: &str,
    client_id: &str,
) -> Result<ResetExamResponse, AppError> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM client_answers WHERE client_id = $1 AND exam_id = $2")
        .bind(client_id)
        .bind(exam_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM client_exam_progress WHERE client_id = $1 AND exam_id = $2")
        .bind(client_id)
        .bind(exam_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM client_question_flags WHERE client_id = $1 AND exam_id = $2")
        .bind(client_id)
        .bind(exam_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(ResetExamResponse {
        exam_id: exam_id.to_string(),
        reset: true,
    })
}

const LIST_QUESTION_NOTES_SQL: &str = r#"
SELECT
    n.id,
    n.question_id,
    n.parent_id,
    n.client_id,
    n.content,
    TO_CHAR(n.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at,
    TO_CHAR(n.updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS updated_at,
    COALESCE(COUNT(*) FILTER (WHERE v.vote = 1), 0)::bigint AS upvotes,
    COALESCE(COUNT(*) FILTER (WHERE v.vote = -1), 0)::bigint AS downvotes,
    COALESCE(SUM(v.vote), 0)::bigint AS score,
    COALESCE(
        MAX(CASE WHEN $3::text IS NOT NULL AND v.client_id = $3::text THEN v.vote END),
        0
    )::smallint AS my_vote
FROM question_notes n
LEFT JOIN question_note_votes v ON v.note_id = n.id
WHERE n.exam_id = $1 AND n.question_id = $2
GROUP BY n.id
ORDER BY score DESC, n.created_at DESC, n.id DESC
"#;

const GET_NOTE_VOTE_COUNTS_SQL: &str = r#"
SELECT
    COALESCE(COUNT(*) FILTER (WHERE vote = 1), 0)::bigint AS upvotes,
    COALESCE(COUNT(*) FILTER (WHERE vote = -1), 0)::bigint AS downvotes,
    COALESCE(SUM(vote), 0)::bigint AS score,
    COALESCE(
        MAX(CASE WHEN $2::text IS NOT NULL AND client_id = $2::text THEN vote END),
        0
    )::smallint AS my_vote
FROM question_note_votes
WHERE note_id = $1
"#;

const CHECK_NOTE_EXISTS_SQL: &str = r#"
SELECT EXISTS(
    SELECT 1 FROM question_notes
    WHERE id = $1 AND exam_id = $2 AND question_id = $3
)
"#;

const UPSERT_NOTE_VOTE_SQL: &str = r#"
INSERT INTO question_note_votes (note_id, client_id, vote, updated_at)
VALUES ($1, $2, $3, NOW())
ON CONFLICT (note_id, client_id) DO UPDATE SET
    vote = EXCLUDED.vote,
    updated_at = NOW()
"#;

const DELETE_NOTE_VOTE_SQL: &str = r#"
DELETE FROM question_note_votes
WHERE note_id = $1 AND client_id = $2
"#;

const CHECK_PARENT_NOTE_SQL: &str = r#"
SELECT parent_id
FROM question_notes
WHERE id = $1 AND exam_id = $2 AND question_id = $3
"#;

const INSERT_QUESTION_NOTE_SQL: &str = r#"
INSERT INTO question_notes (exam_id, question_id, parent_id, client_id, content)
VALUES ($1, $2, $3, $4, $5)
RETURNING
    id,
    question_id,
    parent_id,
    client_id,
    content,
    TO_CHAR(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at,
    TO_CHAR(updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS updated_at
"#;

const UPDATE_QUESTION_NOTE_SQL: &str = r#"
UPDATE question_notes
SET content = $1, updated_at = NOW()
WHERE id = $2 AND exam_id = $3 AND question_id = $4 AND client_id = $5
RETURNING
    id,
    question_id,
    parent_id,
    client_id,
    content,
    TO_CHAR(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at,
    TO_CHAR(updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS updated_at
"#;

const DELETE_QUESTION_NOTE_SQL: &str = r#"
DELETE FROM question_notes
WHERE id = $1 AND exam_id = $2 AND question_id = $3 AND client_id = $4
"#;

#[derive(Debug, FromRow)]
struct QuestionNoteListRow {
    id: i64,
    question_id: String,
    parent_id: Option<i64>,
    client_id: String,
    content: String,
    created_at: String,
    updated_at: String,
    upvotes: i64,
    downvotes: i64,
    score: i64,
    my_vote: i16,
}

#[derive(Debug, FromRow)]
struct QuestionNoteRow {
    id: i64,
    question_id: String,
    parent_id: Option<i64>,
    client_id: String,
    content: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, FromRow)]
struct NoteVoteCountsRow {
    upvotes: i64,
    downvotes: i64,
    score: i64,
    my_vote: i16,
}

fn validate_note_content(raw: &str) -> Result<String, AppError> {
    let trimmed = raw.trim();
    let len = trimmed.chars().count();
    if len == 0 || len > 2000 {
        return Err(AppError::BadRequest(
            "note content must be between 1 and 2000 characters".to_string(),
        ));
    }
    Ok(trimmed.to_string())
}

/// Lists all users' notes for a given question sorted by vote score, marking notes owned by `current_client_id`.
pub async fn list_question_notes(
    pool: &PgPool,
    exam_id: &str,
    question_id: &str,
    current_client_id: Option<&str>,
) -> Result<Vec<QuestionNote>, AppError> {
    let qid = question_id.trim();
    let rows: Vec<QuestionNoteListRow> = sqlx::query_as(LIST_QUESTION_NOTES_SQL)
        .bind(exam_id)
        .bind(qid)
        .bind(current_client_id)
        .fetch_all(pool)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| QuestionNote {
            is_owner: current_client_id.is_some_and(|cid| cid == row.client_id),
            id: row.id,
            question_id: row.question_id,
            parent_id: row.parent_id,
            client_id: row.client_id,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
            upvotes: row.upvotes,
            downvotes: row.downvotes,
            score: row.score,
            my_vote: row.my_vote,
        })
        .collect())
}

/// Creates a new note (or 1-level reply when `parent_id` is provided) on a question for `client_id`.
pub async fn create_question_note(
    pool: &PgPool,
    exam_id: &str,
    question_id: &str,
    client_id: &str,
    raw_content: &str,
    parent_id: Option<i64>,
) -> Result<QuestionNote, AppError> {
    let qid = question_id.trim();
    let content = validate_note_content(raw_content)?;

    let exists: Option<i32> = sqlx::query_scalar(GET_QUESTION_INDEX_SQL)
        .bind(exam_id)
        .bind(qid)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        return Err(AppError::NotFound(format!(
            "question '{qid}' not found in exam '{exam_id}'"
        )));
    }

    if let Some(pid) = parent_id {
        let parent_parent: Option<Option<i64>> = sqlx::query_scalar(CHECK_PARENT_NOTE_SQL)
            .bind(pid)
            .bind(exam_id)
            .bind(qid)
            .fetch_optional(pool)
            .await?;

        match parent_parent {
            None => {
                return Err(AppError::NotFound(format!(
                    "parent note '{pid}' not found for question '{qid}'"
                )));
            }
            Some(Some(_)) => {
                return Err(AppError::BadRequest(
                    "only 1 level of replies is allowed".to_string(),
                ));
            }
            Some(None) => {}
        }
    }

    let mut tx = pool.begin().await?;
    sqlx::query(ENSURE_CLIENT_SQL)
        .bind(client_id)
        .execute(&mut *tx)
        .await?;

    let row: QuestionNoteRow = sqlx::query_as(INSERT_QUESTION_NOTE_SQL)
        .bind(exam_id)
        .bind(qid)
        .bind(parent_id)
        .bind(client_id)
        .bind(&content)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(QuestionNote {
        id: row.id,
        question_id: row.question_id,
        parent_id: row.parent_id,
        client_id: row.client_id,
        content: row.content,
        created_at: row.created_at,
        updated_at: row.updated_at,
        is_owner: true,
        upvotes: 0,
        downvotes: 0,
        score: 0,
        my_vote: 0,
    })
}

/// Updates an existing note owned by `client_id`.
pub async fn update_question_note(
    pool: &PgPool,
    exam_id: &str,
    question_id: &str,
    note_id: i64,
    client_id: &str,
    raw_content: &str,
) -> Result<QuestionNote, AppError> {
    let qid = question_id.trim();
    let content = validate_note_content(raw_content)?;

    let row: QuestionNoteRow = sqlx::query_as(UPDATE_QUESTION_NOTE_SQL)
        .bind(&content)
        .bind(note_id)
        .bind(exam_id)
        .bind(qid)
        .bind(client_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "note '{note_id}' not found or not owned by client"
            ))
        })?;

    let counts: NoteVoteCountsRow = sqlx::query_as(GET_NOTE_VOTE_COUNTS_SQL)
        .bind(row.id)
        .bind(Some(client_id))
        .fetch_one(pool)
        .await?;

    Ok(QuestionNote {
        id: row.id,
        question_id: row.question_id,
        parent_id: row.parent_id,
        client_id: row.client_id,
        content: row.content,
        created_at: row.created_at,
        updated_at: row.updated_at,
        is_owner: true,
        upvotes: counts.upvotes,
        downvotes: counts.downvotes,
        score: counts.score,
        my_vote: counts.my_vote,
    })
}

/// Sets or clears a client's upvote (`1`), downvote (`-1`), or unvote (`0`) on a question note.
pub async fn vote_question_note(
    pool: &PgPool,
    exam_id: &str,
    question_id: &str,
    note_id: i64,
    client_id: &str,
    vote: i16,
) -> Result<VoteNoteResponse, AppError> {
    if !(-1..=1).contains(&vote) {
        return Err(AppError::BadRequest(
            "vote must be -1, 0, or 1".to_string(),
        ));
    }

    let qid = question_id.trim();
    let exists: bool = sqlx::query_scalar(CHECK_NOTE_EXISTS_SQL)
        .bind(note_id)
        .bind(exam_id)
        .bind(qid)
        .fetch_one(pool)
        .await?;
    if !exists {
        return Err(AppError::NotFound(format!(
            "note '{note_id}' not found for question '{qid}'"
        )));
    }

    let mut tx = pool.begin().await?;
    sqlx::query(ENSURE_CLIENT_SQL)
        .bind(client_id)
        .execute(&mut *tx)
        .await?;

    if vote == 0 {
        sqlx::query(DELETE_NOTE_VOTE_SQL)
            .bind(note_id)
            .bind(client_id)
            .execute(&mut *tx)
            .await?;
    } else {
        sqlx::query(UPSERT_NOTE_VOTE_SQL)
            .bind(note_id)
            .bind(client_id)
            .bind(vote)
            .execute(&mut *tx)
            .await?;
    }

    let counts: NoteVoteCountsRow = sqlx::query_as(GET_NOTE_VOTE_COUNTS_SQL)
        .bind(note_id)
        .bind(Some(client_id))
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(VoteNoteResponse {
        note_id,
        upvotes: counts.upvotes,
        downvotes: counts.downvotes,
        score: counts.score,
        my_vote: counts.my_vote,
    })
}

/// Deletes an existing note owned by `client_id`.
pub async fn delete_question_note(
    pool: &PgPool,
    exam_id: &str,
    question_id: &str,
    note_id: i64,
    client_id: &str,
) -> Result<DeleteQuestionNoteResponse, AppError> {
    let qid = question_id.trim();
    let result = sqlx::query(DELETE_QUESTION_NOTE_SQL)
        .bind(note_id)
        .bind(exam_id)
        .bind(qid)
        .bind(client_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "note '{note_id}' not found or not owned by client"
        )));
    }

    Ok(DeleteQuestionNoteResponse {
        id: note_id,
        deleted: true,
    })
}

const ENSURE_CLIENT_SQL: &str = r#"
INSERT INTO clients (id)
VALUES ($1)
ON CONFLICT (id) DO NOTHING
"#;

const REGISTER_CLIENT_IF_NEW_SQL: &str = r#"
INSERT INTO clients (id)
VALUES ($1)
ON CONFLICT (id) DO NOTHING
RETURNING id
"#;

const REGISTER_GENERATED_CLIENT_SQL: &str = r#"
INSERT INTO clients (id)
VALUES (gen_random_uuid()::text)
ON CONFLICT (id) DO NOTHING
RETURNING id
"#;

/// Ensures an existing client ID is registered in `clients` without overwriting it.
pub async fn ensure_client_id(pool: &PgPool, client_id: &str) -> Result<ClientInfoResponse, AppError> {
    sqlx::query(ENSURE_CLIENT_SQL)
        .bind(client_id)
        .execute(pool)
        .await?;

    Ok(ClientInfoResponse {
        client_id: client_id.to_string(),
    })
}

/// Registers a brand-new unique client ID, generating a fresh UUID v4 in Postgres if `candidate` exists.
pub async fn register_unique_client(
    pool: &PgPool,
    candidate: Option<&str>,
) -> Result<ClientInfoResponse, AppError> {
    if let Some(id) = candidate
        && let Some(inserted) = sqlx::query_scalar::<_, String>(REGISTER_CLIENT_IF_NEW_SQL)
            .bind(id)
            .fetch_optional(pool)
            .await?
    {
        return Ok(ClientInfoResponse {
            client_id: inserted,
        });
    }

    for _ in 0..8 {
        if let Some(inserted) = sqlx::query_scalar::<_, String>(REGISTER_GENERATED_CLIENT_SQL)
            .fetch_optional(pool)
            .await?
        {
            return Ok(ClientInfoResponse {
                client_id: inserted,
            });
        }
    }

    Err(AppError::Config(
        "failed to allocate a unique client ID".to_string(),
    ))
}

/// Renames a client ID to `new_client_id` only if `new_client_id` does not already exist.
pub async fn rename_client_id(
    pool: &PgPool,
    old_client_id: Option<&str>,
    new_client_id: &str,
) -> Result<ClientInfoResponse, AppError> {
    let mut tx = pool.begin().await?;

    if old_client_id == Some(new_client_id) {
        sqlx::query(ENSURE_CLIENT_SQL)
            .bind(new_client_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        return Ok(ClientInfoResponse {
            client_id: new_client_id.to_string(),
        });
    }

    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM clients WHERE id = $1)")
        .bind(new_client_id)
        .fetch_one(&mut *tx)
        .await?;

    if exists {
        return Err(AppError::Conflict(
            "client ID already exists".to_string(),
        ));
    }

    let updated = if let Some(old_id) = old_client_id {
        let result = sqlx::query("UPDATE clients SET id = $1, updated_at = NOW() WHERE id = $2")
            .bind(new_client_id)
            .bind(old_id)
            .execute(&mut *tx)
            .await
            .map_err(map_unique_conflict)?;
        result.rows_affected() > 0
    } else {
        false
    };

    if !updated {
        sqlx::query("INSERT INTO clients (id) VALUES ($1)")
            .bind(new_client_id)
            .execute(&mut *tx)
            .await
            .map_err(map_unique_conflict)?;
    }

    tx.commit().await?;

    Ok(ClientInfoResponse {
        client_id: new_client_id.to_string(),
    })
}

fn map_unique_conflict(err: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(db_err) = &err
        && db_err.code().as_deref() == Some("23505")
    {
        return AppError::Conflict("client ID already exists".to_string());
    }
    AppError::Database(err)
}

/// Switches to an existing `target_client_id` and deletes the current client's data via cascade.
pub async fn sync_client_id(
    pool: &PgPool,
    current_client_id: Option<&str>,
    target_client_id: &str,
) -> Result<ClientInfoResponse, AppError> {
    let mut tx = pool.begin().await?;

    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM clients WHERE id = $1)")
        .bind(target_client_id)
        .fetch_one(&mut *tx)
        .await?;

    if !exists {
        return Err(AppError::NotFound(
            "target client ID not found".to_string(),
        ));
    }

    if let Some(current_id) = current_client_id
        && current_id != target_client_id
    {
        sqlx::query("UPDATE question_notes SET client_id = $2 WHERE client_id = $1")
            .bind(current_id)
            .bind(target_client_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "INSERT INTO question_note_votes (note_id, client_id, vote, updated_at)
             SELECT note_id, $2, vote, updated_at
             FROM question_note_votes
             WHERE client_id = $1
             ON CONFLICT (note_id, client_id) DO NOTHING",
        )
        .bind(current_id)
        .bind(target_client_id)
        .execute(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM clients WHERE id = $1")
            .bind(current_id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok(ClientInfoResponse {
        client_id: target_client_id.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn grades_single_choice() {
        let spec = json!({
            "options": [
                { "id": "a", "label": "A", "text": "Yes" },
                { "id": "b", "label": "B", "text": "No" }
            ]
        });
        let key = AnswerValue::Choice(vec!["b".to_string()]);

        let (_, pts, ok) = grade_answer(
            "q1",
            QuestionType::SingleChoice,
            &spec,
            Some(&key),
            1,
            &AnswerValue::Choice(vec!["b".to_string()]),
        )
        .expect("valid single choice");
        assert_eq!((pts, ok), (1, true));

        let (_, pts, ok) = grade_answer(
            "q1",
            QuestionType::SingleChoice,
            &spec,
            Some(&key),
            1,
            &AnswerValue::Choice(vec!["a".to_string()]),
        )
        .expect("wrong single choice");
        assert_eq!((pts, ok), (0, false));
    }

    #[test]
    fn grades_multiple_choice_with_partial_credit() {
        let spec = json!({
            "options": [
                { "id": "a", "label": "A", "text": "A" },
                { "id": "b", "label": "B", "text": "B" },
                { "id": "c", "label": "C", "text": "C" },
                { "id": "d", "label": "D", "text": "D" }
            ],
            "requiredCount": 2
        });
        let key = AnswerValue::Choice(vec!["b".to_string(), "d".to_string()]);

        let (_, pts, ok) = grade_answer(
            "q2",
            QuestionType::MultipleChoice,
            &spec,
            Some(&key),
            2,
            &AnswerValue::Choice(vec!["a".to_string(), "b".to_string()]),
        )
        .expect("partial credit");
        assert_eq!((pts, ok), (1, false));

        let (_, pts, ok) = grade_answer(
            "q2",
            QuestionType::MultipleChoice,
            &spec,
            Some(&key),
            2,
            &AnswerValue::Choice(vec!["d".to_string(), "b".to_string()]),
        )
        .expect("full credit");
        assert_eq!((pts, ok), (2, true));
    }

    #[test]
    fn grades_hotspot_and_rejects_invalid_choice() {
        let spec = json!({
            "mode": "dropdown",
            "slots": [
                { "id": "s1", "label": "Slot 1", "choices": [{ "id": "c1", "text": "W1" }, { "id": "c2", "text": "W2" }] },
                { "id": "s2", "label": "Slot 2", "choices": [{ "id": "c1", "text": "PO" }, { "id": "c2", "text": "TO" }] }
            ]
        });
        let key = AnswerValue::Mapping(BTreeMap::from([
            ("s1".to_string(), "c1".to_string()),
            ("s2".to_string(), "c2".to_string()),
        ]));

        let (_, pts, ok) = grade_answer(
            "q3",
            QuestionType::Hotspot,
            &spec,
            Some(&key),
            2,
            &AnswerValue::Mapping(BTreeMap::from([
                ("s1".to_string(), "c1".to_string()),
                ("s2".to_string(), "c1".to_string()),
            ])),
        )
        .expect("partial hotspot");
        assert_eq!((pts, ok), (1, false));

        let err = grade_answer(
            "q3",
            QuestionType::Hotspot,
            &spec,
            Some(&key),
            2,
            &AnswerValue::Mapping(BTreeMap::from([
                ("s1".to_string(), "bad".to_string()),
                ("s2".to_string(), "c2".to_string()),
            ])),
        )
        .expect_err("invalid choice");
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[test]
    fn grades_drag_drop_and_enforces_non_reusable() {
        let spec = json!({
            "mode": "sequence",
            "reusable": false,
            "pool": [
                { "id": "p1", "text": "Step 1" },
                { "id": "p2", "text": "Step 2" }
            ],
            "targets": [
                { "id": "t1", "label": "1" },
                { "id": "t2", "label": "2" }
            ]
        });
        let key = AnswerValue::Mapping(BTreeMap::from([
            ("t1".to_string(), "p1".to_string()),
            ("t2".to_string(), "p2".to_string()),
        ]));

        let err = grade_answer(
            "q4",
            QuestionType::DragDrop,
            &spec,
            Some(&key),
            2,
            &AnswerValue::Mapping(BTreeMap::from([
                ("t1".to_string(), "p1".to_string()),
                ("t2".to_string(), "p1".to_string()),
            ])),
        )
        .expect_err("duplicate pool item");
        assert!(matches!(err, AppError::BadRequest(_)));

        let (_, pts, ok) = grade_answer(
            "q4",
            QuestionType::DragDrop,
            &spec,
            Some(&key),
            2,
            &AnswerValue::Mapping(BTreeMap::from([
                ("t1".to_string(), "p1".to_string()),
                ("t2".to_string(), "p2".to_string()),
            ])),
        )
        .expect("valid drag drop");
        assert_eq!((pts, ok), (2, true));
    }

    #[test]
    fn rejects_simulation_submission() {
        let err = grade_answer(
            "q5",
            QuestionType::Simulation,
            &json!({}),
            None,
            0,
            &AnswerValue::Choice(vec!["a".to_string()]),
        )
        .expect_err("simulation rejected");
        assert!(matches!(err, AppError::BadRequest(_)));
    }
}
