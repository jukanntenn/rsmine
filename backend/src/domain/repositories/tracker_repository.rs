use super::RepositoryError;
use crate::domain::entities::Tracker;
use async_trait::async_trait;

#[async_trait]
pub trait TrackerRepository: Send + Sync {
    /// Find all trackers
    async fn find_all(&self) -> Result<Vec<Tracker>, RepositoryError>;

    /// Find a tracker by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<Tracker>, RepositoryError>;

    /// Find trackers by project ID
    async fn find_by_project(&self, project_id: i32) -> Result<Vec<Tracker>, RepositoryError>;

    /// Create a new tracker
    async fn create(&self, tracker: &NewTracker) -> Result<Tracker, RepositoryError>;

    /// Update an existing tracker
    async fn update(&self, tracker: &Tracker) -> Result<Tracker, RepositoryError>;

    /// Delete a tracker by ID
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;

    /// Check if a tracker with the given name exists
    async fn exists_by_name(&self, name: &str) -> Result<bool, RepositoryError>;

    /// Check if a tracker with the given name exists (excluding a specific ID)
    async fn exists_by_name_excluding(
        &self,
        name: &str,
        exclude_id: i32,
    ) -> Result<bool, RepositoryError>;

    /// Replace all project associations for a tracker
    async fn set_projects(
        &self,
        tracker_id: i32,
        project_ids: &[i32],
    ) -> Result<(), RepositoryError>;
}

/// Data for creating a new tracker
#[derive(Debug, Clone)]
pub struct NewTracker {
    pub name: String,
    pub position: Option<i32>,
    pub is_in_roadmap: bool,
    pub fields_bits: Option<i32>,
    pub default_status_id: i32,
}
