use sqlx::PgPool;

/// Shared application state injected into Axum handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    /// Creates a new `AppState` wrapping the PostgreSQL connection pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
