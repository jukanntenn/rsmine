use crate::application::errors::ApplicationError;
use crate::domain::entities::Role;
use crate::domain::repositories::RoleRepository;
use std::sync::Arc;

/// Role item in list response (summary)
#[derive(Debug, Clone)]
pub struct RoleItem {
    pub id: i32,
    pub name: String,
}

impl From<Role> for RoleItem {
    fn from(role: Role) -> Self {
        Self {
            id: role.id,
            name: role.name,
        }
    }
}

/// Response for role list endpoint
#[derive(Debug, Clone)]
pub struct RoleListResponse {
    pub roles: Vec<RoleItem>,
}

/// Use case for listing all custom (non-built-in) roles
pub struct ListRolesUseCase<R: RoleRepository> {
    role_repo: Arc<R>,
}

impl<R: RoleRepository> ListRolesUseCase<R> {
    pub fn new(role_repo: Arc<R>) -> Self {
        Self { role_repo }
    }

    /// Execute the use case
    ///
    /// Returns all custom (non-built-in) roles for selection in UI.
    /// Any logged-in user can list roles.
    pub async fn execute(&self) -> Result<RoleListResponse, ApplicationError> {
        // Get custom roles (non-built-in)
        let roles = self
            .role_repo
            .find_custom()
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(RoleListResponse {
            roles: roles.into_iter().map(RoleItem::from).collect(),
        })
    }
}
