use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
