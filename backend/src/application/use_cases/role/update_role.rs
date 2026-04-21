use super::get_role::RoleDetail;
use crate::application::errors::ApplicationError;
use crate::domain::entities::Role;
use crate::domain::repositories::RoleRepository;
use std::sync::Arc;

/// Request for updating a role
#[derive(Debug, Clone)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub issues_visibility: Option<String>,
    pub assignable: Option<bool>,
}

/// Response for update role endpoint
#[derive(Debug, Clone)]
pub struct UpdateRoleResponse {
    pub role: RoleDetail,
}

/// Use case for updating an existing role
pub struct UpdateRoleUseCase<R: RoleRepository> {
    role_repo: Arc<R>,
}

impl<R: RoleRepository> UpdateRoleUseCase<R> {
    pub fn new(role_repo: Arc<R>) -> Self {
        Self { role_repo }
    }

    /// Execute the use case
    ///
    /// Only admin users can update roles (permission checked at API layer)
    pub async fn execute(
        &self,
        id: i32,
        request: UpdateRoleRequest,
    ) -> Result<UpdateRoleResponse, ApplicationError> {
        // Get existing role
        let existing = self
            .role_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Role with id {} not found", id)))?;

        // Validate name if provided
        let name = match &request.name {
            Some(name) => {
                if name.trim().is_empty() {
                    return Err(ApplicationError::Validation(
                        "Role name cannot be empty".into(),
                    ));
                }

                // Check if another role with same name exists
                if self
                    .role_repo
                    .exists_by_name_excluding(name, id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?
                {
                    return Err(ApplicationError::AlreadyExists(format!(
                        "Role with name '{}' already exists",
                        name
                    )));
                }
                name.trim().to_string()
            }
            None => existing.name.clone(),
        };

        // Validate issues_visibility if provided
        let issues_visibility = match &request.issues_visibility {
            Some(vis) => {
                let valid_visibilities = ["all", "default", "own"];
                if !valid_visibilities.contains(&vis.as_str()) {
                    return Err(ApplicationError::Validation(format!(
                        "Invalid issues_visibility '{}'. Must be one of: all, default, own",
                        vis
                    )));
                }
                vis.clone()
            }
            None => existing.issues_visibility.clone(),
        };

        // Update permissions
        let permissions = match &request.permissions {
            Some(perms) => Some(serde_json::to_string(perms).unwrap_or_else(|_| "[]".to_string())),
            None => existing.permissions.clone(),
        };

        // Build updated role
        let updated_role = Role {
            id: existing.id,
            name,
            position: existing.position,
            assignable: request.assignable.unwrap_or(existing.assignable),
            builtin: existing.builtin,
            permissions,
            issues_visibility,
            users_visibility: existing.users_visibility,
            time_entries_visibility: existing.time_entries_visibility,
            all_roles_managed: existing.all_roles_managed,
            settings: existing.settings,
            default_time_entry_activity_id: existing.default_time_entry_activity_id,
        };

        let role = self
            .role_repo
            .update(&updated_role)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(UpdateRoleResponse {
            role: RoleDetail::from_role(&role),
        })
    }
}
