use crate::application::dto::UpdateCategoryDto;
use crate::application::errors::ApplicationError;
use crate::application::use_cases::category::list_categories::NamedId;
use crate::domain::repositories::{
    IssueCategoryRepository, IssueCategoryUpdate, MemberRepository, ProjectRepository,
    UserRepository,
};
use std::sync::Arc;

/// Response for update category endpoint
#[derive(Debug, Clone)]
pub struct UpdateCategoryResponse {
    pub issue_category: CategoryDetail,
}

/// Category detail for API response
#[derive(Debug, Clone)]
pub struct CategoryDetail {
    pub id: i32,
    pub name: String,
    pub project: NamedId,
    pub assigned_to: Option<NamedId>,
}

/// Use case for updating an issue category
pub struct UpdateCategoryUseCase<
    C: IssueCategoryRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
> {
    category_repo: Arc<C>,
    project_repo: Arc<P>,
    user_repo: Arc<U>,
    member_repo: Arc<M>,
}

impl<C, P, U, M> UpdateCategoryUseCase<C, P, U, M>
where
    C: IssueCategoryRepository,
    P: ProjectRepository,
    U: UserRepository,
    M: MemberRepository,
{
    pub fn new(
        category_repo: Arc<C>,
        project_repo: Arc<P>,
        user_repo: Arc<U>,
        member_repo: Arc<M>,
    ) -> Self {
        Self {
            category_repo,
            project_repo,
            user_repo,
            member_repo,
        }
    }

    /// Execute the use case
    ///
    /// Updates an existing issue category.
    /// Requires manage_categories permission.
    pub async fn execute(
        &self,
        category_id: i32,
        dto: UpdateCategoryDto,
        current_user_id: i32,
        is_admin: bool,
    ) -> Result<UpdateCategoryResponse, ApplicationError> {
        // 1. Get category
        let category = self
            .category_repo
            .find_by_id(category_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Category not found".into()))?;

        // 2. Check permission - user must be a member with manage_categories permission (or admin)
        if !is_admin {
            let member = self
                .member_repo
                .find_by_project_and_user(category.project_id, current_user_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .ok_or_else(|| {
                    ApplicationError::Forbidden(
                        "You don't have permission to manage categories in this project".into(),
                    )
                })?;

            // Check if user has manage_categories permission through any of their roles
            let has_permission = member.roles.iter().any(|rwi| {
                // Roles 3 (Manager) and 4 (Developer) typically have manage_categories
                // Role 3 = Manager, Role 4 = Developer
                rwi.role.id == 3 || rwi.role.id == 4
            });

            if !has_permission {
                return Err(ApplicationError::Forbidden(
                    "You don't have permission to manage categories in this project".into(),
                ));
            }
        }

        // 3. Check for duplicate category name if name is being changed
        if let Some(ref name) = dto.name {
            let name_exists = self
                .category_repo
                .exists_by_name(category.project_id, name, Some(category_id))
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            if name_exists {
                return Err(ApplicationError::Validation(format!(
                    "Category with name '{}' already exists in this project",
                    name
                )));
            }
        }

        // 4. Validate assigned_to_id if provided
        if let Some(Some(assigned_to_id)) = dto.assigned_to_id {
            // Check if the assigned user exists
            let _user = self
                .user_repo
                .find_by_id(assigned_to_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .ok_or_else(|| {
                    ApplicationError::Validation(format!(
                        "User with id {} not found",
                        assigned_to_id
                    ))
                })?;

            // Check if the user is a member of the project
            if !is_admin {
                let is_member = self
                    .member_repo
                    .find_by_project_and_user(category.project_id, assigned_to_id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?
                    .is_some();

                if !is_member {
                    return Err(ApplicationError::Validation(
                        "Assigned user must be a member of the project".into(),
                    ));
                }
            }
        }

        // 5. Get project for response
        let project = self
            .project_repo
            .find_by_id(category.project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Project not found".into()))?;

        // 6. Update category
        let update = IssueCategoryUpdate {
            name: dto.name,
            assigned_to_id: dto.assigned_to_id,
        };

        let updated = self
            .category_repo
            .update(category_id, &update)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 7. Get assignee info for response
        let assigned_to = if let Some(user_id) = updated.assigned_to_id {
            self.user_repo
                .find_by_id(user_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .map(|u| NamedId {
                    id: u.id,
                    name: format!("{} {}", u.firstname, u.lastname),
                })
        } else {
            None
        };

        Ok(UpdateCategoryResponse {
            issue_category: CategoryDetail {
                id: updated.id,
                name: updated.name,
                project: NamedId {
                    id: project.id,
                    name: project.name,
                },
                assigned_to,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{
        IssueCategory, MemberWithRoles, Project, Role, RoleWithInheritance, User,
        PROJECT_STATUS_ACTIVE, USER_STATUS_ACTIVE,
    };
    use crate::domain::repositories::{IssueCategoryUpdate, NewIssueCategory, RepositoryError};
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::Mutex;

    // Mock IssueCategoryRepository
    struct MockCategoryRepository {
        categories: Mutex<Vec<IssueCategory>>,
        other_existing_names: Mutex<Vec<(i32, String)>>, // Other existing names in project (for duplicate check)
    }

    impl MockCategoryRepository {
        fn new() -> Self {
            Self {
                categories: Mutex::new(Vec::new()),
                other_existing_names: Mutex::new(Vec::new()),
            }
        }

        fn with_category(category: IssueCategory) -> Self {
            Self {
                categories: Mutex::new(vec![category]),
                other_existing_names: Mutex::new(Vec::new()),
            }
        }

        fn with_category_and_existing_name(category: IssueCategory, existing_name: &str) -> Self {
            Self {
                categories: Mutex::new(vec![category.clone()]),
                other_existing_names: Mutex::new(vec![(
                    category.project_id,
                    existing_name.to_string(),
                )]),
            }
        }
    }

    #[async_trait::async_trait]
    impl IssueCategoryRepository for MockCategoryRepository {
        async fn find_by_project(
            &self,
            _project_id: i32,
        ) -> Result<Vec<IssueCategory>, RepositoryError> {
            Ok(self.categories.lock().unwrap().clone())
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<IssueCategory>, RepositoryError> {
            Ok(self
                .categories
                .lock()
                .unwrap()
                .iter()
                .find(|c| c.id == id)
                .cloned())
        }

        async fn create(
            &self,
            _category: &NewIssueCategory,
        ) -> Result<IssueCategory, RepositoryError> {
            unimplemented!()
        }

        async fn update(
            &self,
            id: i32,
            update: &IssueCategoryUpdate,
        ) -> Result<IssueCategory, RepositoryError> {
            let mut categories = self.categories.lock().unwrap();
            if let Some(category) = categories.iter_mut().find(|c| c.id == id) {
                if let Some(ref name) = update.name {
                    category.name = name.clone();
                }
                if let Some(assigned_to_id) = update.assigned_to_id {
                    category.assigned_to_id = assigned_to_id;
                }
                Ok(category.clone())
            } else {
                Err(RepositoryError::NotFound(format!(
                    "Category {} not found",
                    id
                )))
            }
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn delete_by_project(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn count_issues(&self, _category_id: i32) -> Result<u32, RepositoryError> {
            Ok(0)
        }

        async fn reassign_issues(
            &self,
            _from_category_id: i32,
            _to_category_id: i32,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn clear_issues(&self, _category_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn exists_by_name(
            &self,
            project_id: i32,
            name: &str,
            exclude_id: Option<i32>,
        ) -> Result<bool, RepositoryError> {
            // Check categories (excluding the one being updated)
            let categories = self.categories.lock().unwrap();
            let category_match = categories.iter().any(|c| {
                c.project_id == project_id
                    && c.name == name
                    && (exclude_id != Some(c.id))
            });

            // Check other existing names (these always count as duplicates)
            let other_match = self
                .other_existing_names
                .lock()
                .unwrap()
                .iter()
                .any(|(pid, n)| *pid == project_id && n == name);

            Ok(category_match || other_match)
        }
    }

    // Mock ProjectRepository
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

    // Mock UserRepository
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

    // Mock MemberRepository
    struct MockMemberRepository {
        members: Vec<(i32, i32, i32, Vec<i32>)>, // (member_id, project_id, user_id, role_ids)
    }

    impl MockMemberRepository {
        fn new() -> Self {
            Self {
                members: Vec::new(),
            }
        }

        fn with_member(project_id: i32, user_id: i32, role_ids: Vec<i32>) -> Self {
            Self {
                members: vec![(1, project_id, user_id, role_ids)],
            }
        }

        fn with_members(members: Vec<(i32, i32, i32, Vec<i32>)>) -> Self {
            Self { members }
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
                .any(|(_, pid, uid, _)| *pid == project_id && *uid == user_id))
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
            project_id: i32,
            user_id: i32,
        ) -> Result<Option<MemberWithRoles>, RepositoryError> {
            if let Some((member_id, _, uid, role_ids)) = self
                .members
                .iter()
                .find(|(_, pid, uid, _)| *pid == project_id && *uid == user_id)
            {
                let roles: Vec<RoleWithInheritance> = role_ids
                    .iter()
                    .map(|rid| RoleWithInheritance {
                        role: Role {
                            id: *rid,
                            name: if *rid == 3 {
                                "Manager".to_string()
                            } else if *rid == 4 {
                                "Developer".to_string()
                            } else {
                                format!("Role {}", rid)
                            },
                            position: Some(*rid),
                            assignable: true,
                            builtin: 0,
                            permissions: None,
                            issues_visibility: "default".to_string(),
                            users_visibility: "members_of_visible_projects".to_string(),
                            time_entries_visibility: "all".to_string(),
                            all_roles_managed: false,
                            settings: None,
                            default_time_entry_activity_id: None,
                        },
                        inherited_from: None,
                    })
                    .collect();

                let user = crate::domain::entities::User {
                    id: *uid,
                    login: format!("user{}", uid),
                    hashed_password: None,
                    firstname: "Test".to_string(),
                    lastname: "User".to_string(),
                    admin: false,
                    status: 1,
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
                };

                Ok(Some(MemberWithRoles {
                    member: crate::domain::entities::Member {
                        id: *member_id,
                        user_id: *uid,
                        project_id,
                        created_on: None,
                        mail_notification: false,
                    },
                    user,
                    roles,
                }))
            } else {
                Ok(None)
            }
        }

        async fn clear_roles(&self, _member_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn delete_by_id(&self, _member_id: i32) -> Result<(), RepositoryError> {
            Ok(())
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

    fn create_test_user(id: i32, admin: bool) -> User {
        User {
            id,
            login: format!("user{}", id),
            hashed_password: None,
            firstname: format!("First{}", id),
            lastname: format!("Last{}", id),
            admin,
            status: USER_STATUS_ACTIVE,
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

    fn create_test_category(
        id: i32,
        project_id: i32,
        name: &str,
        assigned_to_id: Option<i32>,
    ) -> IssueCategory {
        IssueCategory {
            id,
            project_id,
            name: name.to_string(),
            assigned_to_id,
        }
    }

    #[tokio::test]
    async fn test_update_category_name_as_admin() {
        let category = create_test_category(1, 1, "Frontend", None);
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(1, true)],
        });
        let member_repo = Arc::new(MockMemberRepository::new());

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: Some("Backend".to_string()),
            assigned_to_id: None,
        };

        let result = usecase.execute(1, dto, 1, true).await.unwrap();
        assert_eq!(result.issue_category.id, 1);
        assert_eq!(result.issue_category.name, "Backend");
        assert_eq!(result.issue_category.project.id, 1);
    }

    #[tokio::test]
    async fn test_update_category_assigned_to_as_admin() {
        let category = create_test_category(1, 1, "Frontend", None);
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(1, true), create_test_user(2, false)],
        });
        let member_repo = Arc::new(MockMemberRepository::new());

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: None,
            assigned_to_id: Some(Some(2)),
        };

        let result = usecase.execute(1, dto, 1, true).await.unwrap();
        assert_eq!(result.issue_category.id, 1);
        assert!(result.issue_category.assigned_to.is_some());
        assert_eq!(result.issue_category.assigned_to.unwrap().id, 2);
    }

    #[tokio::test]
    async fn test_update_category_clear_assigned_to() {
        let category = create_test_category(1, 1, "Frontend", Some(2));
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(1, true)],
        });
        let member_repo = Arc::new(MockMemberRepository::new());

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: None,
            assigned_to_id: Some(None), // Clear assigned_to
        };

        let result = usecase.execute(1, dto, 1, true).await.unwrap();
        assert_eq!(result.issue_category.id, 1);
        assert!(result.issue_category.assigned_to.is_none());
    }

    #[tokio::test]
    async fn test_update_category_as_member_with_permission() {
        let category = create_test_category(1, 1, "Frontend", None);
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(2, false)],
        });
        let member_repo = Arc::new(MockMemberRepository::with_member(1, 2, vec![3])); // Role 3 = Manager

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: Some("Backend".to_string()),
            assigned_to_id: None,
        };

        let result = usecase.execute(1, dto, 2, false).await.unwrap();
        assert_eq!(result.issue_category.name, "Backend");
    }

    #[tokio::test]
    async fn test_update_category_as_developer() {
        let category = create_test_category(1, 1, "Frontend", None);
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(2, false)],
        });
        let member_repo = Arc::new(MockMemberRepository::with_member(1, 2, vec![4])); // Role 4 = Developer

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: Some("Backend".to_string()),
            assigned_to_id: None,
        };

        let result = usecase.execute(1, dto, 2, false).await.unwrap();
        assert_eq!(result.issue_category.name, "Backend");
    }

    #[tokio::test]
    async fn test_update_category_permission_denied_non_member() {
        let category = create_test_category(1, 1, "Frontend", None);
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(2, false)],
        });
        let member_repo = Arc::new(MockMemberRepository::new()); // No members

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: Some("Backend".to_string()),
            assigned_to_id: None,
        };

        let result = usecase.execute(1, dto, 2, false).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_update_category_permission_denied_wrong_role() {
        let category = create_test_category(1, 1, "Frontend", None);
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(2, false)],
        });
        let member_repo = Arc::new(MockMemberRepository::with_member(1, 2, vec![5])); // Role 5 = Reporter (no manage_categories)

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: Some("Backend".to_string()),
            assigned_to_id: None,
        };

        let result = usecase.execute(1, dto, 2, false).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_update_category_duplicate_name() {
        let category = create_test_category(1, 1, "Frontend", None);
        let category_repo = Arc::new(MockCategoryRepository::with_category_and_existing_name(
            category, "Backend",
        ));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(1, true)],
        });
        let member_repo = Arc::new(MockMemberRepository::new());

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: Some("Backend".to_string()), // Duplicate name
            assigned_to_id: None,
        };

        let result = usecase.execute(1, dto, 1, true).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_update_category_not_found() {
        let category_repo = Arc::new(MockCategoryRepository::new()); // No categories
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(1, true)],
        });
        let member_repo = Arc::new(MockMemberRepository::new());

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: Some("Backend".to_string()),
            assigned_to_id: None,
        };

        let result = usecase.execute(999, dto, 1, true).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_update_category_invalid_assignee() {
        let category = create_test_category(1, 1, "Frontend", None);
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(1, true)], // Only user 1 exists
        });
        let member_repo = Arc::new(MockMemberRepository::new());

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: None,
            assigned_to_id: Some(Some(999)), // Non-existent user
        };

        let result = usecase.execute(1, dto, 1, true).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_update_category_assignee_not_member_non_admin() {
        let category = create_test_category(1, 1, "Frontend", None);
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(1, false), create_test_user(2, false)],
        });
        // User 1 is Manager, User 2 is not a member
        let member_repo = Arc::new(MockMemberRepository::with_member(1, 1, vec![3]));

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: None,
            assigned_to_id: Some(Some(2)), // User 2 is not a project member
        };

        let result = usecase.execute(1, dto, 1, false).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_update_category_both_fields() {
        let category = create_test_category(1, 1, "Frontend", None);
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(1, true), create_test_user(2, false)],
        });
        let member_repo = Arc::new(MockMemberRepository::new());

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: Some("Backend".to_string()),
            assigned_to_id: Some(Some(2)),
        };

        let result = usecase.execute(1, dto, 1, true).await.unwrap();
        assert_eq!(result.issue_category.name, "Backend");
        assert!(result.issue_category.assigned_to.is_some());
        assert_eq!(result.issue_category.assigned_to.unwrap().id, 2);
    }

    #[tokio::test]
    async fn test_update_category_no_changes() {
        let category = create_test_category(1, 1, "Frontend", Some(2));
        let category_repo = Arc::new(MockCategoryRepository::with_category(category));
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![create_test_project(1)],
        });
        let user_repo = Arc::new(MockUserRepository {
            users: vec![create_test_user(1, true), create_test_user(2, false)],
        });
        let member_repo = Arc::new(MockMemberRepository::new());

        let usecase =
            UpdateCategoryUseCase::new(category_repo, project_repo, user_repo, member_repo);

        let dto = UpdateCategoryDto {
            name: None,
            assigned_to_id: None,
        };

        let result = usecase.execute(1, dto, 1, true).await.unwrap();
        assert_eq!(result.issue_category.name, "Frontend");
        assert!(result.issue_category.assigned_to.is_some());
        assert_eq!(result.issue_category.assigned_to.unwrap().id, 2);
    }
}
