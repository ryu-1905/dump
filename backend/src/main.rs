mod config;
mod error;
mod models;
mod repositories;
mod routes;
mod state;

use std::net::SocketAddr;

use axum::http::{HeaderName, HeaderValue, Method, header};
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{EnvFilter, fmt};

use crate::{config::Config, error::AppError, state::AppState};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("backend=debug,tower_http=debug,info")),
        )
        .init();

    let config = Config::from_env()?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    let origins = config
        .cors_origins
        .iter()
        .map(|raw| {
            raw.parse::<HeaderValue>()
                .map_err(|_| AppError::Config(format!("invalid CORS_ORIGIN entry: {raw}")))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, HeaderName::from_static("x-client-id")]);

    let app = routes::create_router(AppState::new(pool))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|err| AppError::Config(format!("failed to bind {addr}: {err}")))?;

    axum::serve(listener, app)
        .await
        .map_err(|err| AppError::Config(format!("server error: {err}")))?;

    Ok(())
}
