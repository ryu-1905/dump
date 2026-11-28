use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
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

/// Filter for scoped practice questions (`GET /api/exams/{id}/questions?filter=...`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestionFilter {
    All,
    Wrong,
    Flagged,
}

impl QuestionFilter {
    /// Parses the optional `filter` query parameter.
    pub fn parse(raw: Option<&str>) -> Result<Self, AppError> {
        match raw.map(str::trim).filter(|s| !s.is_empty()) {
            None | Some("all") => Ok(Self::All),
            Some("wrong") => Ok(Self::Wrong),
            Some("flagged") => Ok(Self::Flagged),
            Some(other) => Err(AppError::BadRequest(format!(
                "invalid question filter '{other}', expected one of: all, wrong, flagged"
            ))),
        }
    }

    /// Returns the string tag used inside SQL question filtering.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Wrong => "wrong",
            Self::Flagged => "flagged",
        }
    }
}

/// Native PostgreSQL `question_type` enum for the 5 Microsoft-exam question formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "question_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
    Hotspot,
    DragDrop,
    Simulation,
}

/// Polymorphic answer value stored in `questions.answer_key` and `client_answers.submitted_answer`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerValue {
    Choice(Vec<String>),
    Mapping(BTreeMap<String, String>),
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
    pub case_study: Option<String>,
    pub filter: Option<String>,
}

/// Single option inside a `single_choice` / `multiple_choice` `spec.options` array.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionOption {
    pub id: String,
    pub label: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_vi: Option<String>,
}

/// Typed `spec` for `single_choice` and `multiple_choice` questions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceSpec {
    pub options: Vec<QuestionOption>,
    #[serde(default)]
    pub required_count: Option<usize>,
}

/// Single choice inside a `hotspot` slot.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotspotSlotChoice {
    pub id: String,
    #[allow(dead_code)]
    pub text: String,
    #[allow(dead_code)]
    #[serde(default)]
    pub text_vi: Option<String>,
}

/// Single dropdown or Yes/No row inside a `hotspot` `spec.slots` array.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotspotSlot {
    pub id: String,
    #[allow(dead_code)]
    pub label: String,
    #[allow(dead_code)]
    #[serde(default)]
    pub label_vi: Option<String>,
    pub choices: Vec<HotspotSlotChoice>,
}

/// Typed `spec` for `hotspot` questions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotspotSpec {
    #[allow(dead_code)]
    pub mode: String,
    pub slots: Vec<HotspotSlot>,
}

/// Draggable item inside `drag_drop` `spec.pool`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragDropItem {
    pub id: String,
    #[allow(dead_code)]
    pub text: String,
    #[allow(dead_code)]
    #[serde(default)]
    pub text_vi: Option<String>,
}

/// Drop target slot inside `drag_drop` `spec.targets`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragDropTarget {
    pub id: String,
    #[allow(dead_code)]
    pub label: String,
    #[allow(dead_code)]
    #[serde(default)]
    pub label_vi: Option<String>,
}

/// Typed `spec` for `drag_drop` questions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragDropSpec {
    #[allow(dead_code)]
    pub mode: String,
    #[serde(default)]
    pub reusable: bool,
    pub pool: Vec<DragDropItem>,
    pub targets: Vec<DragDropTarget>,
}

/// Inline image metadata referenced via `[[img:<id>]]` placeholders (`images` JSONB array).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionImage {
    pub id: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

/// Single section tab inside a Microsoft-style `CaseStudy`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseStudySection {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_vi: Option<String>,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_vi: Option<String>,
}

/// Shared scenario context attached to case-study questions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseStudy {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_vi: Option<String>,
    pub sections: Vec<CaseStudySection>,
    pub images: Vec<QuestionImage>,
}

/// Full question payload returned inside `ExamQuestionPayload`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExamQuestion {
    pub id: String,
    pub topic_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_study_id: Option<String>,
    pub index: i32,
    pub question_type: QuestionType,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_vi: Option<String>,
    pub spec: Value,
    pub answer_key: Option<AnswerValue>,
    pub max_points: i16,
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
    pub submitted_answer: AnswerValue,
    pub earned_points: i16,
    pub max_points: i16,
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
    pub is_flagged: bool,
}

/// Response payload for `GET /api/exams/{id}/questions`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExamQuestionPayload {
    pub exam: ExamDetail,
    pub total_in_scope: i64,
    pub current_index: i32,
    pub question: ExamQuestion,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_study: Option<CaseStudy>,
    pub answer: Option<ExamAnswerRecord>,
    pub is_flagged: bool,
    pub nav_items: Vec<QuestionNavItem>,
    pub answered_count: i64,
    pub flagged_count: i64,
}

/// Request body accepted by `POST /api/exams/{id}/answers`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitAnswerRequest {
    pub question_id: String,
    pub question_index: i32,
    pub submitted_answer: AnswerValue,
}

/// Request body accepted by `PUT /api/exams/{id}/flags`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagQuestionRequest {
    pub question_id: String,
    pub question_index: i32,
}

/// Response returned by `PUT /api/exams/{id}/flags` and `DELETE /api/exams/{id}/flags/{questionId}`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagQuestionResponse {
    pub question_id: String,
    pub is_flagged: bool,
}

/// Response returned by `DELETE /api/exams/{id}/answers/{questionId}`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetQuestionResponse {
    pub question_id: String,
    pub reset: bool,
}

/// Response returned by `DELETE /api/exams/{id}/answers`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetExamResponse {
    pub exam_id: String,
    pub reset: bool,
}

/// Community note attached to a question (`question_notes`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionNote {
    pub id: i64,
    pub question_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    pub client_id: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_owner: bool,
    pub upvotes: i64,
    pub downvotes: i64,
    pub score: i64,
    pub my_vote: i16,
}

/// Request body accepted by `POST /api/exams/{id}/questions/{questionId}/notes`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestionNoteRequest {
    pub content: String,
    #[serde(default)]
    pub parent_id: Option<i64>,
}

/// Request body accepted by `PUT /api/exams/{id}/questions/{questionId}/notes/{noteId}`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQuestionNoteRequest {
    pub content: String,
}

/// Response returned by `DELETE /api/exams/{id}/questions/{questionId}/notes/{noteId}`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteQuestionNoteResponse {
    pub id: i64,
    pub deleted: bool,
}

/// Request body accepted by `PUT /api/exams/{id}/questions/{questionId}/notes/{noteId}/vote`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteNoteRequest {
    pub vote: i16,
}

/// Response returned by `PUT /api/exams/{id}/questions/{questionId}/notes/{noteId}/vote`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteNoteResponse {
    pub note_id: i64,
    pub upvotes: i64,
    pub downvotes: i64,
    pub score: i64,
    pub my_vote: i16,
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
