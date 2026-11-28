use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::HeaderMap,
    routing::{get, post, put},
};

use crate::{
    error::AppError,
    models::exam::{
        ClientInfoResponse, ExamAnswerRecord, ExamDetail, ExamListQuery, ExamQuestionPayload,
        ExamQuestionQuery, NormalizedExamListParams, PaginatedExamsResponse, RenameClientRequest,
        SubmitAnswerRequest, SyncClientRequest,
    },
    repositories::exam_repo,
    state::AppState,
};

const CLIENT_ID_HEADER: &str = "x-client-id";

/// Validates a client ID (`3..=64` ASCII alphanumeric, `-`, or `_` characters).
fn validate_client_id(raw: &str) -> Result<String, AppError> {
    let trimmed = raw.trim();
    if trimmed.len() < 3 || trimmed.len() > 64 {
        return Err(AppError::BadRequest(
            "client ID must be between 3 and 64 characters".to_string(),
        ));
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(AppError::BadRequest(
            "client ID may only contain letters, numbers, '-' and '_'".to_string(),
        ));
    }
    Ok(trimmed.to_string())
}

/// Parses optional `X-Client-Id` header; rejects malformed non-empty values with 400.
fn parse_optional_client_id(headers: &HeaderMap) -> Result<Option<String>, AppError> {
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

    Ok(Some(validate_client_id(raw)?))
}

/// Requires a valid `X-Client-Id` header.
fn require_client_id(headers: &HeaderMap) -> Result<String, AppError> {
    parse_optional_client_id(headers)?
        .ok_or_else(|| AppError::BadRequest("missing X-Client-Id header".to_string()))
}

/// `GET /api/exams` — returns paginated exam summaries with optional client progress.
async fn list_exams_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ExamListQuery>,
) -> Result<Json<PaginatedExamsResponse>, AppError> {
    let client_id = parse_optional_client_id(&headers)?;
    let params = NormalizedExamListParams::try_from(query)?;
    let response = exam_repo::list_exams(&state.pool, client_id.as_deref(), &params).await?;
    Ok(Json(response))
}

/// `GET /api/exams/{id}` — returns full exam overview and per-topic progress.
async fn get_exam_overview_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(exam_id): Path<String>,
) -> Result<Json<ExamDetail>, AppError> {
    let client_id = parse_optional_client_id(&headers)?;
    let detail = exam_repo::get_exam_overview(&state.pool, &exam_id, client_id.as_deref()).await?;
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

    let payload = exam_repo::get_exam_question(
        &state.pool,
        &exam_id,
        client_id.as_deref(),
        topic_id,
        query.q,
    )
    .await?;
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

    let record =
        exam_repo::submit_exam_answer(&state.pool, &exam_id, client_id.as_deref(), &payload)
            .await?;
    Ok(Json(record))
}

/// `POST /api/clients/register` — registers a new unique client ID (generates one if taken).
async fn register_client_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ClientInfoResponse>, AppError> {
    let candidate = parse_optional_client_id(&headers)?;
    let info = exam_repo::register_unique_client(&state.pool, candidate.as_deref()).await?;
    Ok(Json(info))
}

/// `POST /api/clients/ensure` — ensures an existing client ID is recorded in PostgreSQL.
async fn ensure_client_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ClientInfoResponse>, AppError> {
    let client_id = require_client_id(&headers)?;
    let info = exam_repo::ensure_client_id(&state.pool, &client_id).await?;
    Ok(Json(info))
}

/// `PUT /api/clients/id` — renames the client ID if the requested new ID is not taken.
async fn rename_client_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RenameClientRequest>,
) -> Result<Json<ClientInfoResponse>, AppError> {
    let old_client_id = parse_optional_client_id(&headers)?;
    let new_client_id = validate_client_id(&payload.new_client_id)?;
    let info =
        exam_repo::rename_client_id(&state.pool, old_client_id.as_deref(), &new_client_id).await?;
    Ok(Json(info))
}

/// `POST /api/clients/sync` — verifies `targetClientId` exists, deletes current client data, and switches ID.
async fn sync_client_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SyncClientRequest>,
) -> Result<Json<ClientInfoResponse>, AppError> {
    let current_client_id = parse_optional_client_id(&headers)?;
    let target_client_id = validate_client_id(&payload.target_client_id)?;
    let info =
        exam_repo::sync_client_id(&state.pool, current_client_id.as_deref(), &target_client_id)
            .await?;
    Ok(Json(info))
}

/// Sub-router mounted under `/api`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/exams", get(list_exams_handler))
        .route("/exams/{id}", get(get_exam_overview_handler))
        .route("/exams/{id}/questions", get(get_exam_question_handler))
        .route("/exams/{id}/answers", post(submit_exam_answer_handler))
        .route("/clients/register", post(register_client_handler))
        .route("/clients/ensure", post(ensure_client_handler))
        .route("/clients/id", put(rename_client_handler))
        .route("/clients/sync", post(sync_client_handler))
}
