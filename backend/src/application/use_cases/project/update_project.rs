use crate::application::dto::UpdateProjectDto;
use crate::application::errors::ApplicationError;
use crate::application::use_cases::project::{NamedId, ProjectDetail};
use crate::domain::repositories::{MemberRepository, ProjectRepository, UserRepository};
use crate::presentation::middleware::CurrentUser;
use chrono::Utc;
use std::sync::Arc;

/// Response for update project endpoint
#[derive(Debug, Clone)]
pub struct UpdateProjectResponse {
    pub project: ProjectDetail,
}

/// Use case for updating an existing project
pub struct UpdateProjectUseCase<P: ProjectRepository, M: MemberRepository, U: UserRepository> {
    project_repo: Arc<P>,
    member_repo: Arc<M>,
    user_repo: Arc<U>,
}

impl<P: ProjectRepository, M: MemberRepository, U: UserRepository> UpdateProjectUseCase<P, M, U> {
    pub fn new(project_repo: Arc<P>, member_repo: Arc<M>, user_repo: Arc<U>) -> Self {
        Self {
            project_repo,
            member_repo,
            user_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin: can update any project and change identifier/status
    /// - Non-admin: needs `edit_project` permission on the project
    /// - Changing `is_public` requires `select_project_publicity` permission
    ///
    /// Validation rules:
    /// - Name: if provided, cannot be blank, max 255 characters
    /// - Identifier: if provided (admin only), must be unique, URL-safe format
    /// - Parent project must exist if parent_id is specified
    pub async fn execute(
        &self,
        project_id: i32,
        dto: UpdateProjectDto,
        current_user: &CurrentUser,
    ) -> Result<UpdateProjectResponse, ApplicationError> {
        // 1. Get existing project
        let mut project = self
            .project_repo
            .find_by_id(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Project not found".into()))?;

        // 2. Check permission
        if !current_user.admin {
            let can_edit = self
                .member_repo
                .is_member(project.id, current_user.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            if !can_edit {
                return Err(ApplicationError::Forbidden(
                    "You don't have permission to edit this project".into(),
                ));
            }
        }

        // 3. Validate and update name if provided
        if let Some(ref name) = dto.name {
            if let Err(e) = UpdateProjectDto::validate_name(name) {
                return Err(ApplicationError::Validation(e));
            }
            project.name = name.trim().to_string();
        }

        // 4. Update description if provided
        if let Some(ref description) = dto.description {
            project.description = if description.trim().is_empty() {
                None
            } else {
                Some(description.trim().to_string())
            };
        }

        // 5. Update homepage if provided
        if let Some(ref homepage) = dto.homepage {
            project.homepage = if homepage.trim().is_empty() {
                None
            } else {
                Some(homepage.trim().to_string())
            };
        }

        // 6. Update is_public (requires select_project_publicity permission for non-admins)
        if let Some(is_public) = dto.is_public {
            if !current_user.admin {
                // TODO: Check for select_project_publicity permission when permission service is implemented
                // For now, only admins can change publicity
                return Err(ApplicationError::Forbidden(
                    "You don't have permission to change project publicity".into(),
                ));
            }
            project.is_public = is_public;
        }

        // 7. Update parent_id if provided
        if let Some(parent_id) = dto.parent_id {
            // Validate parent exists and is not the same project
            if parent_id == project.id {
                return Err(ApplicationError::Validation(
                    "Project cannot be its own parent".into(),
                ));
            }

            let _parent = self
                .project_repo
                .find_by_id(parent_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .ok_or_else(|| ApplicationError::Validation("Parent project not found".into()))?;

            // TODO: Check for circular parent references
            // For now, just validate parent exists

            project.parent_id = Some(parent_id);
        }

        // 8. Update inherit_members
        if let Some(inherit_members) = dto.inherit_members {
            let old_inherit = project.inherit_members;
            project.inherit_members = inherit_members;

            // If changed from false to true, inherit members from parent
            if !old_inherit && inherit_members {
                if let Some(parent_id) = project.parent_id {
                    self.member_repo
                        .inherit_from_parent(project.id, parent_id)
                        .await
                        .map_err(|e| ApplicationError::Internal(e.to_string()))?;
                }
            }
        }

        // 9. Admin-only fields: identifier and status
        if current_user.admin {
            // Update identifier if provided
            if let Some(ref identifier) = dto.identifier {
                if let Err(e) = UpdateProjectDto::validate_identifier(identifier) {
                    return Err(ApplicationError::Validation(e));
                }

                let identifier = identifier.trim();

                // Check uniqueness if identifier is being changed
                if Some(identifier) != project.identifier.as_deref() {
                    if self
                        .project_repo
                        .exists_by_identifier_excluding(identifier, project.id)
                        .await
                        .map_err(|e| ApplicationError::Internal(e.to_string()))?
                    {
                        return Err(ApplicationError::Validation(
                            "Identifier has already been taken".into(),
                        ));
                    }
                    project.identifier = Some(identifier.to_string());
                }
            }

            // Update status if provided
            if let Some(status) = dto.status {
                // Validate status value (1=active, 5=closed, 9=archived)
                if ![1, 5, 9].contains(&status) {
                    return Err(ApplicationError::Validation(
                        "Invalid status value. Must be 1 (active), 5 (closed), or 9 (archived)"
                            .into(),
                    ));
                }
                project.status = status;
            }
        } else {
            // Non-admin trying to change admin-only fields
            if dto.identifier.is_some() {
                return Err(ApplicationError::Forbidden(
                    "Only administrators can change the project identifier".into(),
                ));
            }
            if dto.status.is_some() {
                return Err(ApplicationError::Forbidden(
                    "Only administrators can change the project status".into(),
                ));
            }
        }

        // 10. Update trackers if provided
        if let Some(ref tracker_ids) = dto.tracker_ids {
            self.project_repo
                .set_trackers(project.id, tracker_ids)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
        }

        // 11. Update timestamp
        project.updated_on = Some(Utc::now());

        // 12. Save project
        let updated_project = self
            .project_repo
            .update(project)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 13. Get default assignee if set
        let default_assignee = if let Some(assignee_id) = updated_project.default_assigned_to_id {
            self.user_repo
                .find_by_id(assignee_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .map(|u| NamedId {
                    id: u.id,
                    name: u.full_name(),
                })
        } else {
            None
        };

        // 14. Return response
        Ok(UpdateProjectResponse {
            project: ProjectDetail::from_project(&updated_project, default_assignee),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Project, User, PROJECT_STATUS_ACTIVE, PROJECT_STATUS_CLOSED};
    use crate::domain::repositories::RepositoryError;

    // Mock implementations for testing
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

        async fn update(&self, project: Project) -> Result<Project, RepositoryError> {
            Ok(project)
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
            unimplemented!()
        }

        async fn clear_trackers(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockMemberRepository {
        member_project_ids: Vec<i32>,
    }

    #[async_trait::async_trait]
    impl MemberRepository for MockMemberRepository {
        async fn find_by_project(
            &self,
            _project_id: i32,
        ) -> Result<Vec<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            Ok(vec![])
        }

        async fn delete_by_user_id(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn is_member(&self, project_id: i32, _user_id: i32) -> Result<bool, RepositoryError> {
            Ok(self.member_project_ids.contains(&project_id))
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
        ) -> Result<Option<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_project_and_user(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<Option<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            Ok(None)
        }

        async fn clear_roles(&self, _member_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn delete_by_id(&self, _member_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockUserRepository;

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_login(&self, _login: &str) -> Result<Option<User>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_id(&self, _id: i32) -> Result<Option<User>, RepositoryError> {
            Ok(None)
        }

        async fn update_last_login(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn find_all(
            &self,
            _params: crate::domain::repositories::UserQueryParams,
        ) -> Result<Vec<User>, RepositoryError> {
            Ok(vec![])
        }

        async fn count(
            &self,
            _params: &crate::domain::repositories::UserQueryParams,
        ) -> Result<u32, RepositoryError> {
            Ok(0)
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
            Ok(vec![])
        }

        async fn delete(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    fn create_test_project(id: i32, identifier: &str) -> Project {
        Project {
            id,
            name: format!("Project {}", id),
            description: Some(format!("Description for project {}", id)),
            homepage: None,
            is_public: true,
            parent_id: None,
            created_on: Some(Utc::now()),
            updated_on: Some(Utc::now()),
            identifier: Some(identifier.to_string()),
            status: PROJECT_STATUS_ACTIVE,
            lft: Some(id * 2 - 1),
            rgt: Some(id * 2),
            inherit_members: false,
            default_version_id: None,
            default_assigned_to_id: None,
        }
    }

    #[tokio::test]
    async fn test_update_project_as_admin() {
        let projects = vec![create_test_project(1, "proj1")];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository);

        let usecase = UpdateProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateProjectDto {
            name: Some("Updated Project".to_string()),
            identifier: None,
            description: Some("New description".to_string()),
            homepage: None,
            is_public: None,
            parent_id: None,
            inherit_members: None,
            status: None,
            tracker_ids: None,
        };

        let result = usecase.execute(1, dto, &current_user).await.unwrap();
        assert_eq!(result.project.name, "Updated Project");
        assert_eq!(result.project.description, "New description");
    }

    #[tokio::test]
    async fn test_update_project_as_member() {
        let projects = vec![create_test_project(1, "proj1")];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });
        let user_repo = Arc::new(MockUserRepository);

        let usecase = UpdateProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let dto = UpdateProjectDto {
            name: Some("Updated by Member".to_string()),
            identifier: None,
            description: None,
            homepage: None,
            is_public: None,
            parent_id: None,
            inherit_members: None,
            status: None,
            tracker_ids: None,
        };

        let result = usecase.execute(1, dto, &current_user).await.unwrap();
        assert_eq!(result.project.name, "Updated by Member");
    }

    #[tokio::test]
    async fn test_update_project_non_member_forbidden() {
        let projects = vec![create_test_project(1, "proj1")];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository);

        let usecase = UpdateProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let dto = UpdateProjectDto {
            name: Some("Updated Name".to_string()),
            identifier: None,
            description: None,
            homepage: None,
            is_public: None,
            parent_id: None,
            inherit_members: None,
            status: None,
            tracker_ids: None,
        };

        let result = usecase.execute(1, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_update_project_status_admin_only() {
        let projects = vec![create_test_project(1, "proj1")];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });
        let user_repo = Arc::new(MockUserRepository);

        let usecase = UpdateProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let dto = UpdateProjectDto {
            name: None,
            identifier: None,
            description: None,
            homepage: None,
            is_public: None,
            parent_id: None,
            inherit_members: None,
            status: Some(PROJECT_STATUS_CLOSED),
            tracker_ids: None,
        };

        let result = usecase.execute(1, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_update_project_status_as_admin() {
        let projects = vec![create_test_project(1, "proj1")];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository);

        let usecase = UpdateProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateProjectDto {
            name: None,
            identifier: None,
            description: None,
            homepage: None,
            is_public: None,
            parent_id: None,
            inherit_members: None,
            status: Some(PROJECT_STATUS_CLOSED),
            tracker_ids: None,
        };

        let result = usecase.execute(1, dto, &current_user).await.unwrap();
        assert_eq!(result.project.status, PROJECT_STATUS_CLOSED);
    }

    #[tokio::test]
    async fn test_update_project_not_found() {
        let projects = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository);

        let usecase = UpdateProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateProjectDto {
            name: Some("Updated".to_string()),
            identifier: None,
            description: None,
            homepage: None,
            is_public: None,
            parent_id: None,
            inherit_members: None,
            status: None,
            tracker_ids: None,
        };

        let result = usecase.execute(999, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_update_project_empty_name() {
        let projects = vec![create_test_project(1, "proj1")];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository);

        let usecase = UpdateProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = UpdateProjectDto {
            name: Some("".to_string()),
            identifier: None,
            description: None,
            homepage: None,
            is_public: None,
            parent_id: None,
            inherit_members: None,
            status: None,
            tracker_ids: None,
        };

        let result = usecase.execute(1, dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }
}
