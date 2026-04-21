use super::RepositoryError;
use crate::domain::entities::IssueCategory;
use async_trait::async_trait;

/// Data for creating a new issue category
#[derive(Debug, Clone)]
pub struct NewIssueCategory {
    pub project_id: i32,
    pub name: String,
    pub assigned_to_id: Option<i32>,
}

/// Data for updating an issue category
#[derive(Debug, Clone, Default)]
pub struct IssueCategoryUpdate {
    pub name: Option<String>,
    pub assigned_to_id: Option<Option<i32>>,
}

#[async_trait]
pub trait IssueCategoryRepository: Send + Sync {
    /// Find all issue categories for a project
    async fn find_by_project(&self, project_id: i32)
        -> Result<Vec<IssueCategory>, RepositoryError>;

    /// Find an issue category by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<IssueCategory>, RepositoryError>;

    /// Create a new issue category
    async fn create(&self, category: &NewIssueCategory) -> Result<IssueCategory, RepositoryError>;

    /// Update an existing issue category
    async fn update(
        &self,
        id: i32,
        category: &IssueCategoryUpdate,
    ) -> Result<IssueCategory, RepositoryError>;

    /// Delete an issue category by ID
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;

    /// Delete all issue categories for a project
    async fn delete_by_project(&self, project_id: i32) -> Result<(), RepositoryError>;

    /// Count issues in a category
    async fn count_issues(&self, category_id: i32) -> Result<u32, RepositoryError>;

    /// Reassign issues from one category to another
    async fn reassign_issues(
        &self,
        from_category_id: i32,
        to_category_id: i32,
    ) -> Result<(), RepositoryError>;

    /// Clear category assignment for issues in a category
    async fn clear_issues(&self, category_id: i32) -> Result<(), RepositoryError>;

    /// Check if a category with the given name exists in a project
    async fn exists_by_name(
        &self,
        project_id: i32,
        name: &str,
        exclude_id: Option<i32>,
    ) -> Result<bool, RepositoryError>;
}
