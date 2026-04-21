use serde::{Deserialize, Serialize};

/// Request for updating a tracker (wraps the tracker data)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTrackerRequest {
    pub tracker: UpdateTrackerDto,
}

/// DTO for updating an existing tracker
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTrackerDto {
    /// Tracker name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Position for ordering trackers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// Whether this tracker is shown in roadmap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_in_roadmap: Option<bool>,
    /// Default status ID for issues with this tracker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_status_id: Option<i32>,
    /// List of enabled standard fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_standard_fields: Option<Vec<String>>,
    /// Project IDs to associate with this tracker (replaces existing associations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i32>>,
}

impl From<UpdateTrackerDto> for crate::application::use_cases::UpdateTrackerRequest {
    fn from(dto: UpdateTrackerDto) -> Self {
        Self {
            name: dto.name,
            position: dto.position,
            is_in_roadmap: dto.is_in_roadmap,
            default_status_id: dto.default_status_id,
            enabled_standard_fields: dto.enabled_standard_fields,
            project_ids: dto.project_ids,
        }
    }
}
