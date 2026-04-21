use crate::application::errors::ApplicationError;
use crate::domain::entities::Enumeration;
use crate::domain::entities::ENUM_TYPE_ISSUE_PRIORITY;
use crate::domain::repositories::EnumerationRepository;
use std::sync::Arc;

/// Priority item in list response
#[derive(Debug, Clone)]
pub struct PriorityItem {
    pub id: i32,
    pub name: String,
    pub is_default: bool,
    pub active: bool,
}

impl From<Enumeration> for PriorityItem {
    fn from(e: Enumeration) -> Self {
        Self {
            id: e.id,
            name: e.name,
            is_default: e.is_default,
            active: e.active,
        }
    }
}

/// Response for priority list endpoint
#[derive(Debug, Clone)]
pub struct PriorityListResponse {
    pub issue_priorities: Vec<PriorityItem>,
}

/// Use case for listing all active issue priorities
pub struct ListPrioritiesUseCase<E: EnumerationRepository> {
    enum_repo: Arc<E>,
}

impl<E: EnumerationRepository> ListPrioritiesUseCase<E> {
    pub fn new(enum_repo: Arc<E>) -> Self {
        Self { enum_repo }
    }

    /// Execute the use case
    ///
    /// Returns all active issue priorities.
    /// Any logged-in user can list priorities.
    pub async fn execute(&self) -> Result<PriorityListResponse, ApplicationError> {
        // Get all active priorities
        let priorities = self
            .enum_repo
            .find_active_by_type(ENUM_TYPE_ISSUE_PRIORITY)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Build response
        let priority_items: Vec<PriorityItem> =
            priorities.into_iter().map(PriorityItem::from).collect();

        Ok(PriorityListResponse {
            issue_priorities: priority_items,
        })
    }
}
