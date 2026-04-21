use crate::application::use_cases::{
    CreateRoleResponse, GetRoleResponse, RoleDetail, RoleItem, RoleListResponse, UpdateRoleResponse,
};
use serde::{Deserialize, Serialize};

/// Role JSON response for list endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleJson {
    pub id: i32,
    pub name: String,
}

impl From<RoleItem> for RoleJson {
    fn from(item: RoleItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
        }
    }
}

/// Response for GET /api/v1/roles.json
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListJsonResponse {
    pub roles: Vec<RoleJson>,
}

impl From<RoleListResponse> for RoleListJsonResponse {
    fn from(response: RoleListResponse) -> Self {
        Self {
            roles: response.roles.into_iter().map(RoleJson::from).collect(),
        }
    }
}

/// Role detail JSON with permissions
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleDetailJson {
    pub id: i32,
    pub name: String,
    pub permissions: Vec<String>,
    pub issues_visibility: String,
}

impl From<RoleDetail> for RoleDetailJson {
    fn from(detail: RoleDetail) -> Self {
        Self {
            id: detail.id,
            name: detail.name,
            permissions: detail.permissions,
            issues_visibility: detail.issues_visibility,
        }
    }
}

/// Response for GET /api/v1/roles/:id.json
#[derive(Debug, Serialize, Deserialize)]
pub struct GetRoleJsonResponse {
    pub role: RoleDetailJson,
}

impl From<GetRoleResponse> for GetRoleJsonResponse {
    fn from(response: GetRoleResponse) -> Self {
        Self {
            role: RoleDetailJson::from(response.role),
        }
    }
}

/// Response for POST /api/v1/roles.json
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleJsonResponse {
    pub role: RoleDetailJson,
}

impl From<CreateRoleResponse> for CreateRoleJsonResponse {
    fn from(response: CreateRoleResponse) -> Self {
        Self {
            role: RoleDetailJson::from(response.role),
        }
    }
}

/// Response for PUT /api/v1/roles/:id.json
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleJsonResponse {
    pub role: RoleDetailJson,
}

impl From<UpdateRoleResponse> for UpdateRoleJsonResponse {
    fn from(response: UpdateRoleResponse) -> Self {
        Self {
            role: RoleDetailJson::from(response.role),
        }
    }
}
