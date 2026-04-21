use serde::{Deserialize, Serialize};

/// Request for creating an issue status (wraps the status data)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIssueStatusRequest {
    pub issue_status: CreateIssueStatusDto,
}

/// DTO for creating a new issue status
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIssueStatusDto {
    /// Status name (required)
    pub name: String,
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

impl From<CreateIssueStatusDto> for crate::domain::repositories::NewIssueStatus {
    fn from(dto: CreateIssueStatusDto) -> Self {
        Self {
            name: dto.name,
            position: None,
            is_closed: dto.is_closed.unwrap_or(false),
            is_default: dto.is_default.unwrap_or(false),
            default_done_ratio: dto.default_done_ratio,
        }
    }
}
