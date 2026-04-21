use crate::application::errors::ApplicationError;
use crate::domain::repositories::{IssueRelationRepository, IssueRepository, ProjectRepository};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Summary of an issue for relation details
#[derive(Debug, Clone)]
pub struct IssueSummary {
    pub id: i32,
    pub subject: String,
}

/// Response for a single issue relation with details
#[derive(Debug, Clone)]
pub struct RelationDetail {
    pub id: i32,
    pub issue_from_id: i32,
    pub issue_to_id: i32,
    pub relation_type: String,
    pub delay: Option<i32>,
    pub issue_from: IssueSummary,
    pub issue_to: IssueSummary,
}

/// Response for the get relation endpoint
#[derive(Debug, Clone)]
pub struct GetRelationResponse {
    pub relation: RelationDetail,
}

/// Use case for getting a single issue relation by ID
pub struct GetRelationUseCase<I, P, R>
where
    I: IssueRepository,
    P: ProjectRepository,
    R: IssueRelationRepository,
{
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    relation_repo: Arc<R>,
}

impl<I, P, R> GetRelationUseCase<I, P, R>
where
    I: IssueRepository,
    P: ProjectRepository,
    R: IssueRelationRepository,
{
    pub fn new(issue_repo: Arc<I>, project_repo: Arc<P>, relation_repo: Arc<R>) -> Self {
        Self {
            issue_repo,
            project_repo,
            relation_repo,
        }
    }

    /// Execute the use case
    ///
    /// Visibility rules:
    /// - Admin users can view relations of all issues
    /// - Regular users can view relations of issues in public projects
    /// - Regular users can view relations of issues in projects they are members of
    /// - User must have view permission on BOTH related issues
    pub async fn execute(
        &self,
        relation_id: i32,
        current_user: &CurrentUser,
    ) -> Result<GetRelationResponse, ApplicationError> {
        // 1. Get the relation by ID
        let relation = self
            .relation_repo
            .find_by_id(relation_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Relation not found".into()))?;

        // 2. Get source issue
        let source_issue = self
            .issue_repo
            .find_by_id(relation.issue_from_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Issue {} not found", relation.issue_from_id))
            })?;

        // 3. Get target issue
        let target_issue = self
            .issue_repo
            .find_by_id(relation.issue_to_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Issue {} not found", relation.issue_to_id))
            })?;

        // 4. Check visibility on both issues
        self.check_issue_visibility(&source_issue, current_user)
            .await?;
        self.check_issue_visibility(&target_issue, current_user)
            .await?;

        // 5. Build response
        Ok(GetRelationResponse {
            relation: RelationDetail {
                id: relation.id,
                issue_from_id: relation.issue_from_id,
                issue_to_id: relation.issue_to_id,
                relation_type: relation.relation_type,
                delay: relation.delay,
                issue_from: IssueSummary {
                    id: source_issue.id,
                    subject: source_issue.subject,
                },
                issue_to: IssueSummary {
                    id: target_issue.id,
                    subject: target_issue.subject,
                },
            },
        })
    }

    /// Check if the current user can view the issue
    async fn check_issue_visibility(
        &self,
        issue: &crate::domain::entities::Issue,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Admin can see all issues
        if current_user.admin {
            return Ok(());
        }

        // Get the project
        let project = self
            .project_repo
            .find_by_id(issue.project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Project {} not found", issue.project_id))
            })?;

        // Public projects are visible to all logged-in users
        if project.is_public {
            return Ok(());
        }

        // Check if user is a member of the project
        let member_project_ids = self
            .project_repo
            .find_project_ids_by_user_membership(current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if member_project_ids.contains(&issue.project_id) {
            return Ok(());
        }

        Err(ApplicationError::Forbidden(
            "You don't have permission to view this relation".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Issue, IssueRelation, Project};
    use crate::domain::repositories::{
        IssueQueryParams, IssueUpdate, ProjectQueryParams, RepositoryError,
    };
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Mock Issue Repository
    struct MockIssueRepository {
        issues: Mutex<HashMap<i32, Issue>>,
    }

    impl MockIssueRepository {
        fn new(issues: Vec<Issue>) -> Self {
            let map = issues.into_iter().map(|i| (i.id, i)).collect();
            Self {
                issues: Mutex::new(map),
            }
        }
    }

    #[async_trait]
    impl IssueRepository for MockIssueRepository {
        async fn find_by_id(&self, id: i32) -> Result<Option<Issue>, RepositoryError> {
            Ok(self.issues.lock().unwrap().get(&id).cloned())
        }

        async fn find_by_project(&self, _project_id: i32) -> Result<Vec<Issue>, RepositoryError> {
            Ok(vec![])
        }

        async fn create(
            &self,
            _issue: crate::domain::repositories::NewIssue,
        ) -> Result<Issue, RepositoryError> {
            unimplemented!()
        }

        async fn update(&self, _id: i32, _update: IssueUpdate) -> Result<Issue, RepositoryError> {
            unimplemented!()
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn find_all(&self, _params: IssueQueryParams) -> Result<Vec<Issue>, RepositoryError> {
            Ok(vec![])
        }

        async fn count(&self, _params: &IssueQueryParams) -> Result<u32, RepositoryError> {
            Ok(0)
        }

        async fn delete_by_project(&self, _project_id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn clear_assignee_in_project(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn find_children(&self, _parent_id: i32) -> Result<Vec<Issue>, RepositoryError> {
            Ok(vec![])
        }
    }

    // Mock Project Repository
    struct MockProjectRepository {
        projects: Mutex<HashMap<i32, Project>>,
        member_project_ids: Vec<i32>,
    }

    impl MockProjectRepository {
        fn new(projects: Vec<Project>, member_project_ids: Vec<i32>) -> Self {
            let map = projects.into_iter().map(|p| (p.id, p)).collect();
            Self {
                projects: Mutex::new(map),
                member_project_ids,
            }
        }
    }

    #[async_trait]
    impl ProjectRepository for MockProjectRepository {
        async fn find_by_id(&self, id: i32) -> Result<Option<Project>, RepositoryError> {
            Ok(self.projects.lock().unwrap().get(&id).cloned())
        }

        async fn find_all(
            &self,
            _params: ProjectQueryParams,
        ) -> Result<Vec<Project>, RepositoryError> {
            Ok(vec![])
        }

        async fn count(&self, _params: &ProjectQueryParams) -> Result<u32, RepositoryError> {
            Ok(0)
        }

        async fn create(&self, _project: Project) -> Result<Project, RepositoryError> {
            unimplemented!()
        }

        async fn update(&self, _project: Project) -> Result<Project, RepositoryError> {
            unimplemented!()
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn find_by_identifier(
            &self,
            _identifier: &str,
        ) -> Result<Option<Project>, RepositoryError> {
            unimplemented!()
        }

        async fn find_visible_for_user(
            &self,
            _user_id: i32,
            _params: ProjectQueryParams,
        ) -> Result<Vec<Project>, RepositoryError> {
            Ok(vec![])
        }

        async fn count_visible_for_user(
            &self,
            _user_id: i32,
            _params: &ProjectQueryParams,
        ) -> Result<u32, RepositoryError> {
            Ok(0)
        }

        async fn find_project_ids_by_user_membership(
            &self,
            _user_id: i32,
        ) -> Result<Vec<i32>, RepositoryError> {
            Ok(self.member_project_ids.clone())
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
            Ok(vec![])
        }

        async fn clear_trackers(&self, _project_id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }
    }

    // Mock Issue Relation Repository
    struct MockIssueRelationRepository {
        relations: Mutex<Vec<IssueRelation>>,
    }

    impl MockIssueRelationRepository {
        fn new(relations: Vec<IssueRelation>) -> Self {
            Self {
                relations: Mutex::new(relations),
            }
        }
    }

    #[async_trait]
    impl IssueRelationRepository for MockIssueRelationRepository {
        async fn find_by_issue(
            &self,
            issue_id: i32,
        ) -> Result<Vec<IssueRelation>, RepositoryError> {
            let relations = self.relations.lock().unwrap();
            Ok(relations
                .iter()
                .filter(|r| r.issue_from_id == issue_id || r.issue_to_id == issue_id)
                .cloned()
                .collect())
        }

        async fn delete_by_issue(&self, _issue_id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn create(
            &self,
            _relation: crate::domain::repositories::NewIssueRelation,
        ) -> Result<IssueRelation, RepositoryError> {
            unimplemented!()
        }

        async fn exists_relation(
            &self,
            _issue_from_id: i32,
            _issue_to_id: i32,
            _relation_type: &str,
        ) -> Result<bool, RepositoryError> {
            unimplemented!()
        }

        async fn find_relation(
            &self,
            _issue_from_id: i32,
            _issue_to_id: i32,
            _relation_type: &str,
        ) -> Result<Option<IssueRelation>, RepositoryError> {
            unimplemented!()
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<IssueRelation>, RepositoryError> {
            let relations = self.relations.lock().unwrap();
            Ok(relations.iter().find(|r| r.id == id).cloned())
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn delete_by_issues_and_type(
            &self,
            _issue_from_id: i32,
            _issue_to_id: i32,
            _relation_type: &str,
        ) -> Result<(), RepositoryError> {
            unimplemented!()
        }
    }

    fn create_test_issue(id: i32, project_id: i32, subject: &str) -> Issue {
        Issue {
            id,
            tracker_id: 1,
            project_id,
            subject: subject.to_string(),
            description: None,
            due_date: None,
            category_id: None,
            status_id: 1,
            assigned_to_id: None,
            priority_id: 1,
            fixed_version_id: None,
            author_id: 1,
            lock_version: 0,
            created_on: None,
            updated_on: None,
            start_date: None,
            done_ratio: 0,
            estimated_hours: None,
            parent_id: None,
            root_id: None,
            lft: None,
            rgt: None,
            is_private: false,
            closed_on: None,
        }
    }

    fn create_test_project(id: i32, is_public: bool) -> Project {
        Project {
            id,
            name: format!("Test project {}", id),
            description: None,
            homepage: None,
            is_public,
            parent_id: None,
            created_on: None,
            updated_on: None,
            identifier: Some(format!("project-{}", id)),
            status: 1,
            lft: None,
            rgt: None,
            inherit_members: false,
            default_version_id: None,
            default_assigned_to_id: None,
        }
    }

    fn create_admin_user() -> CurrentUser {
        CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        }
    }

    fn create_regular_user() -> CurrentUser {
        CurrentUser {
            id: 2,
            login: "user".to_string(),
            admin: false,
        }
    }

    #[tokio::test]
    async fn test_get_relation_success() {
        let issues = vec![
            create_test_issue(1, 1, "First issue"),
            create_test_issue(2, 1, "Second issue"),
        ];
        let projects = vec![create_test_project(1, true)];
        let relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = GetRelationUseCase::new(issue_repo, project_repo, relation_repo);

        let result = usecase.execute(1, &create_admin_user()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.relation.id, 1);
        assert_eq!(response.relation.issue_from_id, 1);
        assert_eq!(response.relation.issue_to_id, 2);
        assert_eq!(response.relation.relation_type, "relates");
        assert_eq!(response.relation.issue_from.subject, "First issue");
        assert_eq!(response.relation.issue_to.subject, "Second issue");
    }

    #[tokio::test]
    async fn test_get_relation_with_delay() {
        let issues = vec![
            create_test_issue(1, 1, "First issue"),
            create_test_issue(2, 1, "Second issue"),
        ];
        let projects = vec![create_test_project(1, true)];
        let relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "precedes".to_string(),
            delay: Some(5),
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = GetRelationUseCase::new(issue_repo, project_repo, relation_repo);

        let result = usecase.execute(1, &create_admin_user()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.relation.delay, Some(5));
    }

    #[tokio::test]
    async fn test_get_relation_not_found() {
        let issues = vec![
            create_test_issue(1, 1, "First issue"),
            create_test_issue(2, 1, "Second issue"),
        ];
        let projects = vec![create_test_project(1, true)];
        let relations: Vec<IssueRelation> = vec![];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = GetRelationUseCase::new(issue_repo, project_repo, relation_repo);

        let result = usecase.execute(999, &create_admin_user()).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::NotFound(msg) => {
                assert!(msg.contains("Relation not found"));
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_relation_public_project_visible_to_all() {
        let issues = vec![
            create_test_issue(1, 1, "First issue"),
            create_test_issue(2, 1, "Second issue"),
        ];
        let projects = vec![create_test_project(1, true)]; // Public project
        let relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![])); // Not a member
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = GetRelationUseCase::new(issue_repo, project_repo, relation_repo);

        // Regular user should be able to view relation in public project
        let result = usecase.execute(1, &create_regular_user()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_relation_private_project_member_can_view() {
        let issues = vec![
            create_test_issue(1, 1, "First issue"),
            create_test_issue(2, 1, "Second issue"),
        ];
        let projects = vec![create_test_project(1, false)]; // Private project
        let relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1])); // Member of project
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = GetRelationUseCase::new(issue_repo, project_repo, relation_repo);

        // Member should be able to view relation
        let result = usecase.execute(1, &create_regular_user()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_relation_private_project_non_member_denied() {
        let issues = vec![
            create_test_issue(1, 1, "First issue"),
            create_test_issue(2, 1, "Second issue"),
        ];
        let projects = vec![create_test_project(1, false)]; // Private project
        let relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![])); // Not a member
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = GetRelationUseCase::new(issue_repo, project_repo, relation_repo);

        // Non-member should not be able to view relation
        let result = usecase.execute(1, &create_regular_user()).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Forbidden(msg) => {
                assert!(msg.contains("permission"));
            }
            _ => panic!("Expected Forbidden error"),
        }
    }

    #[tokio::test]
    async fn test_get_relation_cross_project_need_permission_on_both() {
        // Issue 1 is in public project
        // Issue 2 is in private project
        let issues = vec![
            create_test_issue(1, 1, "First issue"),
            create_test_issue(2, 2, "Second issue"),
        ];
        let projects = vec![
            create_test_project(1, true),  // Public
            create_test_project(2, false), // Private
        ];
        let relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![2])); // Member of private project 2
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = GetRelationUseCase::new(issue_repo, project_repo, relation_repo);

        // Should be able to view because:
        // - Issue 1 is in public project (visible to all)
        // - Issue 2 is in private project 2 (user is a member)
        let result = usecase.execute(1, &create_regular_user()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_relation_cross_project_denied_on_one() {
        // Issue 1 is in public project
        // Issue 2 is in private project (user is not a member)
        let issues = vec![
            create_test_issue(1, 1, "First issue"),
            create_test_issue(2, 2, "Second issue"),
        ];
        let projects = vec![
            create_test_project(1, true),  // Public
            create_test_project(2, false), // Private
        ];
        let relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![])); // Not a member of any
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = GetRelationUseCase::new(issue_repo, project_repo, relation_repo);

        // Should be denied because user can't see issue 2
        let result = usecase.execute(1, &create_regular_user()).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Forbidden(msg) => {
                assert!(msg.contains("permission"));
            }
            _ => panic!("Expected Forbidden error"),
        }
    }
}
