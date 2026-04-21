use super::RepositoryError;
use crate::domain::entities::IssueStatus;
use async_trait::async_trait;

#[async_trait]
pub trait IssueStatusRepository: Send + Sync {
    /// Find all issue statuses
    async fn find_all(&self) -> Result<Vec<IssueStatus>, RepositoryError>;

    /// Find an issue status by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<IssueStatus>, RepositoryError>;

    /// Find the default issue status (used for new issues)
    async fn find_default(&self) -> Result<Option<IssueStatus>, RepositoryError>;

    /// Find all open (not closed) issue statuses
    async fn find_open(&self) -> Result<Vec<IssueStatus>, RepositoryError>;

    /// Find all closed issue statuses
    async fn find_closed(&self) -> Result<Vec<IssueStatus>, RepositoryError>;

    /// Create a new issue status
    async fn create(&self, status: &NewIssueStatus) -> Result<IssueStatus, RepositoryError>;

    /// Update an existing issue status
    async fn update(&self, status: &IssueStatus) -> Result<IssueStatus, RepositoryError>;

    /// Delete an issue status by ID
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;

    /// Check if a status with the given name exists
    async fn exists_by_name(&self, name: &str) -> Result<bool, RepositoryError>;

    /// Check if a status with the given name exists (excluding a specific ID)
    async fn exists_by_name_excluding(
        &self,
        name: &str,
        exclude_id: i32,
    ) -> Result<bool, RepositoryError>;

    /// Clear the default flag on all statuses (used when setting a new default)
    async fn clear_default(&self) -> Result<(), RepositoryError>;

    /// Count issues with a specific status
    async fn count_issues_by_status(&self, status_id: i32) -> Result<u64, RepositoryError>;

    /// Reassign all issues from one status to another
    async fn reassign_issues_status(
        &self,
        from_status_id: i32,
        to_status_id: i32,
    ) -> Result<u64, RepositoryError>;
}

/// Data for creating a new issue status
#[derive(Debug, Clone)]
pub struct NewIssueStatus {
    pub name: String,
    pub position: Option<i32>,
    pub is_closed: bool,
    pub is_default: bool,
    pub default_done_ratio: Option<i32>,
}
