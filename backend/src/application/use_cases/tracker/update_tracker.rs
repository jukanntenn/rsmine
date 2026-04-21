use super::create_tracker::fields_to_bits;
use super::list_trackers::{TrackerDefaultStatus, TrackerItem};
use crate::application::errors::ApplicationError;
use crate::domain::repositories::{IssueStatusRepository, TrackerRepository};
use std::sync::Arc;

/// Request for updating a tracker
#[derive(Debug, Clone)]
pub struct UpdateTrackerRequest {
    pub name: Option<String>,
    pub position: Option<i32>,
    pub is_in_roadmap: Option<bool>,
    pub default_status_id: Option<i32>,
    pub enabled_standard_fields: Option<Vec<String>>,
    pub project_ids: Option<Vec<i32>>,
}

/// Response for update tracker endpoint
#[derive(Debug, Clone)]
pub struct UpdateTrackerResponse {
    pub tracker: TrackerItem,
}

/// Use case for updating an existing tracker
pub struct UpdateTrackerUseCase<T: TrackerRepository, S: IssueStatusRepository> {
    tracker_repo: Arc<T>,
    status_repo: Arc<S>,
}

impl<T: TrackerRepository, S: IssueStatusRepository> UpdateTrackerUseCase<T, S> {
    pub fn new(tracker_repo: Arc<T>, status_repo: Arc<S>) -> Self {
        Self {
            tracker_repo,
            status_repo,
        }
    }

    /// Execute the use case
    ///
    /// Only admin users can update trackers (permission checked at API layer)
    pub async fn execute(
        &self,
        id: i32,
        request: UpdateTrackerRequest,
    ) -> Result<UpdateTrackerResponse, ApplicationError> {
        // Get existing tracker
        let mut tracker = self
            .tracker_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Tracker with id {} not found", id))
            })?;

        // Validate and update name if provided
        if let Some(name) = request.name {
            if name.trim().is_empty() {
                return Err(ApplicationError::Validation(
                    "Tracker name cannot be empty".into(),
                ));
            }

            // Check if another tracker with same name already exists
            if self
                .tracker_repo
                .exists_by_name_excluding(&name, id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
            {
                return Err(ApplicationError::AlreadyExists(format!(
                    "Tracker with name '{}' already exists",
                    name
                )));
            }

            tracker.name = name.trim().to_string();
        }

        // Update position if provided
        if let Some(position) = request.position {
            tracker.position = Some(position);
        }

        // Update is_in_roadmap if provided
        if let Some(is_in_roadmap) = request.is_in_roadmap {
            tracker.is_in_roadmap = is_in_roadmap;
        }

        // Update default_status_id if provided
        if let Some(status_id) = request.default_status_id {
            self.status_repo
                .find_by_id(status_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .ok_or_else(|| {
                    ApplicationError::Validation(format!(
                        "Issue status with id {} not found",
                        status_id
                    ))
                })?;

            tracker.default_status_id = status_id;
        }

        // Update fields_bits if enabled_standard_fields provided
        if let Some(fields) = request.enabled_standard_fields {
            tracker.fields_bits = Some(fields_to_bits(&fields));
        }

        // Save changes
        let updated_tracker = self
            .tracker_repo
            .update(&tracker)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Update project associations if project_ids provided
        if let Some(project_ids) = request.project_ids {
            self.tracker_repo
                .set_projects(id, &project_ids)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
        }

        // Get default status for response
        let default_status = self
            .status_repo
            .find_by_id(updated_tracker.default_status_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .map(|s| TrackerDefaultStatus {
                id: s.id,
                name: s.name,
            });

        Ok(UpdateTrackerResponse {
            tracker: TrackerItem::from_tracker(&updated_tracker, default_status),
        })
    }
}
