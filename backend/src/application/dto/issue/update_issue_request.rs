use chrono::NaiveDate;
use serde::Deserialize;

/// Request wrapper for updating an issue
#[derive(Debug, Deserialize)]
pub struct UpdateIssueRequest {
    pub issue: UpdateIssueDto,
}

/// DTO for updating an existing issue
#[derive(Debug, Deserialize)]
pub struct UpdateIssueDto {
    /// Subject (1-255 chars)
    pub subject: Option<String>,

    /// Description
    pub description: Option<String>,

    /// Status ID
    pub status_id: Option<i32>,

    /// Priority ID
    pub priority_id: Option<i32>,

    /// Tracker ID
    pub tracker_id: Option<i32>,

    /// Assignee ID (null to clear)
    pub assigned_to_id: Option<i32>,

    /// Category ID (null to clear)
    pub category_id: Option<i32>,

    /// Parent Issue ID (for subtasks, null to clear)
    pub parent_id: Option<i32>,

    /// Start date
    pub start_date: Option<NaiveDate>,

    /// Due date (null to clear)
    pub due_date: Option<NaiveDate>,

    /// Estimated hours (null to clear)
    pub estimated_hours: Option<f64>,

    /// Completion ratio (0-100)
    pub done_ratio: Option<i32>,

    /// Is private
    pub is_private: Option<bool>,

    /// Notes/comments to add
    pub notes: Option<String>,

    /// Notes are private (default: false)
    #[serde(default)]
    pub private_notes: bool,

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

impl UpdateIssueDto {
    /// Validate the subject if provided
    pub fn validate_subject(&self) -> Result<(), String> {
        if let Some(subject) = &self.subject {
            let subject = subject.trim();

            if subject.is_empty() {
                return Err("Subject cannot be blank".to_string());
            }

            if subject.len() > 255 {
                return Err("Subject is too long (maximum 255 characters)".to_string());
            }
        }

        Ok(())
    }

    /// Validate done_ratio if provided
    pub fn validate_done_ratio(&self) -> Result<(), String> {
        if let Some(done_ratio) = self.done_ratio {
            if !(0..=100).contains(&done_ratio) {
                return Err("Done ratio must be between 0 and 100".to_string());
            }
        }

        Ok(())
    }

    /// Validate all fields
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Err(e) = self.validate_subject() {
            errors.push(e);
        }

        if let Err(e) = self.validate_done_ratio() {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Check if there are any changes to apply
    pub fn has_changes(&self) -> bool {
        self.subject.is_some()
            || self.description.is_some()
            || self.status_id.is_some()
            || self.priority_id.is_some()
            || self.tracker_id.is_some()
            || self.assigned_to_id.is_some()
            || self.category_id.is_some()
            || self.parent_id.is_some()
            || self.start_date.is_some()
            || self.due_date.is_some()
            || self.estimated_hours.is_some()
            || self.done_ratio.is_some()
            || self.is_private.is_some()
            || self
                .notes
                .as_ref()
                .map(|n| !n.trim().is_empty())
                .unwrap_or(false)
            || self
                .uploads
                .as_ref()
                .map(|u| !u.is_empty())
                .unwrap_or(false)
    }
}
