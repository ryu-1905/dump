use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::HeaderMap,
    routing::{get, post},
};
use sqlx::types::Uuid;

use crate::{
    error::AppError,
    models::exam::{
        ExamAnswerRecord, ExamDetail, ExamListQuery, ExamQuestionPayload, ExamQuestionQuery,
        NormalizedExamListParams, PaginatedExamsResponse, SubmitAnswerRequest,
    },
    repositories::exam_repo,
    state::AppState,
};

const CLIENT_ID_HEADER: &str = "x-client-id";

/// Parses optional `X-Client-Id` UUID header; rejects malformed non-empty headers with 400.
fn parse_optional_client_id(headers: &HeaderMap) -> Result<Option<Uuid>, AppError> {
    let Some(value) = headers.get(CLIENT_ID_HEADER) else {
        return Ok(None);
    };

    let raw = value
        .to_str()
        .map_err(|_| AppError::BadRequest("invalid X-Client-Id header encoding".to_string()))?
        .trim();

    if raw.is_empty() {
        return Ok(None);
    }

    let uuid = Uuid::parse_str(raw)
        .map_err(|_| AppError::BadRequest("X-Client-Id must be a valid UUID".to_string()))?;

    Ok(Some(uuid))
}

/// `GET /api/exams` — returns paginated exam summaries with optional client progress.
async fn list_exams_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ExamListQuery>,
) -> Result<Json<PaginatedExamsResponse>, AppError> {
    let client_id = parse_optional_client_id(&headers)?;
    let params = NormalizedExamListParams::try_from(query)?;
    let response = exam_repo::list_exams(&state.pool, client_id, &params).await?;
    Ok(Json(response))
}

/// `GET /api/exams/{id}` — returns full exam overview and per-topic progress.
async fn get_exam_overview_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(exam_id): Path<String>,
) -> Result<Json<ExamDetail>, AppError> {
    let client_id = parse_optional_client_id(&headers)?;
    let detail = exam_repo::get_exam_overview(&state.pool, &exam_id, client_id).await?;
    Ok(Json(detail))
}

/// `GET /api/exams/{id}/questions` — loads one question in scope and updates `lastQuestionIndex`.
async fn get_exam_question_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(exam_id): Path<String>,
    Query(query): Query<ExamQuestionQuery>,
) -> Result<Json<ExamQuestionPayload>, AppError> {
    let client_id = parse_optional_client_id(&headers)?;

    if let Some(q) = query.q
        && q < 1
    {
        return Err(AppError::BadRequest(
            "query parameter 'q' must be >= 1".to_string(),
        ));
    }

    let topic_id = query
        .topic
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let payload =
        exam_repo::get_exam_question(&state.pool, &exam_id, client_id, topic_id, query.q).await?;
    Ok(Json(payload))
}

/// `POST /api/exams/{id}/answers` — evaluates and persists a submitted answer.
async fn submit_exam_answer_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(exam_id): Path<String>,
    Json(payload): Json<SubmitAnswerRequest>,
) -> Result<Json<ExamAnswerRecord>, AppError> {
    let client_id = parse_optional_client_id(&headers)?;

    if payload.question_id.trim().is_empty() {
        return Err(AppError::BadRequest(
            "questionId must not be empty".to_string(),
        ));
    }
    if payload.question_index < 1 {
        return Err(AppError::BadRequest(
            "questionIndex must be >= 1".to_string(),
        ));
    }

    let record = exam_repo::submit_exam_answer(&state.pool, &exam_id, client_id, &payload).await?;
    Ok(Json(record))
}

/// Sub-router mounted under `/api`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/exams", get(list_exams_handler))
        .route("/exams/{id}", get(get_exam_overview_handler))
        .route("/exams/{id}/questions", get(get_exam_question_handler))
        .route("/exams/{id}/answers", post(submit_exam_answer_handler))
}
