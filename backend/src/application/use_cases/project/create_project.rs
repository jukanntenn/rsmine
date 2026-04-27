use crate::application::dto::CreateProjectDto;
use crate::application::errors::ApplicationError;
use crate::application::use_cases::ProjectDetail;
use crate::domain::entities::{Project, PROJECT_STATUS_ACTIVE};
use crate::domain::repositories::{MemberRepository, ProjectRepository, TrackerRepository};
use crate::presentation::middleware::CurrentUser;
use chrono::Utc;
use std::sync::Arc;

/// Response for create project endpoint
#[derive(Debug, Clone)]
pub struct CreateProjectResponse {
    pub project: ProjectDetail,
}

/// Use case for creating a new project
pub struct CreateProjectUseCase<P: ProjectRepository, M: MemberRepository, T: TrackerRepository> {
    project_repo: Arc<P>,
    member_repo: Arc<M>,
    tracker_repo: Arc<T>,
}

impl<P: ProjectRepository, M: MemberRepository, T: TrackerRepository> CreateProjectUseCase<P, M, T> {
    pub fn new(project_repo: Arc<P>, member_repo: Arc<M>, tracker_repo: Arc<T>) -> Self {
        Self {
            project_repo,
            member_repo,
            tracker_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin: can create any project
    /// - Non-admin: needs global `add_project` permission
    /// - For subprojects: needs `add_subprojects` permission on parent project
    ///
    /// Validation rules:
    /// - Name: required, max 255 characters
    /// - Identifier: required, max 100 characters, URL-safe format, unique
    /// - Parent project must exist if parent_id is specified
    pub async fn execute(
        &self,
        dto: CreateProjectDto,
        current_user: &CurrentUser,
    ) -> Result<CreateProjectResponse, ApplicationError> {
        // 1. Check permission - only admin or users with add_project permission can create projects
        if !current_user.admin {
            // TODO: Check for global add_project permission when permission service is implemented
            // For now, only admins can create projects
            return Err(ApplicationError::Forbidden(
                "You don't have permission to create projects".into(),
            ));
        }

        // 2. Validate fields
        if let Err(errors) = dto.validate() {
            return Err(ApplicationError::Validation(errors.join(", ")));
        }

        // 3. Check identifier uniqueness
        if self
            .project_repo
            .exists_by_identifier(&dto.identifier)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
        {
            return Err(ApplicationError::Validation(
                "Identifier has already been taken".into(),
            ));
        }

        // 4. Validate parent project if specified
        let parent = if let Some(parent_id) = dto.parent_id {
            let parent = self
                .project_repo
                .find_by_id(parent_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .ok_or_else(|| ApplicationError::Validation("Parent project not found".into()))?;

            // TODO: Check add_subprojects permission on parent project for non-admins
            // For now, admins can add subprojects to any project

            Some(parent)
        } else {
            None
        };

        // 5. Calculate nested set position
        let max_rgt = self
            .project_repo
            .get_max_rgt()
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let (lft, rgt) =
            Self::calculate_nested_set_position(parent.as_ref().and_then(|p| p.rgt), max_rgt);

        // 6. Update existing nested set values if needed
        // When inserting into an existing tree, we need to shift existing nodes
        if max_rgt.is_some() {
            self.project_repo
                .update_nested_set_for_insert(lft)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
        }

        // 7. Create project
        let now = Utc::now();
        let project = Project {
            id: 0, // Will be set by database
            name: dto.name.trim().to_string(),
            identifier: Some(dto.identifier.trim().to_string()),
            description: dto
                .description
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
            homepage: dto
                .homepage
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
            is_public: dto.is_public,
            parent_id: dto.parent_id,
            status: PROJECT_STATUS_ACTIVE,
            lft: Some(lft),
            rgt: Some(rgt),
            inherit_members: dto.inherit_members,
            created_on: Some(now),
            updated_on: Some(now),
            default_version_id: None,
            default_assigned_to_id: None,
        };

        let created_project = self
            .project_repo
            .create(project)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 8. Add trackers - use specified trackers or default to all available
        let tracker_ids = match dto.tracker_ids {
            Some(ids) => ids,
            None => {
                let all_trackers = self
                    .tracker_repo
                    .find_all()
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;
                all_trackers.iter().map(|t| t.id).collect()
            }
        };
        for tracker_id in tracker_ids {
            self.project_repo
                .add_tracker(created_project.id, tracker_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
        }

        // 9. Inherit members if specified
        if dto.inherit_members {
            if let Some(parent) = parent {
                self.member_repo
                    .inherit_from_parent(created_project.id, parent.id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;
            }
        }

        // 10. Add creator as manager
        self.member_repo
            .add_manager(created_project.id, current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 11. Return response
        Ok(CreateProjectResponse {
            project: ProjectDetail::from_project(&created_project, None),
        })
    }

    /// Calculate nested set lft and rgt values for a new node
    fn calculate_nested_set_position(parent_rgt: Option<i32>, max_rgt: Option<i32>) -> (i32, i32) {
        match (parent_rgt, max_rgt) {
            // Inserting as child of existing project
            (Some(prgt), _) => (prgt, prgt + 1),
            // Inserting as root project when there are existing projects
            (None, Some(max)) => (max + 2, max + 3),
            // Inserting as first project ever
            (None, None) => (1, 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Project, Tracker};
    use crate::domain::repositories::RepositoryError;

    // Mock implementations for testing
    struct MockProjectRepository {
        projects: Vec<Project>,
        identifiers: Vec<String>,
        max_rgt: Option<i32>,
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

        async fn create(&self, mut project: Project) -> Result<Project, RepositoryError> {
            // Simulate auto-increment
            let max_id = self.projects.iter().map(|p| p.id).max().unwrap_or(0);
            project.id = max_id + 1;
            Ok(project)
        }

        async fn exists_by_identifier(&self, identifier: &str) -> Result<bool, RepositoryError> {
            Ok(self.identifiers.contains(&identifier.to_string()))
        }

        async fn get_max_rgt(&self) -> Result<Option<i32>, RepositoryError> {
            Ok(self.max_rgt)
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
            Ok(())
        }

        async fn clear_trackers(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockTrackerRepository {
        trackers: Vec<Tracker>,
    }

    #[async_trait::async_trait]
    impl TrackerRepository for MockTrackerRepository {
        async fn find_all(&self) -> Result<Vec<Tracker>, RepositoryError> {
            Ok(self.trackers.clone())
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<Tracker>, RepositoryError> {
            Ok(self.trackers.iter().find(|t| t.id == id).cloned())
        }

        async fn find_by_project(&self, _project_id: i32) -> Result<Vec<Tracker>, RepositoryError> {
            Ok(self.trackers.clone())
        }

        async fn create(
            &self,
            _tracker: &crate::domain::repositories::NewTracker,
        ) -> Result<Tracker, RepositoryError> {
            unimplemented!()
        }

        async fn update(&self, _tracker: &Tracker) -> Result<Tracker, RepositoryError> {
            unimplemented!()
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
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

        async fn set_projects(
            &self,
            _tracker_id: i32,
            _project_ids: &[i32],
        ) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockMemberRepository;

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

        async fn is_member(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
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

    fn create_test_project(id: i32, identifier: &str, rgt: Option<i32>) -> Project {
        Project {
            id,
            name: format!("Project {}", id),
            description: None,
            homepage: None,
            is_public: true,
            parent_id: None,
            created_on: Some(Utc::now()),
            updated_on: Some(Utc::now()),
            identifier: Some(identifier.to_string()),
            status: PROJECT_STATUS_ACTIVE,
            lft: Some(id * 2 - 1),
            rgt,
            inherit_members: false,
            default_version_id: None,
            default_assigned_to_id: None,
        }
    }

    #[tokio::test]
    async fn test_create_project_as_admin() {
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![],
            identifiers: vec![],
            max_rgt: None,
        });
        let member_repo = Arc::new(MockMemberRepository);

        let usecase = CreateProjectUseCase::new(project_repo, member_repo, Arc::new(MockTrackerRepository { trackers: vec![] }));
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateProjectDto {
            name: "New Project".to_string(),
            identifier: "new-project".to_string(),
            description: Some("A test project".to_string()),
            homepage: None,
            is_public: true,
            parent_id: None,
            inherit_members: false,
            tracker_ids: None,
        };

        let result = usecase.execute(dto, &current_user).await.unwrap();
        assert_eq!(result.project.name, "New Project");
        assert_eq!(result.project.identifier, "new-project");
    }

    #[tokio::test]
    async fn test_create_project_non_admin_forbidden() {
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![],
            identifiers: vec![],
            max_rgt: None,
        });
        let member_repo = Arc::new(MockMemberRepository);

        let usecase = CreateProjectUseCase::new(project_repo, member_repo, Arc::new(MockTrackerRepository { trackers: vec![] }));
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let dto = CreateProjectDto {
            name: "New Project".to_string(),
            identifier: "new-project".to_string(),
            description: None,
            homepage: None,
            is_public: true,
            parent_id: None,
            inherit_members: false,
            tracker_ids: None,
        };

        let result = usecase.execute(dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Forbidden(_)
        ));
    }

    #[tokio::test]
    async fn test_create_project_duplicate_identifier() {
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![],
            identifiers: vec!["existing-project".to_string()],
            max_rgt: None,
        });
        let member_repo = Arc::new(MockMemberRepository);

        let usecase = CreateProjectUseCase::new(project_repo, member_repo, Arc::new(MockTrackerRepository { trackers: vec![] }));
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateProjectDto {
            name: "New Project".to_string(),
            identifier: "existing-project".to_string(),
            description: None,
            homepage: None,
            is_public: true,
            parent_id: None,
            inherit_members: false,
            tracker_ids: None,
        };

        let result = usecase.execute(dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_create_project_invalid_identifier() {
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![],
            identifiers: vec![],
            max_rgt: None,
        });
        let member_repo = Arc::new(MockMemberRepository);

        let usecase = CreateProjectUseCase::new(project_repo, member_repo, Arc::new(MockTrackerRepository { trackers: vec![] }));
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        // Test identifier starting with digit
        let dto = CreateProjectDto {
            name: "New Project".to_string(),
            identifier: "123project".to_string(),
            description: None,
            homepage: None,
            is_public: true,
            parent_id: None,
            inherit_members: false,
            tracker_ids: None,
        };

        let result = usecase.execute(dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_create_project_parent_not_found() {
        let project_repo = Arc::new(MockProjectRepository {
            projects: vec![],
            identifiers: vec![],
            max_rgt: None,
        });
        let member_repo = Arc::new(MockMemberRepository);

        let usecase = CreateProjectUseCase::new(project_repo, member_repo, Arc::new(MockTrackerRepository { trackers: vec![] }));
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let dto = CreateProjectDto {
            name: "Subproject".to_string(),
            identifier: "subproject".to_string(),
            description: None,
            homepage: None,
            is_public: true,
            parent_id: Some(999),
            inherit_members: false,
            tracker_ids: None,
        };

        let result = usecase.execute(dto, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_calculate_nested_set_position_first_project() {
        let (lft, rgt) = CreateProjectUseCase::<MockProjectRepository, MockMemberRepository, MockTrackerRepository>::calculate_nested_set_position(None, None);
        assert_eq!(lft, 1);
        assert_eq!(rgt, 2);
    }

    #[tokio::test]
    async fn test_calculate_nested_set_position_after_existing() {
        let (lft, rgt) = CreateProjectUseCase::<MockProjectRepository, MockMemberRepository, MockTrackerRepository>::calculate_nested_set_position(None, Some(4));
        assert_eq!(lft, 6);
        assert_eq!(rgt, 7);
    }

    #[tokio::test]
    async fn test_calculate_nested_set_position_as_child() {
        let (lft, rgt) = CreateProjectUseCase::<MockProjectRepository, MockMemberRepository, MockTrackerRepository>::calculate_nested_set_position(Some(5), Some(10));
        assert_eq!(lft, 5);
        assert_eq!(rgt, 6);
    }
}
