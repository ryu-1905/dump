use std::collections::HashSet;

use sqlx::{FromRow, PgPool, types::Json, types::Uuid};

use crate::{
    error::AppError,
    models::exam::{
        ExamAnswerRecord, ExamDetail, ExamQuestion, ExamQuestionPayload, ExamSummary, ExamTopic,
        NormalizedExamListParams, PaginatedExamsResponse, QuestionImage, QuestionNavItem,
        QuestionNavStatus, QuestionOption, SubmitAnswerRequest,
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
    WHERE $1::uuid IS NOT NULL AND client_id = $1::uuid
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
        ON cep.exam_id = e.id AND $1::uuid IS NOT NULL AND cep.client_id = $1::uuid
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
    client_id: Option<Uuid>,
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
        WHERE $2::uuid IS NOT NULL
          AND ca.client_id = $2::uuid
          AND ca.exam_id = e.id
    ) AS answered_count,
    cep.last_question_index,
    TO_CHAR(e.updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD') AS updated_at,
    e.description,
    e.description_vi
FROM exams e
LEFT JOIN client_exam_progress cep
    ON cep.exam_id = e.id
   AND $2::uuid IS NOT NULL
   AND cep.client_id = $2::uuid
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
   AND $2::uuid IS NOT NULL
   AND ca.client_id = $2::uuid
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
    client_id: Option<Uuid>,
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
    ca.selected_option_ids,
    ca.is_correct
FROM questions q
LEFT JOIN client_answers ca
    ON ca.question_id = q.id
   AND $2::uuid IS NOT NULL
   AND ca.client_id = $2::uuid
WHERE q.exam_id = $1
  AND ($3::text IS NULL OR q.topic_id = $3::text)
ORDER BY q.question_index ASC
"#;

const GET_QUESTION_DETAIL_SQL: &str = r#"
SELECT
    id,
    topic_id,
    question_index,
    prompt,
    prompt_vi,
    multiple_choice,
    show_answer_by_default,
    options,
    correct_option_ids,
    explanation,
    explanation_vi,
    keywords,
    keywords_vi,
    images
FROM questions
WHERE id = $1
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
    selected_option_ids: Option<Vec<String>>,
    is_correct: Option<bool>,
}

#[derive(Debug, FromRow)]
struct QuestionDetailRow {
    id: String,
    topic_id: String,
    question_index: i32,
    prompt: String,
    prompt_vi: Option<String>,
    multiple_choice: bool,
    show_answer_by_default: bool,
    options: Json<Vec<QuestionOption>>,
    correct_option_ids: Vec<String>,
    explanation: String,
    explanation_vi: Option<String>,
    keywords: Vec<String>,
    keywords_vi: Vec<String>,
    images: Json<Vec<QuestionImage>>,
}

/// Loads one practice question (optionally scoped to a topic) and records `last_question_index`.
pub async fn get_exam_question(
    pool: &PgPool,
    exam_id: &str,
    client_id: Option<Uuid>,
    topic_id: Option<&str>,
    requested_q: Option<i32>,
) -> Result<ExamQuestionPayload, AppError> {
    let mut exam = get_exam_overview(pool, exam_id, client_id).await?;

    let scoped: Vec<ScopedQuestionRow> = sqlx::query_as(GET_SCOPED_QUESTIONS_SQL)
        .bind(exam_id)
        .bind(client_id)
        .bind(topic_id)
        .fetch_all(pool)
        .await?;

    if scoped.is_empty() {
        let msg = match topic_id {
            Some(topic) => format!("no questions found for topic '{topic}' in exam '{exam_id}'"),
            None => format!("no questions found for exam '{exam_id}'"),
        };
        return Err(AppError::NotFound(msg));
    }

    let selected_pos = if let Some(q) = requested_q {
        scoped.iter().position(|row| row.question_index == q).unwrap_or(0)
    } else if topic_id.is_none()
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

    let answer = match (&selected.selected_option_ids, selected.is_correct) {
        (Some(selected_option_ids), Some(is_correct)) => Some(ExamAnswerRecord {
            question_id: selected.id.clone(),
            selected_option_ids: selected_option_ids.clone(),
            is_correct,
        }),
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
        })
        .collect();

    let detail: QuestionDetailRow = sqlx::query_as(GET_QUESTION_DETAIL_SQL)
        .bind(&selected.id)
        .fetch_one(pool)
        .await?;

    let question = ExamQuestion {
        id: detail.id,
        topic_id: detail.topic_id,
        index: detail.question_index,
        prompt: detail.prompt,
        prompt_vi: detail.prompt_vi,
        multiple_choice: detail.multiple_choice,
        show_answer_by_default: detail.show_answer_by_default,
        options: detail.options.0,
        correct_option_ids: detail.correct_option_ids,
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
        answer,
        nav_items,
        answered_count,
    })
}

