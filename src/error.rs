use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErr {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found")]
    NotFound,

    #[error("Unexpected error: {0}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppErr {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppErr::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database Error"),
            AppErr::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppErr::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppErr::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
        };

        let body = axum::Json(serde_json::json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}