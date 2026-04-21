use super::list_trackers::TrackerItem;
use crate::application::errors::ApplicationError;
use crate::domain::repositories::{IssueStatusRepository, TrackerRepository};
use std::sync::Arc;

/// Response for get tracker endpoint
#[derive(Debug, Clone)]
pub struct GetTrackerResponse {
    pub tracker: TrackerItem,
}

/// Use case for getting a single tracker by ID
pub struct GetTrackerUseCase<T: TrackerRepository, S: IssueStatusRepository> {
    tracker_repo: Arc<T>,
    status_repo: Arc<S>,
}

impl<T: TrackerRepository, S: IssueStatusRepository> GetTrackerUseCase<T, S> {
    pub fn new(tracker_repo: Arc<T>, status_repo: Arc<S>) -> Self {
        Self {
            tracker_repo,
            status_repo,
        }
    }

    /// Execute the use case
    pub async fn execute(&self, id: i32) -> Result<GetTrackerResponse, ApplicationError> {
        // Get tracker by ID
        let tracker = self
            .tracker_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Tracker with id {} not found", id))
            })?;

        // Get default status info
        let default_status = self
            .status_repo
            .find_by_id(tracker.default_status_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .map(|s| super::list_trackers::TrackerDefaultStatus {
                id: s.id,
                name: s.name,
            });

        Ok(GetTrackerResponse {
            tracker: TrackerItem::from_tracker(&tracker, default_status),
        })
    }
}
