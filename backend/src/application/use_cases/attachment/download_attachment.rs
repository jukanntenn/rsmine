use crate::application::errors::ApplicationError;
use crate::domain::repositories::{AttachmentRepository, IssueRepository, ProjectRepository};
use crate::infrastructure::storage::{LocalFileStorage, StorageError};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response for a successful file download
#[derive(Debug, Clone)]
pub struct DownloadResponse {
    /// File content
    pub content: Vec<u8>,
    /// Original filename for download
    pub filename: String,
    /// MIME content type
    pub content_type: String,
}

/// Use case for downloading an attachment
pub struct DownloadAttachmentUseCase<A, I, P>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
{
    attachment_repo: Arc<A>,
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    storage: Arc<LocalFileStorage>,
}

impl<A, I, P> DownloadAttachmentUseCase<A, I, P>
where
    A: AttachmentRepository,
    I: IssueRepository,
    P: ProjectRepository,
{
    pub fn new(
        attachment_repo: Arc<A>,
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        storage: Arc<LocalFileStorage>,
    ) -> Self {
        Self {
            attachment_repo,
            issue_repo,
            project_repo,
            storage,
        }
    }

    /// Execute the download use case
    ///
    /// Permission rules:
    /// - Admin can download all attachments
    /// - For Issue container: user must have view_files permission on the issue's project
    /// - For Project container: user must have view_files permission on the project
    /// - Public projects: all logged-in users can download
    pub async fn execute(
        &self,
        attachment_id: i32,
        current_user: &CurrentUser,
    ) -> Result<DownloadResponse, ApplicationError> {
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
        self.check_download_permission(&attachment, current_user)
            .await?;

        // 3. Load file content
        let disk_directory = attachment.disk_directory.as_deref().unwrap_or("");
        let content = self
            .storage
            .load(disk_directory, &attachment.disk_filename)
            .map_err(|e| match e {
                StorageError::NotFound(_) => ApplicationError::NotFound(format!(
                    "Attachment file {} not found on disk",
                    attachment_id
                )),
                StorageError::Io(io_err) => ApplicationError::Internal(format!(
                    "Failed to read attachment file: {}",
                    io_err
                )),
                StorageError::InvalidFilename(msg) => ApplicationError::Internal(msg),
            })?;

        // 4. Increment download count
        self.attachment_repo
            .increment_downloads(attachment_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 5. Determine content type
        let content_type = attachment
            .content_type
            .clone()
            .unwrap_or_else(|| detect_content_type(&attachment.filename));

        Ok(DownloadResponse {
            content,
            filename: attachment.filename,
            content_type,
        })
    }

    /// Check if the current user has permission to download this attachment
    async fn check_download_permission(
        &self,
        attachment: &crate::domain::entities::Attachment,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Admin can download all attachments
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

    /// Check if user has view_files permission for an issue
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

    /// Check if user has access to a project
    async fn check_project_permission(
        &self,
        project_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Get the project
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Project {} not found", project_id))
            })?;

        // Public projects are accessible to all logged-in users
        if project.is_public {
            return Ok(());
        }

        // Check if user is a member of the project
        let member_project_ids = self
            .project_repo
            .find_project_ids_by_user_membership(current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if member_project_ids.contains(&project_id) {
            return Ok(());
        }

        Err(ApplicationError::Forbidden(
            "You don't have permission to download this attachment".into(),
        ))
    }
}

/// Detect content type from filename extension
fn detect_content_type(filename: &str) -> String {
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "bmp" => "image/bmp",
        "txt" => "text/plain",
        "csv" => "text/csv",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "rar" => "application/vnd.rar",
        "7z" => "application/x-7z-compressed",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_content_type_pdf() {
        assert_eq!(detect_content_type("document.pdf"), "application/pdf");
    }

    #[test]
    fn test_detect_content_type_image() {
        assert_eq!(detect_content_type("image.png"), "image/png");
        assert_eq!(detect_content_type("photo.jpg"), "image/jpeg");
        assert_eq!(detect_content_type("photo.jpeg"), "image/jpeg");
        assert_eq!(detect_content_type("animation.gif"), "image/gif");
    }

    #[test]
    fn test_detect_content_type_text() {
        assert_eq!(detect_content_type("notes.txt"), "text/plain");
        assert_eq!(detect_content_type("data.csv"), "text/csv");
    }

    #[test]
    fn test_detect_content_type_office() {
        assert_eq!(
            detect_content_type("document.docx"),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        );
        assert_eq!(
            detect_content_type("spreadsheet.xlsx"),
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        );
    }

    #[test]
    fn test_detect_content_type_unknown() {
        assert_eq!(
            detect_content_type("unknown.xyz"),
            "application/octet-stream"
        );
        assert_eq!(
            detect_content_type("noextension"),
            "application/octet-stream"
        );
    }
}
