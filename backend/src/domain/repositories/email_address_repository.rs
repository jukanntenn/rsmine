use super::RepositoryError;
use crate::domain::entities::EmailAddress;
use async_trait::async_trait;

#[async_trait]
pub trait EmailAddressRepository: Send + Sync {
    /// Find the default email address for a user
    async fn find_default_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<EmailAddress>, RepositoryError>;

    /// Find all email addresses for a user
    async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<EmailAddress>, RepositoryError>;

    /// Update the address of an existing email record
    async fn update_address(&self, email_id: i32, address: &str) -> Result<(), RepositoryError>;

    /// Create a new email address
    async fn create(&self, email: EmailAddress) -> Result<EmailAddress, RepositoryError>;

    /// Check if an email address exists (globally)
    async fn exists_by_address(&self, address: &str) -> Result<bool, RepositoryError>;

    /// Check if an email address exists (excluding a specific user ID)
    async fn exists_by_address_excluding_user(
        &self,
        address: &str,
        exclude_user_id: i32,
    ) -> Result<bool, RepositoryError>;

    /// Delete all email addresses for a user
    async fn delete_by_user_id(&self, user_id: i32) -> Result<(), RepositoryError>;
}
