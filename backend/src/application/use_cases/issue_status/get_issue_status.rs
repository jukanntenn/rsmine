use super::list_issue_statuses::IssueStatusItem;
use crate::application::errors::ApplicationError;
use crate::domain::repositories::IssueStatusRepository;
use std::sync::Arc;

/// Response for get issue status endpoint
#[derive(Debug, Clone)]
pub struct GetIssueStatusResponse {
    pub issue_status: IssueStatusItem,
}

/// Use case for getting a single issue status by ID
pub struct GetIssueStatusUseCase<T: IssueStatusRepository> {
    status_repo: Arc<T>,
}

impl<T: IssueStatusRepository> GetIssueStatusUseCase<T> {
    pub fn new(status_repo: Arc<T>) -> Self {
        Self { status_repo }
    }

    /// Execute the use case
    ///
    /// Returns a single issue status by ID.
    /// Any logged-in user can view issue statuses.
    pub async fn execute(&self, id: i32) -> Result<GetIssueStatusResponse, ApplicationError> {
        // Get issue status by ID
        let status = self
            .status_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Issue status with id {} not found", id))
            })?;

        Ok(GetIssueStatusResponse {
            issue_status: IssueStatusItem::from(&status),
        })
    }
}
