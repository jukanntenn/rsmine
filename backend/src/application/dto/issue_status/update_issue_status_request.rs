use serde::{Deserialize, Serialize};

/// Request for updating an issue status (wraps the status data)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIssueStatusRequest {
    pub issue_status: UpdateIssueStatusDto,
}

/// DTO for updating an existing issue status
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIssueStatusDto {
    /// Status name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Is this a closed status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    /// Is this the default status for new issues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Default done ratio for issues with this status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_done_ratio: Option<i32>,
}
