use crate::application::errors::ApplicationError;
use crate::application::use_cases::project::NamedId;
use crate::config::StorageConfig;
use crate::domain::entities::Issue;
use crate::domain::repositories::{
    AttachmentRepository, IssueRepository, MemberRepository, NewAttachment, ProjectRepository,
    UserRepository,
};
use crate::infrastructure::storage::{LocalFileStorage, StorageError};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response for uploading an attachment to an issue
#[derive(Debug, Clone)]
pub struct UploadIssueAttachmentResponse {
    pub attachment: UploadedAttachmentItem,
}

/// Uploaded attachment item in the response
#[derive(Debug, Clone)]
pub struct UploadedAttachmentItem {
    pub id: i32,
    pub filename: String,
    pub filesize: i64,
    pub content_type: Option<String>,
    pub author: NamedId,
}

/// Use case for uploading an attachment directly to an issue
pub struct UploadIssueAttachmentUseCase<A, I, P, U, M>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
{
    attachment_repo: Arc<A>,
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    user_repo: Arc<U>,
    member_repo: Arc<M>,
    storage: Arc<LocalFileStorage>,
    config: StorageConfig,
}

impl<A, I, P, U, M> UploadIssueAttachmentUseCase<A, I, P, U, M>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
{
    pub fn new(
        attachment_repo: Arc<A>,
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        user_repo: Arc<U>,
        member_repo: Arc<M>,
        storage: Arc<LocalFileStorage>,
        config: StorageConfig,
    ) -> Self {
        Self {
            attachment_repo,
            issue_repo,
            project_repo,
            user_repo,
            member_repo,
            storage,
            config,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - User must have manage_files permission on the project
    /// - Admin can upload to any issue
    pub async fn execute(
        &self,
        issue_id: i32,
        filename: String,
        content: Vec<u8>,
        content_type: Option<String>,
        description: Option<String>,
        current_user: &CurrentUser,
    ) -> Result<UploadIssueAttachmentResponse, ApplicationError> {
        // 1. Validate file size
        if content.len() as u64 > self.config.max_file_size {
            return Err(ApplicationError::Validation(format!(
                "File size exceeds maximum of {} bytes",
                self.config.max_file_size
            )));
        }

        // 2. Get the issue
        let issue = self
            .issue_repo
            .find_by_id(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Issue {} not found", issue_id)))?;

        // 3. Check permission (manage_files)
        self.check_upload_permission(&issue, current_user).await?;

        // 4. Save file to storage
        let save_result = self
            .storage
            .save(&filename, &content)
            .map_err(|e| match e {
                StorageError::Io(io_err) => {
                    ApplicationError::Internal(format!("Failed to save file: {}", io_err))
                }
                StorageError::InvalidFilename(msg) => ApplicationError::Validation(msg),
                StorageError::NotFound(msg) => ApplicationError::Internal(msg),
            })?;

        // 5. Create attachment record
        let new_attachment = NewAttachment {
            container_id: Some(issue_id),
            container_type: Some("Issue".to_string()),
            filename: filename.clone(),
            disk_filename: save_result.disk_filename,
            filesize: content.len() as i64,
            content_type: content_type.clone(),
            digest: Some(save_result.digest),
            author_id: current_user.id,
            description,
            disk_directory: Some(save_result.disk_directory),
        };

        let attachment = self
            .attachment_repo
            .create(new_attachment)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 6. Get author info for response
        let author = self
            .user_repo
            .find_by_id(current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let author_info = author
            .map(|u| NamedId {
                id: u.id,
                name: format!("{} {}", u.firstname, u.lastname),
            })
            .unwrap_or(NamedId {
                id: current_user.id,
                name: current_user.login.clone(),
            });

        Ok(UploadIssueAttachmentResponse {
            attachment: UploadedAttachmentItem {
                id: attachment.id,
                filename,
                filesize: content.len() as i64,
                content_type,
                author: author_info,
            },
        })
    }

    /// Check if the current user has permission to upload attachments to this issue
    async fn check_upload_permission(
        &self,
        issue: &Issue,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Admin can upload to any issue
        if current_user.admin {
            return Ok(());
        }

        // Check if user is a member of the project
        let member = self
            .member_repo
            .find_by_project_and_user(issue.project_id, current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if member.is_none() {
            return Err(ApplicationError::Forbidden(
                "You don't have permission to upload attachments to this issue".into(),
            ));
        }

        // TODO: Check for manage_files permission in roles when permission system is fully implemented
        // For now, any member can upload attachments

        Ok(())
    }
}
