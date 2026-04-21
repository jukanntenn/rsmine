use crate::application::errors::ApplicationError;
use crate::domain::repositories::{IssueQueryParams, IssueRepository, TrackerRepository};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Use case for deleting a tracker
pub struct DeleteTrackerUseCase<T: TrackerRepository, I: IssueRepository> {
    tracker_repo: Arc<T>,
    issue_repo: Arc<I>,
}

impl<T: TrackerRepository, I: IssueRepository> DeleteTrackerUseCase<T, I> {
    pub fn new(tracker_repo: Arc<T>, issue_repo: Arc<I>) -> Self {
        Self {
            tracker_repo,
            issue_repo,
        }
    }

    /// Execute the use case
    ///
    /// Only admin users can delete trackers (permission checked at API layer)
    ///
    /// # Errors
    /// - `NotFound`: Tracker with the given ID does not exist
    /// - `BadRequest`: Tracker is in use by issues and cannot be deleted
    pub async fn execute(
        &self,
        id: i32,
        _current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Check if tracker exists
        let tracker = self
            .tracker_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Tracker with id {} not found", id))
            })?;

        // Check if tracker is in use by any issues
        let query_params = IssueQueryParams {
            tracker_id: Some(tracker.id),
            ..Default::default()
        };

        let issue_count = self
            .issue_repo
            .count(&query_params)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if issue_count > 0 {
            return Err(ApplicationError::Validation(format!(
                "Cannot delete tracker '{}' because it is used by {} issue(s)",
                tracker.name, issue_count
            )));
        }

        // Delete tracker
        self.tracker_repo
            .delete(tracker.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(())
    }
}
