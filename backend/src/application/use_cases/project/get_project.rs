use crate::application::errors::ApplicationError;
use crate::domain::entities::{Project, PROJECT_STATUS_ARCHIVED};
use crate::domain::repositories::{MemberRepository, ProjectRepository, UserRepository};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Named ID for entities like default assignee
#[derive(Debug, Clone)]
pub struct NamedId {
    pub id: i32,
    pub name: String,
}

/// Project detail for single project response
#[derive(Debug, Clone)]
pub struct ProjectDetail {
    pub id: i32,
    pub name: String,
    pub identifier: String,
    pub description: String,
    pub homepage: String,
    pub status: i32,
    pub is_public: bool,
    pub inherit_members: bool,
    pub default_assignee: Option<NamedId>,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
}

impl ProjectDetail {
    pub fn from_project(project: &Project, default_assignee: Option<NamedId>) -> Self {
        Self {
            id: project.id,
            name: project.name.clone(),
            identifier: project.identifier.clone().unwrap_or_default(),
            description: project.description.clone().unwrap_or_default(),
            homepage: project.homepage.clone().unwrap_or_default(),
            status: project.status,
            is_public: project.is_public,
            inherit_members: project.inherit_members,
            default_assignee,
            created_on: project.created_on.map(|d| d.to_rfc3339()),
            updated_on: project.updated_on.map(|d| d.to_rfc3339()),
        }
    }
}

/// Response for get project endpoint
#[derive(Debug, Clone)]
pub struct GetProjectResponse {
    pub project: ProjectDetail,
}

/// Use case for getting a single project by ID or identifier
pub struct GetProjectUseCase<P: ProjectRepository, M: MemberRepository, U: UserRepository> {
    project_repo: Arc<P>,
    member_repo: Arc<M>,
    user_repo: Arc<U>,
}

impl<P: ProjectRepository, M: MemberRepository, U: UserRepository> GetProjectUseCase<P, M, U> {
    pub fn new(project_repo: Arc<P>, member_repo: Arc<M>, user_repo: Arc<U>) -> Self {
        Self {
            project_repo,
            member_repo,
            user_repo,
        }
    }

