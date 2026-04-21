use serde::{Deserialize, Serialize};

/// Request for updating a role (wraps the role data)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    pub role: UpdateRoleDto,
}

/// DTO for updating an existing role
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleDto {
    /// Role name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of permission names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// Issue visibility scope: "all", "default", or "own"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_visibility: Option<String>,
    /// Whether this role can be assigned to members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignable: Option<bool>,
}

impl From<UpdateRoleDto> for crate::application::use_cases::UpdateRoleRequest {
    fn from(dto: UpdateRoleDto) -> Self {
        Self {
            name: dto.name,
            permissions: dto.permissions,
            issues_visibility: dto.issues_visibility,
            assignable: dto.assignable,
        }
    }
}
