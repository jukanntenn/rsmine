use super::list_issue_statuses::IssueStatusItem;
use crate::application::errors::ApplicationError;
use crate::domain::entities::IssueStatus;
use crate::domain::repositories::IssueStatusRepository;
use std::sync::Arc;

/// Request for updating an issue status
#[derive(Debug, Clone)]
pub struct UpdateIssueStatusRequest {
    pub name: Option<String>,
    pub is_closed: Option<bool>,
    pub is_default: Option<bool>,
    pub default_done_ratio: Option<i32>,
}

/// Response for update issue status endpoint
#[derive(Debug, Clone)]
pub struct UpdateIssueStatusResponse {
    pub issue_status: IssueStatusItem,
}

/// Use case for updating an existing issue status
pub struct UpdateIssueStatusUseCase<T: IssueStatusRepository> {
    status_repo: Arc<T>,
}

impl<T: IssueStatusRepository> UpdateIssueStatusUseCase<T> {
    pub fn new(status_repo: Arc<T>) -> Self {
        Self { status_repo }
    }

    /// Execute the use case
    ///
    /// Only admin users can update issue statuses (permission checked at API layer)
    pub async fn execute(
        &self,
        id: i32,
        request: UpdateIssueStatusRequest,
    ) -> Result<UpdateIssueStatusResponse, ApplicationError> {
        // Get existing status
        let status = self
            .status_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Issue status with id {} not found", id))
            })?;

        // Build updated status
        let mut updated_status = IssueStatus {
            id: status.id,
            name: status.name,
            position: status.position,
            is_closed: status.is_closed,
            is_default: status.is_default,
            default_done_ratio: status.default_done_ratio,
        };

        // Validate and update name if provided
        if let Some(name) = request.name {
            if name.trim().is_empty() {
                return Err(ApplicationError::Validation(
                    "Issue status name cannot be empty".into(),
                ));
            }

            // Check if another status with same name already exists
            if self
                .status_repo
                .exists_by_name_excluding(&name, id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
            {
                return Err(ApplicationError::AlreadyExists(format!(
                    "Issue status with name '{}' already exists",
                    name
                )));
            }

            updated_status.name = name.trim().to_string();
        }

        // Update is_closed if provided
        if let Some(is_closed) = request.is_closed {
            updated_status.is_closed = is_closed;
        }

        // Update is_default if provided
        if let Some(is_default) = request.is_default {
            // If setting as default, clear other defaults first
            if is_default {
                self.status_repo
                    .clear_default()
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;
            }
            updated_status.is_default = is_default;
        }

        // Update default_done_ratio if provided
        if let Some(default_done_ratio) = request.default_done_ratio {
            updated_status.default_done_ratio = Some(default_done_ratio);
        }

        // Save changes
        let result = self
            .status_repo
            .update(&updated_status)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(UpdateIssueStatusResponse {
            issue_status: IssueStatusItem::from(&result),
        })
    }
}