    /// Execute the use case
    ///
    /// Visibility rules:
    /// - Admin users can see all projects
    /// - Regular users can see public projects + projects they are a member of
    /// - Archived projects are only visible to admins
    pub async fn execute(
        &self,
        id_or_identifier: &str,
        current_user: &CurrentUser,
    ) -> Result<GetProjectResponse, ApplicationError> {
        // 1. Find project by ID or identifier
        let project = if let Ok(id) = id_or_identifier.parse::<i32>() {
            self.project_repo.find_by_id(id).await
        } else {
            self.project_repo.find_by_identifier(id_or_identifier).await
        }
        .map_err(|e| ApplicationError::Internal(e.to_string()))?
        .ok_or_else(|| ApplicationError::NotFound("Project not found".into()))?;

        // 2. Check visibility
        let can_view = current_user.admin
            || project.is_public
            || self
                .member_repo
                .is_member(project.id, current_user.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if !can_view {
            return Err(ApplicationError::Forbidden(
                "You don't have permission to view this project".into(),
            ));
        }

        // 3. Check archived status - only admins can view archived projects
        if project.status == PROJECT_STATUS_ARCHIVED && !current_user.admin {
            return Err(ApplicationError::Forbidden("Project is archived".into()));
        }

        // 4. Get default assignee if set
        let default_assignee = if let Some(assignee_id) = project.default_assigned_to_id {
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

        // 5. Build response
        Ok(GetProjectResponse {
            project: ProjectDetail::from_project(&project, default_assignee),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{User, PROJECT_STATUS_ACTIVE};
    use crate::domain::repositories::RepositoryError;
    use chrono::Utc;

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
            identifier: &str,
        ) -> Result<Option<Project>, RepositoryError> {
            Ok(self
                .projects
                .iter()
                .find(|p| p.identifier.as_deref() == Some(identifier))
                .cloned())
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
            unimplemented!("Not used in get_project tests")
        }

        async fn exists_by_identifier(&self, _identifier: &str) -> Result<bool, RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn get_max_rgt(&self) -> Result<Option<i32>, RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn add_tracker(
            &self,
            _project_id: i32,
            _tracker_id: i32,
        ) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn update_nested_set_for_insert(&self, _lft: i32) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn update(&self, _project: Project) -> Result<Project, RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn set_trackers(
            &self,
            _project_id: i32,
            _tracker_ids: &[i32],
        ) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn exists_by_identifier_excluding(
            &self,
            _identifier: &str,
            _exclude_project_id: i32,
        ) -> Result<bool, RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn find_children(&self, _project_id: i32) -> Result<Vec<Project>, RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn delete(&self, _project_id: i32) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn clear_trackers(&self, _project_id: i32) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
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
            unimplemented!("Not used in get_project tests")
        }

        async fn add_member_role(
            &self,
            _member_id: i32,
            _role_id: i32,
            _inherited_from: Option<i32>,
        ) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn add_manager(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn inherit_from_parent(
            &self,
            _project_id: i32,
            _parent_id: i32,
        ) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn delete_by_project(&self, _project_id: i32) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn find_by_id(
            &self,
            _member_id: i32,
        ) -> Result<Option<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn find_by_project_and_user(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<Option<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn clear_roles(&self, _member_id: i32) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
        }

        async fn delete_by_id(&self, _member_id: i32) -> Result<(), RepositoryError> {
            unimplemented!("Not used in get_project tests")
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

        async fn update(&self, user: User) -> Result<User, RepositoryError> {
            Ok(user)
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

    fn create_test_project(
        id: i32,
        identifier: &str,
        is_public: bool,
        status: i32,
        default_assigned_to_id: Option<i32>,
    ) -> Project {
        Project {
            id,
            name: format!("Project {}", id),
            description: Some(format!("Description for project {}", id)),
            homepage: None,
            is_public,
            parent_id: None,
            created_on: Some(Utc::now()),
            updated_on: Some(Utc::now()),
            identifier: Some(identifier.to_string()),
            status,
            lft: None,
            rgt: None,
            inherit_members: false,
            default_version_id: None,
            default_assigned_to_id,
        }
    }

    fn create_test_user(id: i32, firstname: &str, lastname: &str) -> User {
        User {
            id,
            login: format!("user{}", id),
            hashed_password: Some("hash".to_string()),
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
            admin: false,
            status: 1,
            last_login_on: None,
            language: Some("en".to_string()),
            auth_source_id: None,
            created_on: Some(Utc::now()),
            updated_on: Some(Utc::now()),
            r#type: None,
            mail_notification: "only_my_events".to_string(),
            salt: Some("salt".to_string()),
            must_change_passwd: false,
            passwd_changed_on: None,
            twofa_scheme: None,
            twofa_totp_key: None,
            twofa_totp_last_used_at: None,
            twofa_required: false,
        }
    }

    #[tokio::test]
    async fn test_get_project_by_id_as_admin() {
        let projects = vec![create_test_project(
            1,
            "proj1",
            false,
            PROJECT_STATUS_ACTIVE,
            None,
        )];
        let users = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository { users });

        let usecase = GetProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute("1", &current_user).await.unwrap();
        assert_eq!(result.project.id, 1);
        assert_eq!(result.project.identifier, "proj1");
    }

    #[tokio::test]
    async fn test_get_project_by_identifier() {
        let projects = vec![create_test_project(
            1,
            "proj1",
            true,
            PROJECT_STATUS_ACTIVE,
            None,
        )];
        let users = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository { users });

        let usecase = GetProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let result = usecase.execute("proj1", &current_user).await.unwrap();
        assert_eq!(result.project.id, 1);
        assert_eq!(result.project.identifier, "proj1");
    }

    #[tokio::test]
    async fn test_get_public_project_as_non_member() {
        let projects = vec![create_test_project(
            1,
            "proj1",
            true,
            PROJECT_STATUS_ACTIVE,
            None,
        )];
        let users = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository { users });

        let usecase = GetProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let result = usecase.execute("1", &current_user).await.unwrap();
        assert_eq!(result.project.id, 1);
    }

    #[tokio::test]
    async fn test_get_private_project_as_member() {
        let projects = vec![create_test_project(
            1,
            "proj1",
            false,
            PROJECT_STATUS_ACTIVE,
            None,
        )];
        let users = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });
        let user_repo = Arc::new(MockUserRepository { users });

        let usecase = GetProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let result = usecase.execute("1", &current_user).await.unwrap();
        assert_eq!(result.project.id, 1);
    }

    #[tokio::test]
    async fn test_get_private_project_forbidden_for_non_member() {
        let projects = vec![create_test_project(
            1,
            "proj1",
            false,
            PROJECT_STATUS_ACTIVE,
            None,
        )];
        let users = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository { users });

        let usecase = GetProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let result = usecase.execute("1", &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_get_archived_project_as_admin() {
        let projects = vec![create_test_project(
            1,
            "proj1",
            true,
            PROJECT_STATUS_ARCHIVED,
            None,
        )];
        let users = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository { users });

        let usecase = GetProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute("1", &current_user).await.unwrap();
        assert_eq!(result.project.id, 1);
        assert_eq!(result.project.status, PROJECT_STATUS_ARCHIVED);
    }

    #[tokio::test]
    async fn test_get_archived_project_forbidden_for_non_admin() {
        let projects = vec![create_test_project(
            1,
            "proj1",
            true,
            PROJECT_STATUS_ARCHIVED,
            None,
        )];
        let users = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });
        let user_repo = Arc::new(MockUserRepository { users });

        let usecase = GetProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let result = usecase.execute("1", &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_get_project_not_found() {
        let projects = vec![];
        let users = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository { users });

        let usecase = GetProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute("999", &current_user).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_get_project_with_default_assignee() {
        let projects = vec![create_test_project(
            1,
            "proj1",
            true,
            PROJECT_STATUS_ACTIVE,
            Some(2),
        )];
        let users = vec![create_test_user(2, "John", "Doe")];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let user_repo = Arc::new(MockUserRepository { users });

        let usecase = GetProjectUseCase::new(project_repo, member_repo, user_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let result = usecase.execute("1", &current_user).await.unwrap();
        assert_eq!(result.project.id, 1);
        assert!(result.project.default_assignee.is_some());
        let assignee = result.project.default_assignee.unwrap();
        assert_eq!(assignee.id, 2);
        assert_eq!(assignee.name, "John Doe");
    }
}
