use crate::application::errors::ApplicationError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            HttpError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            HttpError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            HttpError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            HttpError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            HttpError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "errors": [message],
        }));

        (status, body).into_response()
    }
}

impl From<ApplicationError> for HttpError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::NotFound(msg) => HttpError::NotFound(msg),
            ApplicationError::Validation(msg) => HttpError::BadRequest(msg),
            ApplicationError::PermissionDenied(msg) => HttpError::Forbidden(msg),
            ApplicationError::AlreadyExists(msg) => HttpError::BadRequest(msg),
            ApplicationError::Authentication(msg) => HttpError::Unauthorized(msg),
            ApplicationError::Unauthorized(msg) => HttpError::Unauthorized(msg),
            ApplicationError::Forbidden(msg) => HttpError::Forbidden(msg),
            ApplicationError::Internal(msg) => HttpError::Internal(msg),
        }
    }
}
