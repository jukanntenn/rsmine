use crate::application::errors::ApplicationError;
use crate::domain::repositories::{
    AttachmentRepository, IssueRepository, MemberRepository, ProjectRepository,
};
use crate::infrastructure::storage::{LocalFileStorage, StorageError};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Use case for deleting an attachment
pub struct DeleteAttachmentUseCase<A, I, P, M>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    M: MemberRepository,
{
    attachment_repo: Arc<A>,
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    member_repo: Arc<M>,
    storage: Arc<LocalFileStorage>,
}

impl<A, I, P, M> DeleteAttachmentUseCase<A, I, P, M>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    M: MemberRepository,
{
    pub fn new(
        attachment_repo: Arc<A>,
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        member_repo: Arc<M>,
        storage: Arc<LocalFileStorage>,
    ) -> Self {
        Self {
            attachment_repo,
            issue_repo,
            project_repo,
            member_repo,
            storage,
        }
    }

    /// Execute the delete use case
    ///
    /// Permission rules:
    /// - Admin can delete all attachments
    /// - For Issue container: user must have manage_files permission on the issue's project
    /// - For Project container: user must have manage_files permission on the project
    /// - For MVP: member of the project can delete attachments
    pub async fn execute(
        &self,
        attachment_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // 1. Get attachment
        let attachment = self
            .attachment_repo
            .find_by_id(attachment_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Attachment {} not found", attachment_id))
            })?;

        // 2. Check permission
        self.check_delete_permission(&attachment, current_user)
            .await?;

        // 3. Check if other attachments use the same file (by digest)
        // If so, don't delete the physical file
        let should_delete_file = if let Some(ref digest) = attachment.digest {
            !self
                .attachment_repo
                .has_other_with_digest(digest, attachment_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
        } else {
            true // No digest means no deduplication, safe to delete
        };

        // 4. Delete from database first
        self.attachment_repo
            .delete(attachment_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 5. Delete file from disk if no other attachments use it
        if should_delete_file {
            let disk_directory = attachment.disk_directory.as_deref().unwrap_or("");
            match self
                .storage
                .delete(disk_directory, &attachment.disk_filename)
            {
                Ok(()) => {}
                Err(StorageError::NotFound(_)) => {
                    // File already gone, that's fine - database record is deleted
                }
                Err(StorageError::Io(io_err)) => {
                    return Err(ApplicationError::Internal(format!(
                        "Failed to delete attachment file: {}",
                        io_err
                    )));
                }
                Err(StorageError::InvalidFilename(msg)) => {
                    return Err(ApplicationError::Internal(msg));
                }
            }
        }

        Ok(())
    }

    /// Check if the current user has permission to delete this attachment
    async fn check_delete_permission(
        &self,
        attachment: &crate::domain::entities::Attachment,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Admin can delete all attachments
        if current_user.admin {
            return Ok(());
        }

        // Check container permission
        match (
            attachment.container_type.as_deref(),
            attachment.container_id,
        ) {
            (Some("Issue"), Some(issue_id)) => {
                self.check_issue_permission(issue_id, current_user).await
            }
            (Some("Project"), Some(project_id)) => {
                self.check_project_permission(project_id, current_user)
                    .await
            }
            _ => {
                // No container = no access (should not happen in normal flow)
                Err(ApplicationError::Forbidden(
                    "Attachment has no associated container".into(),
                ))
            }
        }
    }

    /// Check if user has manage_files permission for an issue
    async fn check_issue_permission(
        &self,
        issue_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Get the issue to find its project
        let issue = self
            .issue_repo
            .find_by_id(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Issue {} not found", issue_id)))?;

        // Check project permission
        self.check_project_permission(issue.project_id, current_user)
            .await
    }

    /// Check if user has permission to manage files in a project
    /// For MVP: any project member can delete attachments
    async fn check_project_permission(
        &self,
        project_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Check if user is a member of the project
        let is_member = self
            .member_repo
            .is_member(project_id, current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if is_member {
            return Ok(());
        }

        // Check if the project is public - public projects allow members to manage
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Project {} not found", project_id))
            })?;

        if project.is_public {
            // For public projects, any logged-in user can delete attachments
            // (This is a simplified permission model for MVP)
            return Ok(());
        }

        Err(ApplicationError::Forbidden(
            "You don't have permission to delete this attachment".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_error_not_found() {
        let err = ApplicationError::NotFound("Attachment 123 not found".to_string());
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_application_error_forbidden() {
        let err = ApplicationError::Forbidden("Permission denied".to_string());
        assert!(err.to_string().contains("Permission denied"));
    }
}
