use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Attachment entity representing a file attached to a container (issue, project, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: i32,
    pub container_id: Option<i32>,
    pub container_type: Option<String>,
    pub filename: String,
    pub disk_filename: String,
    pub filesize: i32,
    pub content_type: Option<String>,
    pub digest: Option<String>,
    pub downloads: i32,
    pub author_id: i32,
    pub created_on: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub disk_directory: Option<String>,
}

/// Temporary attachment token for uploads that haven't been attached to a container yet
/// These are created during file upload and later attached to issues/projects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempAttachment {
    /// Unique token for this temporary upload
    pub token: String,
    /// Original filename
    pub filename: String,
    /// Directory where file is stored (relative to base path)
    pub disk_directory: String,
    /// Filename on disk
    pub disk_filename: String,
    /// File size in bytes
    pub filesize: i64,
    /// MIME content type
    pub content_type: Option<String>,
    /// SHA256 digest of file content
    pub digest: Option<String>,
    /// ID of user who uploaded
    pub author_id: i32,
    /// When the upload was created
    pub created_at: DateTime<Utc>,
    /// When this token expires (24 hours default)
    pub expires_at: DateTime<Utc>,
}
