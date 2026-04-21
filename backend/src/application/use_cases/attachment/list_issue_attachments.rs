use crate::application::errors::ApplicationError;
use crate::application::use_cases::project::NamedId;
use crate::domain::entities::Issue;
use crate::domain::repositories::{
    AttachmentRepository, IssueRepository, ProjectRepository, UserRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response for listing attachments of an issue
#[derive(Debug, Clone)]
pub struct IssueAttachmentListResponse {
    pub attachments: Vec<IssueAttachmentItem>,
}

/// Attachment item in the list response
#[derive(Debug, Clone)]
pub struct IssueAttachmentItem {
    pub id: i32,
    pub filename: String,
    pub filesize: i32,
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub author: NamedId,
    pub created_on: Option<String>,
}

/// Use case for listing attachments of an issue
pub struct ListIssueAttachmentsUseCase<A, I, P, U>
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
}

impl<A, I, P, U> ListIssueAttachmentsUseCase<A, I, P, U>
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
    ) -> Self {
        Self {
            attachment_repo,
            issue_repo,
            project_repo,
            user_repo,
        }
    }

    /// Execute the use case
    ///
    /// Visibility rules:
    /// - Admin users can see attachments of all issues
    /// - Regular users can see attachments of issues in public projects
    /// - Regular users can see attachments of issues in projects they are members of
    pub async fn execute(
        &self,
        issue_id: i32,
        current_user: &CurrentUser,
    ) -> Result<IssueAttachmentListResponse, ApplicationError> {
        // Get the issue
        let issue = self
            .issue_repo
            .find_by_id(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Issue {} not found", issue_id)))?;

        // Check visibility
        self.check_issue_visibility(&issue, current_user).await?;

        // Get attachments for the issue
        let attachments = self
            .attachment_repo
            .find_by_container(issue_id, "Issue")
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Build response items
        let mut items = Vec::new();
        for attachment in attachments {
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

            items.push(IssueAttachmentItem {
                id: attachment.id,
                filename: attachment.filename,
                filesize: attachment.filesize,
                content_type: attachment.content_type,
                description: attachment.description,
                author: author_info,
                created_on: attachment.created_on.map(|d| d.to_rfc3339()),
            });
        }

        Ok(IssueAttachmentListResponse { attachments: items })
    }

    /// Check if the current user can view the issue
    async fn check_issue_visibility(
        &self,
        issue: &Issue,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Admin can see all issues
        if current_user.admin {
            return Ok(());
        }

        // Get the project
        let project = self
            .project_repo
            .find_by_id(issue.project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Project {} not found", issue.project_id))
            })?;

        // Public projects are visible to all logged-in users
        if project.is_public {
            return Ok(());
        }

        // Check if user is a member of the project
        let member_project_ids = self
            .project_repo
            .find_project_ids_by_user_membership(current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if member_project_ids.contains(&issue.project_id) {
            return Ok(());
        }

        Err(ApplicationError::Forbidden(
            "You don't have permission to view this issue".into(),
        ))
    }
}
