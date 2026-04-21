use serde::{Deserialize, Serialize};

/// Request for creating a tracker (wraps the tracker data)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTrackerRequest {
    pub tracker: CreateTrackerDto,
}

/// DTO for creating a new tracker
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTrackerDto {
    /// Tracker name (required)
    pub name: String,
    /// Default status ID for issues with this tracker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_status_id: Option<i32>,
    /// Description (not supported in current schema, but included for API compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of enabled standard fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_standard_fields: Option<Vec<String>>,
}

impl From<CreateTrackerDto> for crate::application::use_cases::CreateTrackerRequest {
    fn from(dto: CreateTrackerDto) -> Self {
        Self {
            name: dto.name,
            default_status_id: dto.default_status_id,
            description: dto.description,
            enabled_standard_fields: dto.enabled_standard_fields,
        }
    }
}