const GET_QUESTION_FOR_GRADING_SQL: &str = r#"
SELECT
    question_index,
    multiple_choice,
    show_answer_by_default,
    options,
    correct_option_ids
FROM questions
WHERE exam_id = $1 AND id = $2
"#;

const UPSERT_CLIENT_ANSWER_SQL: &str = r#"
INSERT INTO client_answers (
    client_id, question_id, exam_id, question_index,
    selected_option_ids, is_correct, answered_at
)
VALUES ($1, $2, $3, $4, $5, $6, NOW())
ON CONFLICT (client_id, question_id) DO UPDATE SET
    question_index = EXCLUDED.question_index,
    selected_option_ids = EXCLUDED.selected_option_ids,
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
    multiple_choice: bool,
    show_answer_by_default: bool,
    options: Json<Vec<QuestionOption>>,
    correct_option_ids: Vec<String>,
}

/// Evaluates selected option(s), persists the answer and progress, and returns the graded record.
pub async fn submit_exam_answer(
    pool: &PgPool,
    exam_id: &str,
    client_id: Option<Uuid>,
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

    if row.show_answer_by_default && row.options.0.is_empty() {
        return Err(AppError::BadRequest(format!(
            "question '{question_id}' is a simulation without selectable options"
        )));
    }

    let valid_option_ids: HashSet<&str> = row.options.0.iter().map(|opt| opt.id.as_str()).collect();
    let mut seen = HashSet::with_capacity(req.selected_option_ids.len());
    let mut normalized_selected = Vec::with_capacity(req.selected_option_ids.len());

    for raw_id in &req.selected_option_ids {
        let opt_id = raw_id.trim();
        if !valid_option_ids.contains(opt_id) {
            return Err(AppError::BadRequest(format!(
                "invalid option id '{opt_id}' for question '{question_id}'"
            )));
        }
        if seen.insert(opt_id.to_string()) {
            normalized_selected.push(opt_id.to_string());
        }
    }

    if !row.multiple_choice && normalized_selected.len() != 1 {
        return Err(AppError::BadRequest(
            "single-choice question requires exactly one selected option".to_string(),
        ));
    }
    if row.multiple_choice && normalized_selected.is_empty() {
        return Err(AppError::BadRequest(
            "at least one option must be selected".to_string(),
        ));
    }

    let expected_set: HashSet<&str> = row.correct_option_ids.iter().map(String::as_str).collect();
    let selected_set: HashSet<&str> = normalized_selected.iter().map(String::as_str).collect();
    let is_correct = expected_set == selected_set;

    if let Some(cid) = client_id {
        let mut tx = pool.begin().await?;

        sqlx::query(UPSERT_CLIENT_ANSWER_SQL)
            .bind(cid)
            .bind(question_id)
            .bind(exam_id)
            .bind(row.question_index)
            .bind(&normalized_selected)
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
        selected_option_ids: normalized_selected,
        is_correct,
    })
}
