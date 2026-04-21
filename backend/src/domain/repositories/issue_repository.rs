use super::RepositoryError;
use crate::domain::entities::Issue;
use crate::domain::value_objects::IssueQueryParams;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// Data for creating a new issue
#[derive(Debug, Clone)]
pub struct NewIssue {
    pub tracker_id: i32,
    pub project_id: i32,
    pub subject: String,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub category_id: Option<i32>,
    pub status_id: i32,
    pub assigned_to_id: Option<i32>,
    pub priority_id: i32,
    pub author_id: i32,
    pub start_date: Option<chrono::NaiveDate>,
    pub estimated_hours: Option<f64>,
    pub parent_id: Option<i32>,
    pub is_private: bool,
}

/// Data for updating an existing issue
#[derive(Debug, Clone)]
pub struct IssueUpdate {
    pub subject: Option<String>,
    pub description: Option<String>,
    pub status_id: Option<i32>,
    pub priority_id: Option<i32>,
    pub tracker_id: Option<i32>,
    pub assigned_to_id: Option<Option<i32>>,
    pub category_id: Option<Option<i32>>,
    pub parent_id: Option<Option<i32>>,
    pub start_date: Option<Option<chrono::NaiveDate>>,
    pub due_date: Option<Option<chrono::NaiveDate>>,
    pub estimated_hours: Option<Option<f64>>,
    pub done_ratio: Option<i32>,
    pub is_private: Option<bool>,
    pub updated_on: Option<DateTime<Utc>>,
    pub closed_on: Option<Option<DateTime<Utc>>>,
}

#[async_trait]
pub trait IssueRepository: Send + Sync {
    /// Find all issues matching the query parameters
    async fn find_all(&self, params: IssueQueryParams) -> Result<Vec<Issue>, RepositoryError>;

    /// Count issues matching the query parameters
    async fn count(&self, params: &IssueQueryParams) -> Result<u32, RepositoryError>;

    /// Find all issues for a project
    async fn find_by_project(&self, project_id: i32) -> Result<Vec<Issue>, RepositoryError>;

    /// Delete all issues for a project
    async fn delete_by_project(&self, project_id: i32) -> Result<(), RepositoryError>;

    /// Find an issue by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<Issue>, RepositoryError>;

    /// Clear assignee on all issues assigned to a user in a specific project
    /// Used when removing a member from a project
    async fn clear_assignee_in_project(
        &self,
        project_id: i32,
        user_id: i32,
    ) -> Result<(), RepositoryError>;

    /// Create a new issue
    async fn create(&self, issue: NewIssue) -> Result<Issue, RepositoryError>;

    /// Update an existing issue
    async fn update(&self, id: i32, update: IssueUpdate) -> Result<Issue, RepositoryError>;

    /// Find all children (subtasks) of an issue
    async fn find_children(&self, parent_id: i32) -> Result<Vec<Issue>, RepositoryError>;

    /// Delete an issue by ID
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}
