use crate::application::errors::ApplicationError;
use crate::domain::entities::IssueStatus;
use crate::domain::repositories::IssueStatusRepository;
use std::sync::Arc;

/// Issue status item in list response
#[derive(Debug, Clone)]
pub struct IssueStatusItem {
    pub id: i32,
    pub name: String,
    pub is_closed: bool,
    pub is_default: bool,
    pub default_done_ratio: Option<i32>,
}

impl From<IssueStatus> for IssueStatusItem {
    fn from(status: IssueStatus) -> Self {
        Self {
            id: status.id,
            name: status.name,
            is_closed: status.is_closed,
            is_default: status.is_default,
            default_done_ratio: status.default_done_ratio,
        }
    }
}

impl From<&IssueStatus> for IssueStatusItem {
    fn from(status: &IssueStatus) -> Self {
        Self {
            id: status.id,
            name: status.name.clone(),
            is_closed: status.is_closed,
            is_default: status.is_default,
            default_done_ratio: status.default_done_ratio,
        }
    }
}

/// Response for issue status list endpoint
#[derive(Debug, Clone)]
pub struct IssueStatusListResponse {
    pub issue_statuses: Vec<IssueStatusItem>,
}

/// Use case for listing all issue statuses
pub struct ListIssueStatusesUseCase<T: IssueStatusRepository> {
    status_repo: Arc<T>,
}

impl<T: IssueStatusRepository> ListIssueStatusesUseCase<T> {
    pub fn new(status_repo: Arc<T>) -> Self {
        Self { status_repo }
    }

    /// Execute the use case
    ///
    /// Returns all issue statuses ordered by position.
    /// Any logged-in user can list issue statuses.
    pub async fn execute(&self) -> Result<IssueStatusListResponse, ApplicationError> {
        // Get all issue statuses
        let statuses = self
            .status_repo
            .find_all()
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Build response
        let status_items: Vec<IssueStatusItem> =
            statuses.into_iter().map(IssueStatusItem::from).collect();

        Ok(IssueStatusListResponse {
            issue_statuses: status_items,
        })
    }
}
