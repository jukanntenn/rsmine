use crate::application::errors::ApplicationError;
use crate::application::use_cases::project::NamedId;
use crate::domain::repositories::{
    AttachmentRepository, IssueRepository, MemberRepository, ProjectRepository, UserRepository,
};
use crate::presentation::middleware::CurrentUser;
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

/// Request for updating an attachment
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAttachmentRequest {
    pub attachment: UpdateAttachmentDto,
}

/// DTO for attachment update fields
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAttachmentDto {
    /// Optional description for the attachment (max 255 characters)
    #[validate(length(max = 255, message = "Description must be at most 255 characters"))]
    pub description: Option<String>,
}

/// Response for attachment update
#[derive(Debug, Clone)]
pub struct UpdateAttachmentResponse {
    pub attachment: UpdateAttachmentDetailResponse,
}

/// Detailed attachment information after update
#[derive(Debug, Clone)]
pub struct UpdateAttachmentDetailResponse {
    pub id: i32,
    pub filename: String,
    pub filesize: i32,
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub content_url: String,
    pub author: NamedId,
    pub created_on: Option<String>,
}

/// Use case for updating attachment description
pub struct UpdateAttachmentUseCase<A, I, P, M, U>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    M: MemberRepository,
    U: UserRepository,
{
    attachment_repo: Arc<A>,
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    member_repo: Arc<M>,
    user_repo: Arc<U>,
    base_url: String,
}

impl<A, I, P, M, U> UpdateAttachmentUseCase<A, I, P, M, U>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    M: MemberRepository,
    U: UserRepository,
{
    pub fn new(
        attachment_repo: Arc<A>,
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        member_repo: Arc<M>,
        user_repo: Arc<U>,
        base_url: String,
    ) -> Self {
        Self {
            attachment_repo,
            issue_repo,
            project_repo,
            member_repo,
            user_repo,
            base_url,
        }
    }

    /// Execute the update use case
    ///
    /// Permission rules:
    /// - Admin can update all attachments
    /// - For Issue container: user must have manage_files permission on the issue's project
    /// - For Project container: user must have manage_files permission on the project
    /// - For MVP: member of the project can update attachments
    pub async fn execute(
        &self,
        attachment_id: i32,
        request: UpdateAttachmentDto,
        current_user: &CurrentUser,
    ) -> Result<UpdateAttachmentResponse, ApplicationError> {
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
        self.check_update_permission(&attachment, current_user)
            .await?;

        // 3. Update description
        let updated_attachment = self
            .attachment_repo
            .update_description(attachment_id, request.description)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 4. Get author info
        let author = self
            .user_repo
            .find_by_id(updated_attachment.author_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let author_info = author
            .map(|u| NamedId {
                id: u.id,
                name: format!("{} {}", u.firstname, u.lastname),
            })
            .unwrap_or(NamedId {
                id: updated_attachment.author_id,
                name: format!("User {}", updated_attachment.author_id),
            });

        // 5. Build content URL
        let content_url = format!(
            "{}/attachments/download/{}/{}",
            self.base_url,
            updated_attachment.id,
            urlencoding::encode(&updated_attachment.filename)
        );

        Ok(UpdateAttachmentResponse {
            attachment: UpdateAttachmentDetailResponse {
                id: updated_attachment.id,
                filename: updated_attachment.filename,
                filesize: updated_attachment.filesize,
                content_type: updated_attachment.content_type,
                description: updated_attachment.description,
                content_url,
                author: author_info,
                created_on: updated_attachment.created_on.map(|d| d.to_rfc3339()),
            },
        })
    }

    /// Check if the current user has permission to update this attachment
    async fn check_update_permission(
        &self,
        attachment: &crate::domain::entities::Attachment,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Admin can update all attachments
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
                // No container = no access
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
    /// For MVP: any project member can update attachments
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

        // Check if the project is public
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Project {} not found", project_id))
            })?;

        if project.is_public {
            // For public projects, any logged-in user can update attachments
            return Ok(());
        }

        Err(ApplicationError::Forbidden(
            "You don't have permission to update this attachment".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_attachment_dto_validation() {
        // Valid: description within limit
        let dto = UpdateAttachmentDto {
            description: Some("A valid description".to_string()),
        };
        assert!(dto.validate().is_ok());

        // Valid: no description
        let dto = UpdateAttachmentDto { description: None };
        assert!(dto.validate().is_ok());

        // Valid: description at exactly 255 characters
        let dto = UpdateAttachmentDto {
            description: Some("a".repeat(255)),
        };
        assert!(dto.validate().is_ok());

        // Invalid: description over 255 characters
        let dto = UpdateAttachmentDto {
            description: Some("a".repeat(256)),
        };
        assert!(dto.validate().is_err());
    }
}
