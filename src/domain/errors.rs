use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    NotFound(String),
    UnauthorizedError(String),
    InvalidInput(String),
    ConflictError(String),
    InternalError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DomainError::UnauthorizedError(msg) => write!(f, "Unauthorized: {}", msg),
            DomainError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            DomainError::ConflictError(msg) => write!(f, "Conflict: {}", msg),
            DomainError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            DomainError::UnauthorizedError(msg) => (StatusCode::UNAUTHORIZED, msg),
            DomainError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            DomainError::ConflictError(msg) => (StatusCode::CONFLICT, msg),
            DomainError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<sea_orm::DbErr> for DomainError {
    fn from(err: sea_orm::DbErr) -> Self {
        match err {
            sea_orm::DbErr::RecordNotFound(_) => DomainError::NotFound("Resource not found".to_string()),
            _ => {
                tracing::error!("Database error: {:?}", err);
                DomainError::InternalError("Database error".to_string())
            }
        }
    }
}
