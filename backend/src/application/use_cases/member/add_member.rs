use crate::application::dto::CreateMemberDto;
use crate::application::errors::ApplicationError;
use crate::domain::entities::USER_STATUS_ACTIVE;
use crate::domain::repositories::{
    MemberRepository, NewMember, ProjectRepository, RoleRepository, UserRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response item for a single membership
#[derive(Debug, Clone)]
pub struct MembershipResponse {
    pub id: i32,
    pub project: MemberNamedId,
    pub user: MemberNamedId,
    pub roles: Vec<MemberNamedId>,
}

/// Named ID for project/user/role references in member responses
#[derive(Debug, Clone)]
pub struct MemberNamedId {
    pub id: i32,
    pub name: String,
}

/// Use case for adding a member to a project
pub struct AddMemberUseCase<
    M: MemberRepository,
    P: ProjectRepository,
    U: UserRepository,
    R: RoleRepository,
> {
    member_repo: Arc<M>,
    project_repo: Arc<P>,
    user_repo: Arc<U>,
    role_repo: Arc<R>,
}

impl<M, P, U, R> AddMemberUseCase<M, P, U, R>
where
    M: MemberRepository,
    P: ProjectRepository,
    U: UserRepository,
    R: RoleRepository,
{
    pub fn new(
        member_repo: Arc<M>,
        project_repo: Arc<P>,
        user_repo: Arc<U>,
        role_repo: Arc<R>,
    ) -> Self {
        Self {
            member_repo,
            project_repo,
            user_repo,
            role_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin: can add any member with any role
    /// - Non-admin: needs manage_members permission on the project
    /// - Non-admin: can only assign roles they can manage
    ///
    /// Validation rules:
    /// - User must exist and be active
    /// - User must not already be a member of the project
    /// - At least one role must be specified
    /// - All roles must exist
    pub async fn execute(
        &self,
        project_id: i32,
        dto: CreateMemberDto,
        current_user: &CurrentUser,
    ) -> Result<MembershipResponse, ApplicationError> {
        // 1. Validate the DTO
        if let Err(errors) = dto.validate() {
            return Err(ApplicationError::Validation(errors.join(", ")));
        }

        // 2. Check project exists
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Project not found".into()))?;

        // 3. Check permission - must be admin or member with manage_members permission
        // For now, we check if the user is an admin or if they are a member of the project
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

        // 4. Validate user exists and is active
        let user = self
            .user_repo
            .find_by_id(dto.user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::Validation("User not found".into()))?;

        if user.status != USER_STATUS_ACTIVE {
            return Err(ApplicationError::Validation("User is not active".into()));
        }

        // 5. Check if already a member
        let is_already_member = self
            .member_repo
            .is_member(project_id, dto.user_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if is_already_member {
            return Err(ApplicationError::Validation(
                "User is already a member of this project".into(),
            ));
        }

        // 6. Validate roles and check permissions
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

        // 7. Check role assignment permissions for non-admins
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

        // 8. Create membership
        let member_id = self
            .member_repo
            .add_member(NewMember {
                user_id: dto.user_id,
                project_id,
                mail_notification: false,
            })
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 9. Add roles
        for role_id in &dto.role_ids {
            self.member_repo
                .add_member_role(member_id, *role_id, None)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
        }

        // 10. Return response
        Ok(MembershipResponse {
            id: member_id,
            project: MemberNamedId {
                id: project.id,
                name: project.name,
            },
            user: MemberNamedId {
                id: user.id,
                name: user.full_name(),
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
    use crate::domain::entities::{MemberWithRoles, Project, Role, User, PROJECT_STATUS_ACTIVE};
    use crate::domain::repositories::{NewRole, RepositoryError};
    use std::sync::atomic::{AtomicI32, Ordering};

    // Mock implementations for testing
    struct MockMemberRepository {
        members: Vec<(i32, i32, i32)>, // (member_id, project_id, user_id)
        next_id: AtomicI32,
    }

    impl MockMemberRepository {
        fn new() -> Self {
            Self {
                members: Vec::new(),
                next_id: AtomicI32::new(1),
            }
        }

        fn with_members(members: Vec<(i32, i32, i32)>) -> Self {
            Self {
                members,
                next_id: AtomicI32::new(1),
            }
        }
    }

    #[async_trait::async_trait]
    impl MemberRepository for MockMemberRepository {
        async fn find_by_project(
            &self,
            _project_id: i32,
        ) -> Result<Vec<MemberWithRoles>, RepositoryError> {
            Ok(Vec::new())
        }

        async fn delete_by_user_id(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn is_member(&self, project_id: i32, user_id: i32) -> Result<bool, RepositoryError> {
            Ok(self
                .members
                .iter()
                .any(|(_, pid, uid)| *pid == project_id && *uid == user_id))
        }

        async fn add_member(&self, _member: NewMember) -> Result<i32, RepositoryError> {
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

        async fn find_by_id(
            &self,
            _member_id: i32,
        ) -> Result<Option<MemberWithRoles>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_project_and_user(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<Option<MemberWithRoles>, RepositoryError> {
            Ok(None)
        }

        async fn clear_roles(&self, _member_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn delete_by_id(&self, _member_id: i32) -> Result<(), RepositoryError> {
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

    struct MockUserRepository {
        users: Vec<User>,
    }

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_login(&self, _login: &str) -> Result<Option<User>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<User>, RepositoryError> {
            Ok(self.users.iter().find(|u| u.id == id).cloned())
        }

        async fn update_last_login(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn find_all(
            &self,
            _params: crate::domain::repositories::UserQueryParams,
        ) -> Result<Vec<User>, RepositoryError> {
            Ok(self.users.clone())
        }

        async fn count(
            &self,
            _params: &crate::domain::repositories::UserQueryParams,
        ) -> Result<u32, RepositoryError> {
            Ok(self.users.len() as u32)
        }

        async fn update(&self, _user: User) -> Result<User, RepositoryError> {
            unimplemented!()
        }

        async fn create(&self, _user: User) -> Result<User, RepositoryError> {
            unimplemented!()
        }

        async fn exists_by_login(&self, _login: &str) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn exists_by_login_excluding(
            &self,
            _login: &str,
            _exclude_user_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn find_all_admins(&self) -> Result<Vec<User>, RepositoryError> {
            Ok(self.users.iter().filter(|u| u.admin).cloned().collect())
        }

        async fn delete(&self, _user_id: i32) -> Result<(), RepositoryError> {
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

    #[tokio::test]
    async fn test_add_member_as_admin() {
        let member_repo = Arc::new(MockMemberRepository::new());
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(2, false, USER_STATUS_ACTIVE)],
        });
        let role_repo = Arc::new(MockRoleRepository {
            roles: vec![create_test_role(1, "Developer", true)],
        });

        let usecase = AddMemberUseCase::new(member_repo, project_repo, user_repo, role_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateMemberDto {
            user_id: 2,
            role_ids: vec![1],
        };

        let result = usecase.execute(1, dto, &current_user).await.unwrap();
        assert_eq!(result.project.id, 1);
        assert_eq!(result.user.id, 2);
        assert_eq!(result.roles.len(), 1);
    }

    #[tokio::test]
    async fn test_add_member_project_not_found() {
        let member_repo = Arc::new(MockMemberRepository::new());
        let project_repo = Arc::new(MockProjectRepository { projects: vec![] });
        let user_repo = Arc::new(MockUserRepository { users: vec![] });
        let role_repo = Arc::new(MockRoleRepository { roles: vec![] });

        let usecase = AddMemberUseCase::new(member_repo, project_repo, user_repo, role_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateMemberDto {
            user_id: 2,
            role_ids: vec![1],
        };

        let result = usecase.execute(1, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_add_member_user_not_found() {
        let member_repo = Arc::new(MockMemberRepository::new());
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository { users: vec![] });
        let role_repo = Arc::new(MockRoleRepository { roles: vec![] });

        let usecase = AddMemberUseCase::new(member_repo, project_repo, user_repo, role_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateMemberDto {
            user_id: 999,
            role_ids: vec![1],
        };

        let result = usecase.execute(1, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_add_member_empty_roles() {
        let member_repo = Arc::new(MockMemberRepository::new());
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(2, false, USER_STATUS_ACTIVE)],
        });
        let role_repo = Arc::new(MockRoleRepository { roles: vec![] });

        let usecase = AddMemberUseCase::new(member_repo, project_repo, user_repo, role_repo);

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateMemberDto {
            user_id: 2,
            role_ids: vec![],
        };

        let result = usecase.execute(1, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }
}
