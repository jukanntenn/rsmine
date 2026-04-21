use serde::{Deserialize, Serialize};

/// Request for creating a role (wraps the role data)
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub role: CreateRoleDto,
}

/// DTO for creating a new role
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleDto {
    /// Role name (required)
    pub name: String,
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

impl From<CreateRoleDto> for crate::application::use_cases::CreateRoleRequest {
    fn from(dto: CreateRoleDto) -> Self {
        Self {
            name: dto.name,
            permissions: dto.permissions,
            issues_visibility: dto.issues_visibility,
            assignable: dto.assignable,
        }
    }
}
