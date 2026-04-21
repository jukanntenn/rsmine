use super::RepositoryError;
use crate::domain::entities::Role;
use async_trait::async_trait;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    /// Find a role by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<Role>, RepositoryError>;

    /// Find all roles
    async fn find_all(&self) -> Result<Vec<Role>, RepositoryError>;

    /// Find custom (non-built-in) roles
    /// Built-in roles have builtin > 0 (Non-Member = 1, Anonymous = 2)
    async fn find_custom(&self) -> Result<Vec<Role>, RepositoryError>;

    /// Find roles that can be managed by a user (based on the user's role's all_roles_managed flag)
    /// Returns all roles if the user has a role with all_roles_managed = true
    async fn find_managed_by_user(&self, user_id: i32) -> Result<Vec<Role>, RepositoryError>;

    /// Check if a role is managed by a user
    async fn is_role_managed_by_user(
        &self,
        user_id: i32,
        role_id: i32,
    ) -> Result<bool, RepositoryError>;

    /// Find roles by IDs
    async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<Role>, RepositoryError>;

    /// Create a new role
    async fn create(&self, role: &NewRole) -> Result<Role, RepositoryError>;

    /// Update an existing role
    async fn update(&self, role: &Role) -> Result<Role, RepositoryError>;

    /// Delete a role by ID
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;

    /// Check if a role with the given name exists
    async fn exists_by_name(&self, name: &str) -> Result<bool, RepositoryError>;

    /// Check if a role with the given name exists (excluding a specific ID)
    async fn exists_by_name_excluding(
        &self,
        name: &str,
        exclude_id: i32,
    ) -> Result<bool, RepositoryError>;

    /// Check if a role is in use (has member_roles associations)
    async fn is_in_use(&self, id: i32) -> Result<bool, RepositoryError>;
}

/// Data for creating a new role
#[derive(Debug, Clone)]
pub struct NewRole {
    pub name: String,
    pub position: Option<i32>,
    pub assignable: bool,
    pub builtin: i32,
    pub permissions: Option<Vec<String>>,
    pub issues_visibility: String,
    pub users_visibility: String,
    pub time_entries_visibility: String,
    pub all_roles_managed: bool,
}

/// Built-in role constants
pub const ROLE_BUILTIN_CUSTOM: i32 = 0;
pub const ROLE_BUILTIN_NON_MEMBER: i32 = 1;
pub const ROLE_BUILTIN_ANONYMOUS: i32 = 2;
