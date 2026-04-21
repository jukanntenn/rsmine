use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Entity already exists: {0}")]
    AlreadyExists(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
