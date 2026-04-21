use super::RepositoryError;
use crate::domain::entities::MemberWithRoles;
use async_trait::async_trait;

/// Member entity for creating a new member
#[derive(Debug, Clone)]
pub struct NewMember {
    pub user_id: i32,
    pub project_id: i32,
    pub mail_notification: bool,
}

#[async_trait]
pub trait MemberRepository: Send + Sync {
    /// Find all members of a project with their roles
    async fn find_by_project(
        &self,
        project_id: i32,
    ) -> Result<Vec<MemberWithRoles>, RepositoryError>;

    /// Find a single member by membership ID with their roles
    async fn find_by_id(&self, member_id: i32) -> Result<Option<MemberWithRoles>, RepositoryError>;

    /// Find a member by project and user ID with their roles
    async fn find_by_project_and_user(
        &self,
        project_id: i32,
        user_id: i32,
    ) -> Result<Option<MemberWithRoles>, RepositoryError>;

    /// Delete all memberships for a user
    async fn delete_by_user_id(&self, user_id: i32) -> Result<(), RepositoryError>;

    /// Check if a user is a member of a project
    async fn is_member(&self, project_id: i32, user_id: i32) -> Result<bool, RepositoryError>;

    /// Add a new member to a project
    async fn add_member(&self, member: NewMember) -> Result<i32, RepositoryError>;

    /// Add a role to a member
    async fn add_member_role(
        &self,
        member_id: i32,
        role_id: i32,
        inherited_from: Option<i32>,
    ) -> Result<(), RepositoryError>;

    /// Clear all roles for a member (delete from member_roles table)
    async fn clear_roles(&self, member_id: i32) -> Result<(), RepositoryError>;

    /// Delete a membership by ID
    async fn delete_by_id(&self, member_id: i32) -> Result<(), RepositoryError>;

    /// Add a user as manager to a project (creates member with manager role)
    async fn add_manager(&self, project_id: i32, user_id: i32) -> Result<(), RepositoryError>;

    /// Inherit members from a parent project
    async fn inherit_from_parent(
        &self,
        project_id: i32,
        parent_id: i32,
    ) -> Result<(), RepositoryError>;

    /// Delete all memberships for a project
    async fn delete_by_project(&self, project_id: i32) -> Result<(), RepositoryError>;
}
