use crate::application::errors::ApplicationError;
use crate::domain::repositories::{IssueRepository, MemberRepository};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Use case for removing a member from a project
pub struct RemoveMemberUseCase<M: MemberRepository, I: IssueRepository> {
    member_repo: Arc<M>,
    issue_repo: Arc<I>,
}

impl<M, I> RemoveMemberUseCase<M, I>
where
    M: MemberRepository,
    I: IssueRepository,
{
    pub fn new(member_repo: Arc<M>, issue_repo: Arc<I>) -> Self {
        Self {
            member_repo,
            issue_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin: can remove any member
    /// - Non-admin: needs manage_members permission on the project
    ///
    /// Validation rules:
    /// - Membership must exist
    /// - Cannot remove inherited memberships directly (must remove from parent project)
    /// - Locked users' memberships can be removed
    ///
    /// Cascade behavior:
    /// - Issues assigned to this user in this project will have assignee cleared
    /// - Member roles are deleted automatically (cascade)
    pub async fn execute(
        &self,
        membership_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // 1. Get existing membership
        let membership = self
            .member_repo
            .find_by_id(membership_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Membership not found".into()))?;

        let project_id = membership.member.project_id;
        let user_id = membership.member.user_id;

        // 2. Check permission - must be admin or have manage_members permission on the project
        if !current_user.admin {
            let is_member = self
                .member_repo
                .is_member(project_id, current_user.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            if !is_member {
                return Err(ApplicationError::Forbidden(
                    "You don't have permission to manage members in this project".into(),
                ));
            }
        }

        // 3. Check for inherited membership
        // Cannot remove inherited memberships directly

        // Check if ALL roles are inherited (then this is purely inherited membership)
        let all_inherited =
            !membership.roles.is_empty() && membership.roles.iter().all(|r| r.is_inherited());

        if all_inherited {
            return Err(ApplicationError::Validation(
                "Cannot remove inherited membership. Remove from parent project.".into(),
            ));
        }

        // 4. Clear assignee on issues assigned to this user in this project
        self.issue_repo
            .clear_assignee_in_project(project_id, user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 5. Delete membership (cascade deletes member_roles)
        // Note: This will delete ALL member_roles including inherited ones
        // If there were inherited roles, they will also be removed
        self.member_repo
            .delete_by_id(membership_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{
        Issue, Member, MemberWithRoles, Project, Role, RoleWithInheritance, User,
        PROJECT_STATUS_ACTIVE, USER_STATUS_ACTIVE,
    };
    use crate::domain::repositories::RepositoryError;

    // Mock implementations for testing
    struct MockMemberRepository {
        members: std::sync::Mutex<Vec<MemberWithRoles>>,
    }

    impl MockMemberRepository {
        fn new() -> Self {
            Self {
                members: std::sync::Mutex::new(Vec::new()),
            }
        }

        fn with_member(member: MemberWithRoles) -> Self {
            Self {
                members: std::sync::Mutex::new(vec![member]),
            }
        }
    }

    #[async_trait::async_trait]
    impl MemberRepository for MockMemberRepository {
        async fn find_by_project(
            &self,
            _project_id: i32,
        ) -> Result<Vec<MemberWithRoles>, RepositoryError> {
            Ok(self.members.lock().unwrap().clone())
        }

        async fn find_by_id(
            &self,
            member_id: i32,
        ) -> Result<Option<MemberWithRoles>, RepositoryError> {
            Ok(self
                .members
                .lock()
                .unwrap()
                .iter()
                .find(|m| m.member.id == member_id)
                .cloned())
        }

        async fn find_by_project_and_user(
            &self,
            project_id: i32,
            user_id: i32,
        ) -> Result<Option<MemberWithRoles>, RepositoryError> {
            Ok(self
                .members
                .lock()
                .unwrap()
                .iter()
                .find(|m| m.member.project_id == project_id && m.member.user_id == user_id)
                .cloned())
        }

        async fn delete_by_user_id(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn is_member(&self, project_id: i32, user_id: i32) -> Result<bool, RepositoryError> {
            Ok(self
                .members
                .lock()
                .unwrap()
                .iter()
                .any(|m| m.member.project_id == project_id && m.member.user_id == user_id))
        }

        async fn add_member(
            &self,
            _member: crate::domain::repositories::NewMember,
        ) -> Result<i32, RepositoryError> {
            Ok(1)
        }

        async fn add_member_role(
            &self,
            _member_id: i32,
            _role_id: i32,
            _inherited_from: Option<i32>,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn clear_roles(&self, _member_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn delete_by_id(&self, member_id: i32) -> Result<(), RepositoryError> {
            let mut members = self.members.lock().unwrap();
            members.retain(|m| m.member.id != member_id);
            Ok(())
        }

        async fn add_manager(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn inherit_from_parent(
            &self,
            _project_id: i32,
            _parent_id: i32,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn delete_by_project(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockIssueRepository;

    #[async_trait::async_trait]
    impl IssueRepository for MockIssueRepository {
        async fn find_all(
            &self,
            _params: crate::domain::value_objects::IssueQueryParams,
        ) -> Result<Vec<Issue>, RepositoryError> {
            Ok(Vec::new())
        }

        async fn count(
            &self,
            _params: &crate::domain::value_objects::IssueQueryParams,
        ) -> Result<u32, RepositoryError> {
            Ok(0)
        }

        async fn find_by_project(&self, _project_id: i32) -> Result<Vec<Issue>, RepositoryError> {
            Ok(Vec::new())
        }

        async fn delete_by_project(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn find_by_id(&self, _id: i32) -> Result<Option<Issue>, RepositoryError> {
            Ok(None)
        }

        async fn clear_assignee_in_project(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn create(
            &self,
            _issue: crate::domain::repositories::NewIssue,
        ) -> Result<Issue, RepositoryError> {
            unimplemented!()
        }

        async fn update(
            &self,
            _id: i32,
            _update: crate::domain::repositories::IssueUpdate,
        ) -> Result<Issue, RepositoryError> {
            unimplemented!()
        }

        async fn find_children(&self, _parent_id: i32) -> Result<Vec<Issue>, RepositoryError> {
            Ok(Vec::new())
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    fn create_test_user(id: i32, admin: bool, status: i32) -> User {
        User {
            id,
            login: format!("user{}", id),
            hashed_password: None,
            firstname: format!("First{}", id),
            lastname: format!("Last{}", id),
            admin,
            status,
            last_login_on: None,
            language: None,
            auth_source_id: None,
            created_on: None,
            updated_on: None,
            r#type: None,
            mail_notification: "all".to_string(),
            salt: None,
            must_change_passwd: false,
            passwd_changed_on: None,
            twofa_scheme: None,
            twofa_totp_key: None,
            twofa_totp_last_used_at: None,
            twofa_required: false,
        }
    }

    fn create_test_member(id: i32, user_id: i32, project_id: i32) -> Member {
        Member {
            id,
            user_id,
            project_id,
            created_on: None,
            mail_notification: false,
        }
    }

    fn create_test_role(id: i32, name: &str) -> Role {
        Role {
            id,
            name: name.to_string(),
            position: Some(id),
            assignable: true,
            builtin: 0,
            permissions: None,
            issues_visibility: "default".to_string(),
            users_visibility: "members_of_visible_projects".to_string(),
            time_entries_visibility: "all".to_string(),
            all_roles_managed: false,
            settings: None,
            default_time_entry_activity_id: None,
        }
    }

    fn create_test_role_with_inheritance(
        role: Role,
        inherited_from: Option<i32>,
    ) -> RoleWithInheritance {
        RoleWithInheritance {
            role,
            inherited_from,
        }
    }

    #[tokio::test]
    async fn test_remove_member_as_admin() {
        let member = create_test_member(1, 2, 1);
        let user = create_test_user(2, false, USER_STATUS_ACTIVE);
        let role = create_test_role(1, "Developer");

        let member_repo = Arc::new(MockMemberRepository::with_member(MemberWithRoles {
            member,
            user,
            roles: vec![create_test_role_with_inheritance(role, None)],
        }));
        let issue_repo = Arc::new(MockIssueRepository);

        let usecase = RemoveMemberUseCase::new(member_repo.clone(), issue_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_ok());

        // Verify member was deleted
        let members = member_repo.find_by_project(1).await.unwrap();
        assert!(members.is_empty());
    }

    #[tokio::test]
    async fn test_remove_member_not_found() {
        let member_repo = Arc::new(MockMemberRepository::new());
        let issue_repo = Arc::new(MockIssueRepository);

        let usecase = RemoveMemberUseCase::new(member_repo, issue_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(999, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_remove_inherited_membership_fails() {
        let member = create_test_member(1, 2, 1);
        let user = create_test_user(2, false, USER_STATUS_ACTIVE);
        let role = create_test_role(1, "Developer");

        // Create membership with inherited role
        let member_repo = Arc::new(MockMemberRepository::with_member(MemberWithRoles {
            member,
            user,
            roles: vec![create_test_role_with_inheritance(role, Some(100))], // inherited_from = 100
        }));
        let issue_repo = Arc::new(MockIssueRepository);

        let usecase = RemoveMemberUseCase::new(member_repo, issue_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_remove_locked_member_should_be_allowed() {
        let member = create_test_member(1, 2, 1);
        let locked_user = User {
            id: 2,
            login: "locked".to_string(),
            status: 3, // Locked status
            ..create_test_user(2, false, USER_STATUS_ACTIVE)
        };
        let role = create_test_role(1, "Developer");

        let member_repo = Arc::new(MockMemberRepository::with_member(MemberWithRoles {
            member,
            user: locked_user,
            roles: vec![create_test_role_with_inheritance(role, None)],
        }));
        let issue_repo = Arc::new(MockIssueRepository);

        let usecase = RemoveMemberUseCase::new(member_repo.clone(), issue_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_ok());

        // Verify member was deleted
        let members = member_repo.find_by_project(1).await.unwrap();
        assert!(members.is_empty());
    }

    #[tokio::test]
    async fn test_remove_member_non_admin_no_permission() {
        let member = create_test_member(1, 3, 1);
        let user = create_test_user(3, false, USER_STATUS_ACTIVE);
        let role = create_test_role(1, "Developer");

        let member_repo = Arc::new(MockMemberRepository::with_member(MemberWithRoles {
            member,
            user,
            roles: vec![create_test_role_with_inheritance(role, None)],
        }));
        let issue_repo = Arc::new(MockIssueRepository);

        let usecase = RemoveMemberUseCase::new(member_repo, issue_repo);

        // Non-admin user who is NOT a member of the project
        let current_user = CurrentUser {
            id: 2,
            login: "nonmember".to_string(),
            admin: false,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_remove_member_non_admin_with_permission() {
        // Create a member that will be deleted
        let target_member = create_test_member(1, 3, 1);
        let target_user = create_test_user(3, false, USER_STATUS_ACTIVE);
        let role = create_test_role(1, "Developer");

        // Create a current user who is also a member (has permission)
        let current_user_member = create_test_member(2, 2, 1);
        let current_user_entity = create_test_user(2, false, USER_STATUS_ACTIVE);

        let member_repo = Arc::new(MockMemberRepository {
            members: std::sync::Mutex::new(vec![
                MemberWithRoles {
                    member: target_member,
                    user: target_user,
                    roles: vec![create_test_role_with_inheritance(role, None)],
                },
                MemberWithRoles {
                    member: current_user_member,
                    user: current_user_entity,
                    roles: vec![create_test_role_with_inheritance(
                        create_test_role(3, "Manager"),
                        None,
                    )],
                },
            ]),
        });
        let issue_repo = Arc::new(MockIssueRepository);

        let usecase = RemoveMemberUseCase::new(member_repo.clone(), issue_repo);

        // Non-admin user who IS a member of the project
        let current_user = CurrentUser {
            id: 2,
            login: "manager".to_string(),
            admin: false,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_ok());
    }
}
