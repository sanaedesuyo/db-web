use std::error::Error;
use std::fmt::Display;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppError {
    pub error: String,
}

impl Error for AppError {
    fn description(&self) -> &str {
        self.error.as_str()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError {
            error: error.to_string(),
        }
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.error).into_response()
    }
}

impl AppError {
    pub fn new(error: &str) -> Self {
        AppError { error: error.into() }
    }
}