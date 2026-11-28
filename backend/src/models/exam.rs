use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::error::AppError;

/// Filter status for exam listing, matching `ExamProgressStatus` in the frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExamStatusFilter {
    All,
    NotStarted,
    InProgress,
    Completed,
}

impl ExamStatusFilter {
    /// Parses the optional `status` query parameter.
    pub fn parse(raw: Option<&str>) -> Result<Self, AppError> {
        match raw.map(str::trim).filter(|s| !s.is_empty()) {
            None | Some("all") => Ok(Self::All),
            Some("not_started") => Ok(Self::NotStarted),
            Some("in_progress") => Ok(Self::InProgress),
            Some("completed") => Ok(Self::Completed),
            Some(other) => Err(AppError::BadRequest(format!(
                "invalid status filter '{other}', expected one of: all, not_started, in_progress, completed"
            ))),
        }
    }

    /// Returns the string tag used inside SQL status filtering.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::NotStarted => "not_started",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
        }
    }
}

/// Raw query parameters accepted by `GET /api/exams`.
#[derive(Debug, Deserialize)]
pub struct ExamListQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub q: Option<String>,
    pub status: Option<String>,
}

/// Validated parameters passed to the exam repository.
#[derive(Debug, Clone)]
pub struct NormalizedExamListParams {
    pub page: i64,
    pub limit: i64,
    pub search: Option<String>,
    pub status: ExamStatusFilter,
}

impl TryFrom<ExamListQuery> for NormalizedExamListParams {
    type Error = AppError;

    fn try_from(query: ExamListQuery) -> Result<Self, Self::Error> {
        let page = match query.page {
            Some(p) if p < 1 => {
                return Err(AppError::BadRequest(
                    "query parameter 'page' must be >= 1".to_string(),
                ));
            }
            Some(p) => p,
            None => 1,
        };

        let limit = match query.limit {
            Some(l) if l < 1 => {
                return Err(AppError::BadRequest(
                    "query parameter 'limit' must be >= 1".to_string(),
                ));
            }
            Some(l) => l.min(100),
            None => 6,
        };

        let search = query
            .q
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToOwned::to_owned);

        let status = ExamStatusFilter::parse(query.status.as_deref())?;

        Ok(Self {
            page,
            limit,
            search,
            status,
        })
    }
}

/// Summary of an exam with client progress, returned in `GET /api/exams`.
#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ExamSummary {
    pub id: String,
    pub code: String,
    pub title: String,
    pub vendor: String,
    pub total_questions: i64,
    pub answered_count: i64,
    pub last_question_index: Option<i32>,
    pub updated_at: String,
}

/// Paginated response envelope for `GET /api/exams`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedExamsResponse {
    pub items: Vec<ExamSummary>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

/// Topic metadata and per-topic progress inside `ExamDetail`.
#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ExamTopic {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_vi: Option<String>,
    pub question_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answered_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_question_index: Option<i32>,
    pub continue_question_index: Option<i32>,
}

/// Full exam overview returned by `GET /api/exams/:id`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExamDetail {
    #[serde(flatten)]
    pub summary: ExamSummary,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_vi: Option<String>,
    pub topics: Vec<ExamTopic>,
}

/// Query parameters accepted by `GET /api/exams/{id}/questions`.
#[derive(Debug, Deserialize)]
pub struct ExamQuestionQuery {
    pub q: Option<i32>,
    pub topic: Option<String>,
}

/// Single option inside a question (`questions.options` JSONB array).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionOption {
    pub id: String,
    pub label: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_vi: Option<String>,
}

/// Inline image metadata referenced via `[[img:<id>]]` placeholders (`questions.images` JSONB array).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionImage {
    pub id: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

/// Full question payload returned inside `ExamQuestionPayload`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExamQuestion {
    pub id: String,
    pub topic_id: String,
    pub index: i32,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_vi: Option<String>,
    pub multiple_choice: bool,
    pub show_answer_by_default: bool,
    pub options: Vec<QuestionOption>,
    pub correct_option_ids: Vec<String>,
    pub explanation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_vi: Option<String>,
    pub keywords: Vec<String>,
    pub keywords_vi: Vec<String>,
    pub images: Vec<QuestionImage>,
}

/// Client's submitted answer record for a single question.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExamAnswerRecord {
    pub question_id: String,
    pub selected_option_ids: Vec<String>,
    pub is_correct: bool,
}

/// Navigation status for a question slot in the practice navigator.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionNavStatus {
    Unanswered,
    Correct,
    Incorrect,
}

/// Single navigator item in `ExamQuestionPayload`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionNavItem {
    pub index: i32,
    pub status: QuestionNavStatus,
}

/// Response payload for `GET /api/exams/{id}/questions`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExamQuestionPayload {
    pub exam: ExamDetail,
    pub total_in_scope: i64,
    pub current_index: i32,
    pub question: ExamQuestion,
    pub answer: Option<ExamAnswerRecord>,
    pub nav_items: Vec<QuestionNavItem>,
    pub answered_count: i64,
}

/// Request body accepted by `POST /api/exams/{id}/answers`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitAnswerRequest {
    pub question_id: String,
    pub question_index: i32,
    pub selected_option_ids: Vec<String>,
}

/// Response returned by client registration/ensure/rename endpoints.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfoResponse {
    pub client_id: String,
}

/// Request body accepted by `PUT /api/clients/id`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameClientRequest {
    pub new_client_id: String,
}

/// Request body accepted by `POST /api/clients/sync`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncClientRequest {
    pub target_client_id: String,
}
