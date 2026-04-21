use super::list_trackers::{TrackerDefaultStatus, TrackerItem};
use crate::application::errors::ApplicationError;
use crate::domain::repositories::{IssueStatusRepository, NewTracker, TrackerRepository};
use std::sync::Arc;

/// Request for creating a new tracker
#[derive(Debug, Clone)]
pub struct CreateTrackerRequest {
    pub name: String,
    pub default_status_id: Option<i32>,
    pub description: Option<String>,
    pub enabled_standard_fields: Option<Vec<String>>,
}

/// Response for create tracker endpoint
#[derive(Debug, Clone)]
pub struct CreateTrackerResponse {
    pub tracker: TrackerItem,
}

/// Use case for creating a new tracker
pub struct CreateTrackerUseCase<T: TrackerRepository, S: IssueStatusRepository> {
    tracker_repo: Arc<T>,
    status_repo: Arc<S>,
}

impl<T: TrackerRepository, S: IssueStatusRepository> CreateTrackerUseCase<T, S> {
    pub fn new(tracker_repo: Arc<T>, status_repo: Arc<S>) -> Self {
        Self {
            tracker_repo,
            status_repo,
        }
    }

    /// Execute the use case
    ///
    /// Only admin users can create trackers (permission checked at API layer)
    pub async fn execute(
        &self,
        request: CreateTrackerRequest,
    ) -> Result<CreateTrackerResponse, ApplicationError> {
        // Validate name is not empty
        if request.name.trim().is_empty() {
            return Err(ApplicationError::Validation(
                "Tracker name cannot be empty".into(),
            ));
        }

        // Check if tracker with same name already exists
        if self
            .tracker_repo
            .exists_by_name(&request.name)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
        {
            return Err(ApplicationError::AlreadyExists(format!(
                "Tracker with name '{}' already exists",
                request.name
            )));
        }

        // Get default status ID (use provided or default status)
        let default_status_id = match request.default_status_id {
            Some(id) => {
                // Validate status exists
                self.status_repo
                    .find_by_id(id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?
                    .ok_or_else(|| {
                        ApplicationError::Validation(format!(
                            "Issue status with id {} not found",
                            id
                        ))
                    })?;
                id
            }
            None => {
                // Use the default status from the system
                self.status_repo
                    .find_default()
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?
                    .ok_or_else(|| {
                        ApplicationError::Validation("No default issue status found".into())
                    })?
                    .id
            }
        };

        // Convert enabled_standard_fields to fields_bits
        let fields_bits = request
            .enabled_standard_fields
            .map(|fields| fields_to_bits(&fields));

        // Create new tracker
        let new_tracker = NewTracker {
            name: request.name.trim().to_string(),
            position: None,
            is_in_roadmap: true,
            fields_bits,
            default_status_id,
        };

        let tracker = self
            .tracker_repo
            .create(&new_tracker)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Get default status for response
        let default_status = self
            .status_repo
            .find_by_id(tracker.default_status_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .map(|s| TrackerDefaultStatus {
                id: s.id,
                name: s.name,
            });

        Ok(CreateTrackerResponse {
            tracker: TrackerItem::from_tracker(&tracker, default_status),
        })
    }
}

/// Convert field names to bits
pub fn fields_to_bits(fields: &[String]) -> i32 {
    let all_fields = vec![
        "assigned_to_id",
        "category_id",
        "fixed_version_id",
        "parent_issue_id",
        "start_date",
        "due_date",
        "estimated_hours",
        "done_ratio",
        "description",
        "priority_id",
    ];

    let mut bits = 0i32;
    for field in fields {
        if let Some(pos) = all_fields.iter().position(|f| f == field) {
            bits |= 1 << pos;
        }
    }
    bits
}
