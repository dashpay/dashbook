use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Rpc { code: i32, message: String },
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Rpc { code, message } => write!(f, "RPC error {}: {}", code, message),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Rpc { code, message } => {
                // RPC error codes -5 and -8 typically mean "not found"
                if *code == -5 || *code == -8 {
                    (StatusCode::NOT_FOUND, message.clone())
                } else {
                    (
                        StatusCode::BAD_GATEWAY,
                        format!("RPC error {}: {}", code, message),
                    )
                }
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = json!({ "error": message });
        (status, axum::Json(body)).into_response()
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Internal(format!("HTTP client error: {}", e))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Internal(format!("JSON error: {}", e))
    }
}
