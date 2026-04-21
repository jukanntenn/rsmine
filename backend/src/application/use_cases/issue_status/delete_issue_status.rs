use crate::application::errors::ApplicationError;
use crate::domain::repositories::IssueStatusRepository;
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Use case for deleting an issue status
pub struct DeleteIssueStatusUseCase<T: IssueStatusRepository> {
    status_repo: Arc<T>,
}

impl<T: IssueStatusRepository> DeleteIssueStatusUseCase<T> {
    pub fn new(status_repo: Arc<T>) -> Self {
        Self { status_repo }
    }

    /// Execute the use case
    ///
    /// Only admin users can delete issue statuses (permission checked at API layer)
    ///
    /// # Arguments
    /// * `id` - The ID of the status to delete
    /// * `reassign_to_id` - Optional ID of the status to reassign issues to
    /// * `_current_user` - The current user (for potential future permission checks)
    ///
    /// # Errors
    /// - `NotFound`: Status with the given ID does not exist
    /// - `Validation`: Status has issues and no reassign_to_id was provided
    /// - `Validation`: Target status for reassignment does not exist
    /// - `Validation`: Cannot reassign to the same status being deleted
    pub async fn execute(
        &self,
        id: i32,
        reassign_to_id: Option<i32>,
        _current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Check if status exists
        let status = self
            .status_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Issue status with id {} not found", id))
            })?;

        // Check if there are issues using this status
        let issue_count = self
            .status_repo
            .count_issues_by_status(status.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if issue_count > 0 {
            // Must provide a target status for reassignment
            let target_id = reassign_to_id.ok_or_else(|| {
                ApplicationError::Validation(format!(
                    "Cannot delete issue status '{}' because it is used by {} issue(s). Please provide reassign_to_id.",
                    status.name, issue_count
                ))
            })?;

            // Cannot reassign to the same status being deleted
            if target_id == status.id {
                return Err(ApplicationError::Validation(
                    "Cannot reassign issues to the status being deleted".to_string(),
                ));
            }

            // Check if target status exists
            let target_status = self
                .status_repo
                .find_by_id(target_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .ok_or_else(|| {
                    ApplicationError::NotFound(format!(
                        "Target issue status with id {} not found",
                        target_id
                    ))
                })?;

            // Reassign issues to the target status
            self.status_repo
                .reassign_issues_status(status.id, target_status.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
        }

        // Delete status
        self.status_repo
            .delete(status.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(())
    }
}
