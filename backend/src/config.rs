use std::env;

use crate::error::AppError;

/// Runtime configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

impl Config {
    /// Loads configuration from process environment variables (after `.env` is loaded).
    pub fn from_env() -> Result<Self, AppError> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| AppError::Config("missing DATABASE_URL environment variable".to_string()))?;

        let port = match env::var("BACKEND_PORT").or_else(|_| env::var("PORT")) {
            Ok(raw) => raw
                .parse::<u16>()
                .map_err(|_| AppError::Config(format!("invalid port value: {raw}")))?,
            Err(_) => 8080,
        };

        let cors_origins: Vec<String> = env::var("CORS_ORIGIN")
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToOwned::to_owned)
            .collect();

        let cors_origins = if cors_origins.is_empty() {
            vec!["http://localhost:5173".to_string()]
        } else {
            cors_origins
        };

        Ok(Self {
            database_url,
            port,
            cors_origins,
        })
    }
}
