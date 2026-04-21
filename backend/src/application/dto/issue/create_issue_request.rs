use chrono::NaiveDate;
use serde::Deserialize;

/// Request wrapper for creating an issue
#[derive(Debug, Deserialize)]
pub struct CreateIssueRequest {
    pub issue: CreateIssueDto,
}

/// DTO for creating a new issue
#[derive(Debug, Deserialize)]
pub struct CreateIssueDto {
    /// Project ID (required)
    pub project_id: i32,

    /// Tracker ID (required)
    pub tracker_id: i32,

    /// Subject (required, 1-255 chars)
    pub subject: String,

    /// Description
    pub description: Option<String>,

    /// Status ID (default: tracker's default status)
    pub status_id: Option<i32>,

    /// Priority ID (default: default priority)
    pub priority_id: Option<i32>,

    /// Assignee ID
    pub assigned_to_id: Option<i32>,

    /// Category ID
    pub category_id: Option<i32>,

    /// Parent Issue ID (for subtasks)
    pub parent_id: Option<i32>,

    /// Start date
    pub start_date: Option<NaiveDate>,

    /// Due date
    pub due_date: Option<NaiveDate>,

    /// Estimated hours
    pub estimated_hours: Option<f64>,

    /// Is private (default: false)
    #[serde(default)]
    pub is_private: bool,

    /// Attachment tokens from upload API
    pub uploads: Option<Vec<UploadToken>>,
}

/// Upload token for attaching files
#[derive(Debug, Deserialize)]
pub struct UploadToken {
    /// Token from upload API
    pub token: String,

    /// Original filename
    pub filename: String,

    /// MIME type
    pub content_type: Option<String>,

    /// Description
    pub description: Option<String>,
}

impl CreateIssueDto {
    /// Validate the subject
    pub fn validate_subject(&self) -> Result<(), String> {
        let subject = self.subject.trim();

        if subject.is_empty() {
            return Err("Subject cannot be blank".to_string());
        }

        if subject.len() > 255 {
            return Err("Subject is too long (maximum 255 characters)".to_string());
        }

        Ok(())
    }

    /// Validate all required fields
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Err(e) = self.validate_subject() {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
