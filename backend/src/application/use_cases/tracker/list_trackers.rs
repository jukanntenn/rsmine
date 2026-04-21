use crate::application::errors::ApplicationError;
use crate::domain::entities::{DefaultStatus, Tracker};
use crate::domain::repositories::{IssueStatusRepository, TrackerRepository};
use std::sync::Arc;

/// Default status info for API response
#[derive(Debug, Clone)]
pub struct TrackerDefaultStatus {
    pub id: i32,
    pub name: String,
}

impl From<DefaultStatus> for TrackerDefaultStatus {
    fn from(status: DefaultStatus) -> Self {
        Self {
            id: status.id,
            name: status.name,
        }
    }
}

/// Tracker item in list response
#[derive(Debug, Clone)]
pub struct TrackerItem {
    pub id: i32,
    pub name: String,
    pub position: Option<i32>,
    pub is_in_roadmap: bool,
    pub default_status: Option<TrackerDefaultStatus>,
    pub description: Option<String>,
    pub enabled_standard_fields: Vec<String>,
}

impl TrackerItem {
    pub fn from_tracker(tracker: &Tracker, default_status: Option<TrackerDefaultStatus>) -> Self {
        Self {
            id: tracker.id,
            name: tracker.name.clone(),
            position: tracker.position,
            is_in_roadmap: tracker.is_in_roadmap,
            default_status,
            description: None, // Description field doesn't exist in the database schema
            enabled_standard_fields: tracker.enabled_standard_fields(),
        }
    }
}

/// Response for tracker list endpoint
#[derive(Debug, Clone)]
pub struct TrackerListResponse {
    pub trackers: Vec<TrackerItem>,
}

/// Use case for listing all trackers
pub struct ListTrackersUseCase<T: TrackerRepository, S: IssueStatusRepository> {
    tracker_repo: Arc<T>,
    status_repo: Arc<S>,
}

impl<T: TrackerRepository, S: IssueStatusRepository> ListTrackersUseCase<T, S> {
    pub fn new(tracker_repo: Arc<T>, status_repo: Arc<S>) -> Self {
        Self {
            tracker_repo,
            status_repo,
        }
    }

    /// Execute the use case
    ///
    /// Returns all trackers with their default status information.
    /// Any logged-in user can list trackers.
    pub async fn execute(&self) -> Result<TrackerListResponse, ApplicationError> {
        // Get all trackers
        let trackers = self
            .tracker_repo
            .find_all()
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Build response with default status info
        let mut tracker_items = Vec::new();
        for tracker in trackers {
            // Get default status info
            let default_status = self
                .status_repo
                .find_by_id(tracker.default_status_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .map(|s| TrackerDefaultStatus {
                    id: s.id,
                    name: s.name,
                });

            tracker_items.push(TrackerItem::from_tracker(&tracker, default_status));
        }

        Ok(TrackerListResponse {
            trackers: tracker_items,
        })
    }
}
