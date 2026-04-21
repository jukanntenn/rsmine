use super::RepositoryError;
use crate::domain::entities::Token;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// DTO for creating a new token
#[derive(Debug, Clone)]
pub struct CreateTokenDto {
    pub user_id: i32,
    pub action: String,
    pub value: String,
    pub validity_expires_on: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait TokenRepository: Send + Sync {
    /// Find a token by user ID and action type
    async fn find_by_user_and_action(
        &self,
        user_id: i32,
        action: &str,
    ) -> Result<Option<Token>, RepositoryError>;

    /// Find a token by its value
    async fn find_by_value(&self, value: &str) -> Result<Option<Token>, RepositoryError>;

    /// Delete all tokens for a user
    async fn delete_by_user_id(&self, user_id: i32) -> Result<(), RepositoryError>;

    /// Create a new token (used for blacklisting JWT tokens on logout)
    async fn create(&self, dto: CreateTokenDto) -> Result<Token, RepositoryError>;

    /// Delete expired tokens (cleanup for blacklisted tokens)
    async fn delete_expired(&self) -> Result<u64, RepositoryError>;
}
