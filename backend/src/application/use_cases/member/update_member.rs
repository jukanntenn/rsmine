use super::{MemberNamedId, MembershipResponse};
use crate::application::dto::UpdateMemberDto;
use crate::application::errors::ApplicationError;
use crate::domain::repositories::{MemberRepository, ProjectRepository, RoleRepository};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Use case for updating a member's roles
pub struct UpdateMemberUseCase<M: MemberRepository, P: ProjectRepository, R: RoleRepository> {
    member_repo: Arc<M>,
    project_repo: Arc<P>,
    role_repo: Arc<R>,
}

impl<M, P, R> UpdateMemberUseCase<M, P, R>
where
    M: MemberRepository,
    P: ProjectRepository,
    R: RoleRepository,
{
    pub fn new(member_repo: Arc<M>, project_repo: Arc<P>, role_repo: Arc<R>) -> Self {
        Self {
            member_repo,
            project_repo,
            role_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin: can update any member with any role
    /// - Non-admin: needs manage_members permission on the project
    /// - Non-admin: can only assign roles they can manage
    ///
    /// Validation rules:
    /// - Membership must exist
    /// - At least one role must be specified
    /// - All roles must exist
    /// - Cannot modify inherited roles directly (only non-inherited roles are updated)
    ///
    /// Locked users can still have their membership updated
    pub async fn execute(
        &self,
        membership_id: i32,
        dto: UpdateMemberDto,
        current_user: &CurrentUser,
    ) -> Result<MembershipResponse, ApplicationError> {
        // 1. Validate the DTO
        if let Err(errors) = dto.validate() {
            return Err(ApplicationError::Validation(errors.join(", ")));
        }

        // 2. Get existing membership
        let membership = self
            .member_repo
            .find_by_id(membership_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Membership not found".into()))?;

        let project_id = membership.member.project_id;

        // 3. Check permission - must be admin or have manage_members permission on the project
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

        // 4. Validate roles and check permissions
        let roles = self
            .role_repo
            .find_by_ids(&dto.role_ids)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Check if all requested roles exist
        if roles.len() != dto.role_ids.len() {
            let found_ids: Vec<i32> = roles.iter().map(|r| r.id).collect();
            let missing: Vec<i32> = dto
                .role_ids
                .iter()
                .filter(|id| !found_ids.contains(id))
                .copied()
                .collect();
            return Err(ApplicationError::Validation(format!(
                "Role(s) not found: {:?}",
                missing
            )));
        }

        // 5. Check role assignment permissions for non-admins
        if !current_user.admin {
            let manageable_roles = self
                .role_repo
                .find_managed_by_user(current_user.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            let manageable_ids: Vec<i32> = manageable_roles.iter().map(|r| r.id).collect();

            for role_id in &dto.role_ids {
                if !manageable_ids.contains(role_id) {
                    return Err(ApplicationError::Forbidden(
                        "You don't have permission to assign one or more of the specified roles"
                            .into(),
                    ));
                }
            }
        }

        // 6. Clear existing non-inherited roles
        self.member_repo
            .clear_roles(membership_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 7. Add new roles
        for role_id in &dto.role_ids {
            self.member_repo
                .add_member_role(membership_id, *role_id, None)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
        }

        // 8. Get project info for response
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Project not found".into()))?;

        // 9. Return response
        Ok(MembershipResponse {
            id: membership.member.id,
            project: MemberNamedId {
                id: project.id,
                name: project.name,
            },
            user: MemberNamedId {
                id: membership.user.id,
                name: membership.user.full_name(),
            },
            roles: roles
                .into_iter()
                .map(|r| MemberNamedId {
                    id: r.id,
                    name: r.name,
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{
        Member, MemberWithRoles, Project, Role, RoleWithInheritance, User, PROJECT_STATUS_ACTIVE,
        USER_STATUS_ACTIVE,
    };
    use crate::domain::repositories::{NewRole, RepositoryError};
    use std::sync::atomic::{AtomicI32, Ordering};

    // Mock implementations for testing
    struct MockMemberRepository {
        members: std::sync::Mutex<Vec<MemberWithRoles>>,
        next_id: AtomicI32,
    }

    impl MockMemberRepository {
        fn new() -> Self {
            Self {
                members: std::sync::Mutex::new(Vec::new()),
                next_id: AtomicI32::new(1),
            }
        }

        fn with_member(member: MemberWithRoles) -> Self {
            Self {
                members: std::sync::Mutex::new(vec![member]),
                next_id: AtomicI32::new(100),
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
                .find(|m| m.member.project_id == project_id && m.user.id == user_id)
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
                .any(|m| m.member.project_id == project_id && m.user.id == user_id))
        }

        async fn add_member(
            &self,
            _member: crate::domain::repositories::NewMember,
        ) -> Result<i32, RepositoryError> {
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            Ok(id)
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

        async fn delete_by_id(&self, _member_id: i32) -> Result<(), RepositoryError> {
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

    struct MockProjectRepository {
        projects: Vec<Project>,
    }

    #[async_trait::async_trait]
    impl ProjectRepository for MockProjectRepository {
        async fn find_all(
            &self,
            _params: crate::domain::repositories::ProjectQueryParams,
        ) -> Result<Vec<Project>, RepositoryError> {
            Ok(self.projects.clone())
        }

        async fn count(
            &self,
            _params: &crate::domain::repositories::ProjectQueryParams,
        ) -> Result<u32, RepositoryError> {
            Ok(self.projects.len() as u32)
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<Project>, RepositoryError> {
            Ok(self.projects.iter().find(|p| p.id == id).cloned())
        }

        async fn find_by_identifier(
            &self,
            _identifier: &str,
        ) -> Result<Option<Project>, RepositoryError> {
            Ok(None)
        }

        async fn find_visible_for_user(
            &self,
            _user_id: i32,
            _params: crate::domain::repositories::ProjectQueryParams,
        ) -> Result<Vec<Project>, RepositoryError> {
            Ok(self.projects.clone())
        }

        async fn count_visible_for_user(
            &self,
            _user_id: i32,
            _params: &crate::domain::repositories::ProjectQueryParams,
        ) -> Result<u32, RepositoryError> {
            Ok(self.projects.len() as u32)
        }

        async fn find_project_ids_by_user_membership(
            &self,
            _user_id: i32,
        ) -> Result<Vec<i32>, RepositoryError> {
            Ok(vec![])
        }

        async fn create(&self, _project: Project) -> Result<Project, RepositoryError> {
            unimplemented!()
        }

        async fn exists_by_identifier(&self, _identifier: &str) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn get_max_rgt(&self) -> Result<Option<i32>, RepositoryError> {
            Ok(None)
        }

        async fn add_tracker(
            &self,
            _project_id: i32,
            _tracker_id: i32,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn update_nested_set_for_insert(&self, _lft: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn update(&self, _project: Project) -> Result<Project, RepositoryError> {
            unimplemented!()
        }

        async fn set_trackers(
            &self,
            _project_id: i32,
            _tracker_ids: &[i32],
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn exists_by_identifier_excluding(
            &self,
            _identifier: &str,
            _exclude_project_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn find_children(&self, _project_id: i32) -> Result<Vec<Project>, RepositoryError> {
            Ok(vec![])
        }

        async fn delete(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn clear_trackers(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockRoleRepository {
        roles: Vec<Role>,
    }

    #[async_trait::async_trait]
    impl RoleRepository for MockRoleRepository {
        async fn find_by_id(&self, id: i32) -> Result<Option<Role>, RepositoryError> {
            Ok(self.roles.iter().find(|r| r.id == id).cloned())
        }

        async fn find_all(&self) -> Result<Vec<Role>, RepositoryError> {
            Ok(self.roles.clone())
        }

        async fn find_custom(&self) -> Result<Vec<Role>, RepositoryError> {
            Ok(self
                .roles
                .iter()
                .filter(|r| r.builtin == 0)
                .cloned()
                .collect())
        }

        async fn find_managed_by_user(&self, _user_id: i32) -> Result<Vec<Role>, RepositoryError> {
            // For testing, return all assignable roles
            Ok(self
                .roles
                .iter()
                .filter(|r| r.assignable)
                .cloned()
                .collect())
        }

        async fn is_role_managed_by_user(
            &self,
            user_id: i32,
            role_id: i32,
        ) -> Result<bool, RepositoryError> {
            let manageable = self.find_managed_by_user(user_id).await?;
            Ok(manageable.iter().any(|r| r.id == role_id))
        }

        async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<Role>, RepositoryError> {
            Ok(self
                .roles
                .iter()
                .filter(|r| ids.contains(&r.id))
                .cloned()
                .collect())
        }

        async fn create(&self, _role: &NewRole) -> Result<Role, RepositoryError> {
            Err(RepositoryError::Database("Not implemented".into()))
        }

        async fn update(&self, _role: &Role) -> Result<Role, RepositoryError> {
            Err(RepositoryError::Database("Not implemented".into()))
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            Err(RepositoryError::Database("Not implemented".into()))
        }

        async fn exists_by_name(&self, _name: &str) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn exists_by_name_excluding(
            &self,
            _name: &str,
            _exclude_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn is_in_use(&self, _id: i32) -> Result<bool, RepositoryError> {
            Ok(false)
        }
    }

    fn create_test_project(id: i32) -> Project {
        Project {
            id,
            name: format!("Project {}", id),
            description: None,
            homepage: None,
            is_public: true,
            parent_id: None,
            created_on: None,
            updated_on: None,
            identifier: Some(format!("project-{}", id)),
            status: PROJECT_STATUS_ACTIVE,
            lft: None,
            rgt: None,
            inherit_members: false,
            default_version_id: None,
            default_assigned_to_id: None,
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

    fn create_test_role(id: i32, name: &str, assignable: bool) -> Role {
        Role {
            id,
            name: name.to_string(),
            position: Some(id),
            assignable,
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

    fn wrap_role(role: Role) -> RoleWithInheritance {
        RoleWithInheritance {
            role,
            inherited_from: None,
        }
    }

    #[tokio::test]
    async fn test_update_member_as_admin() {
        let member = create_test_member(1, 2, 1);
        let user = create_test_user(2, false, USER_STATUS_ACTIVE);
        let role1 = create_test_role(3, "Manager", true);
        let role2 = create_test_role(4, "Developer", true);

        let member_repo = Arc::new(MockMemberRepository::with_member(MemberWithRoles {
            member,
            user: user.clone(),
            roles: vec![wrap_role(role1.clone())],
        }));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let role_repo = Arc::new(MockRoleRepository {
            roles: vec![role1.clone(), role2.clone()],
        });

        let usecase = UpdateMemberUseCase::new(member_repo, project_repo, role_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateMemberDto {
            role_ids: vec![3, 4],
        };

        let result = usecase.execute(1, dto, &current_user).await.unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.project.id, 1);
        assert_eq!(result.user.id, 2);
        assert_eq!(result.roles.len(), 2);
    }

    #[tokio::test]
    async fn test_update_member_not_found() {
        let member_repo = Arc::new(MockMemberRepository::new());
        let project_repo = Arc::new(MockProjectRepository { projects: vec![] });
        let role_repo = Arc::new(MockRoleRepository { roles: vec![] });

        let usecase = UpdateMemberUseCase::new(member_repo, project_repo, role_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateMemberDto { role_ids: vec![1] };

        let result = usecase.execute(999, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_update_member_empty_roles() {
        let member = create_test_member(1, 2, 1);
        let user = create_test_user(2, false, USER_STATUS_ACTIVE);
        let role = create_test_role(1, "Developer", true);

        let member_repo = Arc::new(MockMemberRepository::with_member(MemberWithRoles {
            member,
            user,
            roles: vec![wrap_role(role)],
        }));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let role_repo = Arc::new(MockRoleRepository { roles: vec![] });

        let usecase = UpdateMemberUseCase::new(member_repo, project_repo, role_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateMemberDto { role_ids: vec![] };

        let result = usecase.execute(1, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_update_member_role_not_found() {
        let member = create_test_member(1, 2, 1);
        let user = create_test_user(2, false, USER_STATUS_ACTIVE);
        let role = create_test_role(1, "Developer", true);

        let member_repo = Arc::new(MockMemberRepository::with_member(MemberWithRoles {
            member,
            user,
            roles: vec![wrap_role(role)],
        }));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let role_repo = Arc::new(MockRoleRepository {
            roles: vec![], // No roles available
        });

        let usecase = UpdateMemberUseCase::new(member_repo, project_repo, role_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateMemberDto {
            role_ids: vec![99], // Non-existent role
        };

        let result = usecase.execute(1, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_update_locked_member_should_be_allowed() {
        // Locked users can still have their membership updated
        let member = create_test_member(1, 2, 1);
        let locked_user = User {
            id: 2,
            login: "locked".to_string(),
            status: 3, // Locked status
            ..create_test_user(2, false, USER_STATUS_ACTIVE)
        };
        let role1 = create_test_role(1, "Developer", true);
        let role2 = create_test_role(2, "Reporter", true);

        let member_repo = Arc::new(MockMemberRepository::with_member(MemberWithRoles {
            member,
            user: locked_user.clone(),
            roles: vec![wrap_role(role1.clone())],
        }));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let role_repo = Arc::new(MockRoleRepository {
            roles: vec![role1.clone(), role2.clone()],
        });

        let usecase = UpdateMemberUseCase::new(member_repo, project_repo, role_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateMemberDto {
            role_ids: vec![2], // Change to Reporter role
        };

        let result = usecase.execute(1, dto, &current_user).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.roles.len(), 1);
        assert_eq!(response.roles[0].id, 2);
    }
}
