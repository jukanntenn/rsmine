use super::list_issue_statuses::IssueStatusItem;
use crate::application::errors::ApplicationError;
use crate::domain::repositories::{IssueStatusRepository, NewIssueStatus};
use std::sync::Arc;

/// Response for create issue status endpoint
#[derive(Debug, Clone)]
pub struct CreateIssueStatusResponse {
    pub issue_status: IssueStatusItem,
}

/// Use case for creating a new issue status
pub struct CreateIssueStatusUseCase<T: IssueStatusRepository> {
    status_repo: Arc<T>,
}

impl<T: IssueStatusRepository> CreateIssueStatusUseCase<T> {
    pub fn new(status_repo: Arc<T>) -> Self {
        Self { status_repo }
    }

    /// Execute the use case
    ///
    /// Only admin users can create issue statuses (permission checked at API layer)
    pub async fn execute(
        &self,
        new_status: NewIssueStatus,
    ) -> Result<CreateIssueStatusResponse, ApplicationError> {
        // Validate name is not empty
        if new_status.name.trim().is_empty() {
            return Err(ApplicationError::Validation(
                "Issue status name cannot be empty".into(),
            ));
        }

        // Check if status with same name already exists
        if self
            .status_repo
            .exists_by_name(&new_status.name)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
        {
            return Err(ApplicationError::AlreadyExists(format!(
                "Issue status with name '{}' already exists",
                new_status.name
            )));
        }

        // If setting as default, clear other defaults first
        if new_status.is_default {
            self.status_repo
                .clear_default()
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
        }

        // Create new issue status
        let status = self
            .status_repo
            .create(&new_status)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(CreateIssueStatusResponse {
            issue_status: IssueStatusItem::from(&status),
        })
    }
}
