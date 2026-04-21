use super::RepositoryError;
use crate::domain::entities::{Attachment, TempAttachment};
use async_trait::async_trait;

/// New attachment data for creation
#[derive(Debug, Clone)]
pub struct NewAttachment {
    pub container_id: Option<i32>,
    pub container_type: Option<String>,
    pub filename: String,
    pub disk_filename: String,
    pub filesize: i64,
    pub content_type: Option<String>,
    pub digest: Option<String>,
    pub author_id: i32,
    pub description: Option<String>,
    pub disk_directory: Option<String>,
}

#[async_trait]
pub trait AttachmentRepository: Send + Sync {
    /// Find all attachments for a container (e.g., project, issue)
    async fn find_by_container(
        &self,
        container_id: i32,
        container_type: &str,
    ) -> Result<Vec<Attachment>, RepositoryError>;

    /// Find an attachment by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<Attachment>, RepositoryError>;

    /// Create a new attachment
    async fn create(&self, attachment: NewAttachment) -> Result<Attachment, RepositoryError>;

    /// Delete all attachments for a container
    async fn delete_by_container(
        &self,
        container_id: i32,
        container_type: &str,
    ) -> Result<(), RepositoryError>;

    /// Delete an attachment by ID
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;

    /// Increment the download count for an attachment
    async fn increment_downloads(&self, id: i32) -> Result<(), RepositoryError>;

    /// Check if other attachments (excluding the given ID) have the same digest
    /// Used for file deduplication during delete
    async fn has_other_with_digest(
        &self,
        digest: &str,
        exclude_id: i32,
    ) -> Result<bool, RepositoryError>;

    /// Update attachment description
    async fn update_description(
        &self,
        id: i32,
        description: Option<String>,
    ) -> Result<Attachment, RepositoryError>;
}

/// Trait for managing temporary upload tokens
/// Implementations can use in-memory storage, database, or cache
#[async_trait]
pub trait TempAttachmentStore: Send + Sync {
    /// Store a temporary attachment
    async fn store(&self, temp: TempAttachment) -> Result<(), RepositoryError>;

    /// Retrieve and remove a temporary attachment by token
    async fn take(&self, token: &str) -> Result<Option<TempAttachment>, RepositoryError>;

    /// Clean up expired temporary attachments
    async fn cleanup_expired(&self) -> Result<usize, RepositoryError>;
}
