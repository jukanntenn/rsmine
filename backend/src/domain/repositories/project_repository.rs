use super::RepositoryError;
use crate::domain::entities::Project;
use async_trait::async_trait;

/// Project status constants for queries
pub const PROJECT_STATUS_ACTIVE: i32 = 1;
pub const PROJECT_STATUS_CLOSED: i32 = 5;
pub const PROJECT_STATUS_ARCHIVED: i32 = 9;

/// Query parameters for listing projects
#[derive(Debug, Clone, Default)]
pub struct ProjectQueryParams {
    pub status: Option<i32>,
    pub name: Option<String>,
    pub parent_id: Option<i32>,
    pub is_public: Option<bool>,
    pub offset: u32,
    pub limit: u32,
}

impl ProjectQueryParams {
    pub fn new(status: Option<i32>, name: Option<String>, offset: u32, limit: u32) -> Self {
        Self {
            status,
            name,
            parent_id: None,
            is_public: None,
            offset,
            limit: limit.min(100), // Max 100 items per page
        }
    }
}

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    /// Find all projects matching the query parameters
    async fn find_all(&self, params: ProjectQueryParams) -> Result<Vec<Project>, RepositoryError>;

    /// Count projects matching the query parameters
    async fn count(&self, params: &ProjectQueryParams) -> Result<u32, RepositoryError>;

    /// Find a project by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<Project>, RepositoryError>;

    /// Find a project by identifier
    async fn find_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<Project>, RepositoryError>;

    /// Find all visible projects for a user (public projects + projects they are a member of)
    async fn find_visible_for_user(
        &self,
        user_id: i32,
        params: ProjectQueryParams,
    ) -> Result<Vec<Project>, RepositoryError>;

    /// Count visible projects for a user
    async fn count_visible_for_user(
        &self,
        user_id: i32,
        params: &ProjectQueryParams,
    ) -> Result<u32, RepositoryError>;

    /// Find project IDs that a user is a member of
    async fn find_project_ids_by_user_membership(
        &self,
        user_id: i32,
    ) -> Result<Vec<i32>, RepositoryError>;

    /// Create a new project
    async fn create(&self, project: Project) -> Result<Project, RepositoryError>;

    /// Check if a project with the given identifier exists
    async fn exists_by_identifier(&self, identifier: &str) -> Result<bool, RepositoryError>;

    /// Get the maximum rgt value for nested set positioning
    async fn get_max_rgt(&self) -> Result<Option<i32>, RepositoryError>;

    /// Add a tracker to a project
    async fn add_tracker(&self, project_id: i32, tracker_id: i32) -> Result<(), RepositoryError>;

    /// Update nested set values for all projects when inserting a new node
    async fn update_nested_set_for_insert(&self, lft: i32) -> Result<(), RepositoryError>;

    /// Update an existing project
    async fn update(&self, project: Project) -> Result<Project, RepositoryError>;

    /// Set trackers for a project (replaces existing trackers)
    async fn set_trackers(
        &self,
        project_id: i32,
        tracker_ids: &[i32],
    ) -> Result<(), RepositoryError>;

    /// Check if a project with the given identifier exists, excluding a specific project ID
    async fn exists_by_identifier_excluding(
        &self,
        identifier: &str,
        exclude_project_id: i32,
    ) -> Result<bool, RepositoryError>;

    /// Find all child projects (subprojects) of a given project
    async fn find_children(&self, project_id: i32) -> Result<Vec<Project>, RepositoryError>;

    /// Delete a project by ID
    async fn delete(&self, project_id: i32) -> Result<(), RepositoryError>;

    /// Clear all tracker associations for a project
    async fn clear_trackers(&self, project_id: i32) -> Result<(), RepositoryError>;
}
