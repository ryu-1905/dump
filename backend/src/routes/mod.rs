pub mod exams;

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

use crate::state::AppState;

async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

/// Builds the top-level Axum router with health check and `/api` routes.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/api", exams::router())
        .with_state(state)
}
