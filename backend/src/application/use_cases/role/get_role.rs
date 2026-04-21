use crate::application::errors::ApplicationError;
use crate::domain::entities::Role;
use crate::domain::repositories::RoleRepository;
use std::sync::Arc;

/// Role detail with permissions
#[derive(Debug, Clone)]
pub struct RoleDetail {
    pub id: i32,
    pub name: String,
    pub permissions: Vec<String>,
    pub issues_visibility: String,
}

impl RoleDetail {
    pub fn from_role(role: &Role) -> Self {
        let permissions: Vec<String> = role
            .permissions
            .as_ref()
            .and_then(|p| serde_json::from_str(p).ok())
            .unwrap_or_default();

        Self {
            id: role.id,
            name: role.name.clone(),
            permissions,
            issues_visibility: role.issues_visibility.clone(),
        }
    }
}

/// Response for get role endpoint
#[derive(Debug, Clone)]
pub struct GetRoleResponse {
    pub role: RoleDetail,
}

/// Use case for getting a single role by ID
pub struct GetRoleUseCase<R: RoleRepository> {
    role_repo: Arc<R>,
}

impl<R: RoleRepository> GetRoleUseCase<R> {
    pub fn new(role_repo: Arc<R>) -> Self {
        Self { role_repo }
    }

    /// Execute the use case
    pub async fn execute(&self, id: i32) -> Result<GetRoleResponse, ApplicationError> {
        // Get role by ID
        let role = self
            .role_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Role with id {} not found", id)))?;

        Ok(GetRoleResponse {
            role: RoleDetail::from_role(&role),
        })
    }
}
