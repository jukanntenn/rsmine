use super::get_role::RoleDetail;
use crate::application::errors::ApplicationError;
use crate::domain::repositories::ROLE_BUILTIN_CUSTOM;
use crate::domain::repositories::{NewRole, RoleRepository};
use std::sync::Arc;

/// Request for creating a new role
#[derive(Debug, Clone)]
pub struct CreateRoleRequest {
    pub name: String,
    pub permissions: Option<Vec<String>>,
    pub issues_visibility: Option<String>,
    pub assignable: Option<bool>,
}

/// Response for create role endpoint
#[derive(Debug, Clone)]
pub struct CreateRoleResponse {
    pub role: RoleDetail,
}

/// Use case for creating a new role
pub struct CreateRoleUseCase<R: RoleRepository> {
    role_repo: Arc<R>,
}

impl<R: RoleRepository> CreateRoleUseCase<R> {
    pub fn new(role_repo: Arc<R>) -> Self {
        Self { role_repo }
    }

    /// Execute the use case
    ///
    /// Only admin users can create roles (permission checked at API layer)
    pub async fn execute(
        &self,
        request: CreateRoleRequest,
    ) -> Result<CreateRoleResponse, ApplicationError> {
        // Validate name is not empty
        if request.name.trim().is_empty() {
            return Err(ApplicationError::Validation(
                "Role name cannot be empty".into(),
            ));
        }

        // Check if role with same name already exists
        if self
            .role_repo
            .exists_by_name(&request.name)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
        {
            return Err(ApplicationError::AlreadyExists(format!(
                "Role with name '{}' already exists",
                request.name
            )));
        }

        // Validate issues_visibility value
        let issues_visibility = request
            .issues_visibility
            .unwrap_or_else(|| "default".to_string());
        let valid_visibilities = ["all", "default", "own"];
        if !valid_visibilities.contains(&issues_visibility.as_str()) {
            return Err(ApplicationError::Validation(format!(
                "Invalid issues_visibility '{}'. Must be one of: all, default, own",
                issues_visibility
            )));
        }

        // Create new role
        let new_role = NewRole {
            name: request.name.trim().to_string(),
            position: None,
            assignable: request.assignable.unwrap_or(true),
            builtin: ROLE_BUILTIN_CUSTOM,
            permissions: request.permissions,
            issues_visibility,
            users_visibility: "all".to_string(),
            time_entries_visibility: "all".to_string(),
            all_roles_managed: false,
        };

        let role = self
            .role_repo
            .create(&new_role)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(CreateRoleResponse {
            role: RoleDetail::from_role(&role),
        })
    }
}
