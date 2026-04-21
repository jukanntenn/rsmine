use crate::application::errors::ApplicationError;
use crate::application::use_cases::project::NamedId;
use crate::domain::repositories::{
    AttachmentRepository, IssueRepository, ProjectRepository, UserRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response for getting attachment metadata
#[derive(Debug, Clone)]
pub struct GetAttachmentResponse {
    pub attachment: AttachmentDetailResponse,
}

/// Detailed attachment information
#[derive(Debug, Clone)]
pub struct AttachmentDetailResponse {
    pub id: i32,
    pub filename: String,
    pub filesize: i32,
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub content_url: String,
    pub thumbnail_url: Option<String>,
    pub author: NamedId,
    pub created_on: Option<String>,
}

/// Use case for getting attachment metadata
pub struct GetAttachmentUseCase<A, I, P, U>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
{
    attachment_repo: Arc<A>,
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    user_repo: Arc<U>,
    base_url: String,
}

impl<A, I, P, U> GetAttachmentUseCase<A, I, P, U>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
{
    pub fn new(
        attachment_repo: Arc<A>,
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        user_repo: Arc<U>,
        base_url: String,
    ) -> Self {
        Self {
            attachment_repo,
            issue_repo,
            project_repo,
            user_repo,
            base_url,
        }
    }

    /// Execute the use case
    ///
    /// Returns attachment metadata including download URLs.
    /// Uses the same permission check as download.
    pub async fn execute(
        &self,
        attachment_id: i32,
        current_user: &CurrentUser,
    ) -> Result<GetAttachmentResponse, ApplicationError> {
        // Get attachment
        let attachment = self
            .attachment_repo
            .find_by_id(attachment_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Attachment {} not found", attachment_id))
            })?;

        // Check permission (same as download)
        self.check_attachment_permission(&attachment, current_user)
            .await?;

        // Get author info
        let author = self
            .user_repo
            .find_by_id(attachment.author_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let author_info = author
            .map(|u| NamedId {
                id: u.id,
                name: format!("{} {}", u.firstname, u.lastname),
            })
            .unwrap_or(NamedId {
                id: attachment.author_id,
                name: format!("User {}", attachment.author_id),
            });

        // Build content URL
        let content_url = format!(
            "{}/attachments/download/{}/{}",
            self.base_url,
            attachment.id,
            urlencoding::encode(&attachment.filename)
        );

        // Build thumbnail URL (only for images)
        let thumbnail_url = if is_image_content_type(attachment.content_type.as_deref()) {
            Some(format!(
                "{}/attachments/thumbnail/{}",
                self.base_url, attachment.id
            ))
        } else {
            None
        };

        Ok(GetAttachmentResponse {
            attachment: AttachmentDetailResponse {
                id: attachment.id,
                filename: attachment.filename,
                filesize: attachment.filesize,
                content_type: attachment.content_type,
                description: attachment.description,
                content_url,
                thumbnail_url,
                author: author_info,
                created_on: attachment.created_on.map(|d| d.to_rfc3339()),
            },
        })
    }

    /// Check if user has permission to view this attachment
    async fn check_attachment_permission(
        &self,
        attachment: &crate::domain::entities::Attachment,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Admin can view all attachments
        if current_user.admin {
            return Ok(());
        }

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
            _ => Err(ApplicationError::Forbidden(
                "Attachment has no associated container".into(),
            )),
        }
    }

    async fn check_issue_permission(
        &self,
        issue_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        let issue = self
            .issue_repo
            .find_by_id(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Issue {} not found", issue_id)))?;

        self.check_project_permission(issue.project_id, current_user)
            .await
    }

    async fn check_project_permission(
        &self,
        project_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Project {} not found", project_id))
            })?;

        if project.is_public {
            return Ok(());
        }

        let member_project_ids = self
            .project_repo
            .find_project_ids_by_user_membership(current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if member_project_ids.contains(&project_id) {
            return Ok(());
        }

        Err(ApplicationError::Forbidden(
            "You don't have permission to view this attachment".into(),
        ))
    }
}

/// Check if content type is an image type that supports thumbnails
fn is_image_content_type(content_type: Option<&str>) -> bool {
    match content_type {
        Some(ct) => {
            ct.starts_with("image/") && 
            !ct.contains("svg") && // SVG doesn't support thumbnail generation
            ct != "image/x-icon"
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_image_content_type() {
        assert!(is_image_content_type(Some("image/png")));
        assert!(is_image_content_type(Some("image/jpeg")));
        assert!(is_image_content_type(Some("image/gif")));
        assert!(is_image_content_type(Some("image/webp")));
        assert!(!is_image_content_type(Some("image/svg+xml")));
        assert!(!is_image_content_type(Some("image/x-icon")));
        assert!(!is_image_content_type(Some("application/pdf")));
        assert!(!is_image_content_type(None));
    }
}
