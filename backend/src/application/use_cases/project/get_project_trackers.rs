use crate::application::errors::ApplicationError;
use crate::application::use_cases::{TrackerDefaultStatus, TrackerItem};
use crate::domain::entities::PROJECT_STATUS_ARCHIVED;
use crate::domain::repositories::{
    IssueStatusRepository, MemberRepository, ProjectRepository, TrackerRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response for project trackers endpoint
#[derive(Debug, Clone)]
pub struct ProjectTrackersResponse {
    pub trackers: Vec<TrackerItem>,
}

/// Use case for getting trackers associated with a project
pub struct GetProjectTrackersUseCase<
    P: ProjectRepository,
    M: MemberRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
> {
    project_repo: Arc<P>,
    member_repo: Arc<M>,
    tracker_repo: Arc<T>,
    status_repo: Arc<S>,
}

impl<P: ProjectRepository, M: MemberRepository, T: TrackerRepository, S: IssueStatusRepository>
    GetProjectTrackersUseCase<P, M, T, S>
{
    pub fn new(
        project_repo: Arc<P>,
        member_repo: Arc<M>,
        tracker_repo: Arc<T>,
        status_repo: Arc<S>,
    ) -> Self {
        Self {
            project_repo,
            member_repo,
            tracker_repo,
            status_repo,
        }
    }

    /// Execute the use case
    ///
    /// Visibility rules:
    /// - Admin users can see trackers for all projects
    /// - Regular users can see trackers for public projects + projects they are a member of
    /// - Archived projects are only visible to admins
    ///
    /// If a project has no trackers associated, returns an empty list.
    pub async fn execute(
        &self,
        project_id: i32,
        current_user: &CurrentUser,
    ) -> Result<ProjectTrackersResponse, ApplicationError> {
        // 1. Find project by ID
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await
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

        // 4. Get trackers associated with the project
        let trackers = self
            .tracker_repo
            .find_by_project(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 5. Build response with default status info
        let mut tracker_items = Vec::new();
        for tracker in trackers {
            // Get default status info
            let default_status = self
                .status_repo
                .find_by_id(tracker.default_status_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .map(|s| TrackerDefaultStatus {
                    id: s.id,
                    name: s.name,
                });

            tracker_items.push(TrackerItem::from_tracker(&tracker, default_status));
        }

        Ok(ProjectTrackersResponse {
            trackers: tracker_items,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{IssueStatus, Project, Tracker, PROJECT_STATUS_ACTIVE};
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
            unimplemented!()
        }

        async fn get_max_rgt(&self) -> Result<Option<i32>, RepositoryError> {
            unimplemented!()
        }

        async fn add_tracker(
            &self,
            _project_id: i32,
            _tracker_id: i32,
        ) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn update_nested_set_for_insert(&self, _lft: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn update(&self, _project: Project) -> Result<Project, RepositoryError> {
            unimplemented!()
        }

        async fn set_trackers(
            &self,
            _project_id: i32,
            _tracker_ids: &[i32],
        ) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn exists_by_identifier_excluding(
            &self,
            _identifier: &str,
            _exclude_project_id: i32,
        ) -> Result<bool, RepositoryError> {
            unimplemented!()
        }

        async fn find_children(&self, _project_id: i32) -> Result<Vec<Project>, RepositoryError> {
            unimplemented!()
        }

        async fn delete(&self, _project_id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn clear_trackers(&self, _project_id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
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
            unimplemented!()
        }

        async fn add_member_role(
            &self,
            _member_id: i32,
            _role_id: i32,
            _inherited_from: Option<i32>,
        ) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn add_manager(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn inherit_from_parent(
            &self,
            _project_id: i32,
            _parent_id: i32,
        ) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn delete_by_project(&self, _project_id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn find_by_id(
            &self,
            _member_id: i32,
        ) -> Result<Option<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            unimplemented!()
        }

        async fn find_by_project_and_user(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<Option<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            unimplemented!()
        }

        async fn clear_roles(&self, _member_id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn delete_by_id(&self, _member_id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }
    }

    struct MockTrackerRepository {
        project_trackers: std::collections::HashMap<i32, Vec<Tracker>>,
    }

    #[async_trait::async_trait]
    impl TrackerRepository for MockTrackerRepository {
        async fn find_all(&self) -> Result<Vec<Tracker>, RepositoryError> {
            Ok(vec![])
        }

        async fn find_by_id(&self, _id: i32) -> Result<Option<Tracker>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_project(&self, project_id: i32) -> Result<Vec<Tracker>, RepositoryError> {
            Ok(self
                .project_trackers
                .get(&project_id)
                .cloned()
                .unwrap_or_default())
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
            unimplemented!()
        }

        async fn exists_by_name_excluding(
            &self,
            _name: &str,
            _exclude_id: i32,
        ) -> Result<bool, RepositoryError> {
            unimplemented!()
        }

        async fn set_projects(
            &self,
            _tracker_id: i32,
            _project_ids: &[i32],
        ) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockIssueStatusRepository {
        statuses: Vec<IssueStatus>,
    }

    #[async_trait::async_trait]
    impl IssueStatusRepository for MockIssueStatusRepository {
        async fn find_all(&self) -> Result<Vec<IssueStatus>, RepositoryError> {
            Ok(self.statuses.clone())
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<IssueStatus>, RepositoryError> {
            Ok(self.statuses.iter().find(|s| s.id == id).cloned())
        }

        async fn find_default(&self) -> Result<Option<IssueStatus>, RepositoryError> {
            Ok(self.statuses.iter().find(|s| s.is_default).cloned())
        }

        async fn find_open(&self) -> Result<Vec<IssueStatus>, RepositoryError> {
            Ok(self
                .statuses
                .iter()
                .filter(|s| !s.is_closed)
                .cloned()
                .collect())
        }

        async fn find_closed(&self) -> Result<Vec<IssueStatus>, RepositoryError> {
            Ok(self
                .statuses
                .iter()
                .filter(|s| s.is_closed)
                .cloned()
                .collect())
        }

        async fn create(
            &self,
            _status: &crate::domain::repositories::NewIssueStatus,
        ) -> Result<IssueStatus, RepositoryError> {
            unimplemented!()
        }

        async fn update(&self, _status: &IssueStatus) -> Result<IssueStatus, RepositoryError> {
            unimplemented!()
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn exists_by_name(&self, _name: &str) -> Result<bool, RepositoryError> {
            unimplemented!()
        }

        async fn exists_by_name_excluding(
            &self,
            _name: &str,
            _exclude_id: i32,
        ) -> Result<bool, RepositoryError> {
            unimplemented!()
        }

        async fn clear_default(&self) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn count_issues_by_status(&self, _status_id: i32) -> Result<u64, RepositoryError> {
            Ok(0)
        }

        async fn reassign_issues_status(
            &self,
            _from_status_id: i32,
            _to_status_id: i32,
        ) -> Result<u64, RepositoryError> {
            Ok(0)
        }
    }

    fn create_test_project(id: i32, is_public: bool, status: i32) -> Project {
        Project {
            id,
            name: format!("Project {}", id),
            description: None,
            homepage: None,
            is_public,
            parent_id: None,
            created_on: Some(Utc::now()),
            updated_on: Some(Utc::now()),
            identifier: Some(format!("proj{}", id)),
            status,
            lft: None,
            rgt: None,
            inherit_members: false,
            default_version_id: None,
            default_assigned_to_id: None,
        }
    }

    fn create_test_tracker(id: i32, name: &str, default_status_id: i32) -> Tracker {
        Tracker {
            id,
            name: name.to_string(),
            position: Some(id),
            is_in_roadmap: true,
            fields_bits: None,
            default_status_id,
        }
    }

    fn create_test_status(id: i32, name: &str) -> IssueStatus {
        IssueStatus {
            id,
            name: name.to_string(),
            is_closed: false,
            position: Some(id),
            is_default: false,
            default_done_ratio: None,
        }
    }

    #[tokio::test]
    async fn test_get_project_trackers_as_admin() {
        let projects = vec![create_test_project(1, false, PROJECT_STATUS_ACTIVE)];
        let statuses = vec![create_test_status(1, "New")];
        let trackers = vec![
            create_test_tracker(1, "Bug", 1),
            create_test_tracker(2, "Feature", 1),
        ];
        let mut project_trackers = std::collections::HashMap::new();
        project_trackers.insert(1, trackers);

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let tracker_repo = Arc::new(MockTrackerRepository { project_trackers });
        let status_repo = Arc::new(MockIssueStatusRepository { statuses });

        let usecase =
            GetProjectTrackersUseCase::new(project_repo, member_repo, tracker_repo, status_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(1, &current_user).await.unwrap();
        assert_eq!(result.trackers.len(), 2);
        assert_eq!(result.trackers[0].name, "Bug");
        assert_eq!(result.trackers[1].name, "Feature");
    }

    #[tokio::test]
    async fn test_get_project_trackers_as_member() {
        let projects = vec![create_test_project(1, false, PROJECT_STATUS_ACTIVE)];
        let statuses = vec![create_test_status(1, "New")];
        let trackers = vec![create_test_tracker(1, "Bug", 1)];
        let mut project_trackers = std::collections::HashMap::new();
        project_trackers.insert(1, trackers);

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });
        let tracker_repo = Arc::new(MockTrackerRepository { project_trackers });
        let status_repo = Arc::new(MockIssueStatusRepository { statuses });

        let usecase =
            GetProjectTrackersUseCase::new(project_repo, member_repo, tracker_repo, status_repo);
        let current_user = CurrentUser {
            id: 2,
            login: "member".to_string(),
            admin: false,
        };

        let result = usecase.execute(1, &current_user).await.unwrap();
        assert_eq!(result.trackers.len(), 1);
        assert_eq!(result.trackers[0].name, "Bug");
    }

    #[tokio::test]
    async fn test_get_project_trackers_forbidden_for_non_member() {
        let projects = vec![create_test_project(1, false, PROJECT_STATUS_ACTIVE)];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let tracker_repo = Arc::new(MockTrackerRepository {
            project_trackers: std::collections::HashMap::new(),
        });
        let status_repo = Arc::new(MockIssueStatusRepository { statuses: vec![] });

        let usecase =
            GetProjectTrackersUseCase::new(project_repo, member_repo, tracker_repo, status_repo);
        let current_user = CurrentUser {
            id: 2,
            login: "stranger".to_string(),
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
    async fn test_get_project_trackers_not_found() {
        let projects = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let tracker_repo = Arc::new(MockTrackerRepository {
            project_trackers: std::collections::HashMap::new(),
        });
        let status_repo = Arc::new(MockIssueStatusRepository { statuses: vec![] });

        let usecase =
            GetProjectTrackersUseCase::new(project_repo, member_repo, tracker_repo, status_repo);
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
    async fn test_get_project_trackers_empty() {
        let projects = vec![create_test_project(1, true, PROJECT_STATUS_ACTIVE)];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let tracker_repo = Arc::new(MockTrackerRepository {
            project_trackers: std::collections::HashMap::new(),
        });
        let status_repo = Arc::new(MockIssueStatusRepository { statuses: vec![] });

        let usecase =
            GetProjectTrackersUseCase::new(project_repo, member_repo, tracker_repo, status_repo);
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let result = usecase.execute(1, &current_user).await.unwrap();
        assert_eq!(result.trackers.len(), 0);
    }
}
