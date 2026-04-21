use crate::application::errors::ApplicationError;
use crate::domain::repositories::{
    RoleRepository, ROLE_BUILTIN_ANONYMOUS, ROLE_BUILTIN_NON_MEMBER,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Use case for deleting a role
pub struct DeleteRoleUseCase<R: RoleRepository> {
    role_repo: Arc<R>,
}

impl<R: RoleRepository> DeleteRoleUseCase<R> {
    pub fn new(role_repo: Arc<R>) -> Self {
        Self { role_repo }
    }

    /// Execute the use case
    ///
    /// Only admin users can delete roles (permission checked at API layer)
    /// Cannot delete built-in roles (Non-Member, Anonymous)
    /// Cannot delete roles that are in use
    pub async fn execute(
        &self,
        id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Get existing role
        let existing = self
            .role_repo
            .find_by_id(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Role with id {} not found", id)))?;

        // Check if it's a built-in role
        if existing.builtin == ROLE_BUILTIN_NON_MEMBER || existing.builtin == ROLE_BUILTIN_ANONYMOUS
        {
            return Err(ApplicationError::Validation(
                "Cannot delete built-in roles (Non-Member, Anonymous)".into(),
            ));
        }

        // Check if role is in use
        if self
            .role_repo
            .is_in_use(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
        {
            return Err(ApplicationError::Validation(
                "Cannot delete role that is in use by project members".into(),
            ));
        }

        // Delete the role
        self.role_repo
            .delete(id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        tracing::info!(
            role_id = id,
            role_name = %existing.name,
            user_id = current_user.id,
            "Role deleted"
        );

        Ok(())
    }
}
