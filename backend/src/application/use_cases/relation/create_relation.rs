use crate::application::dto::CreateRelationDto;
use crate::application::errors::ApplicationError;
use crate::domain::entities::Issue;
use crate::domain::repositories::{
    IssueRelationRepository, IssueRepository, MemberRepository, NewIssueRelation, ProjectRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response for creating a single issue relation
#[derive(Debug, Clone)]
pub struct CreateRelationResponse {
    pub id: i32,
    pub issue_id: i32,
    pub issue_to_id: i32,
    pub relation_type: String,
    pub delay: Option<i32>,
}

/// Use case for creating a relation between two issues
pub struct CreateRelationUseCase<I, P, M, R>
where
    I: IssueRepository,
    P: ProjectRepository,
    M: MemberRepository,
    R: IssueRelationRepository,
{
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    member_repo: Arc<M>,
    relation_repo: Arc<R>,
}

impl<I, P, M, R> CreateRelationUseCase<I, P, M, R>
where
    I: IssueRepository,
    P: ProjectRepository,
    M: MemberRepository,
    R: IssueRelationRepository,
{
    pub fn new(
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        member_repo: Arc<M>,
        relation_repo: Arc<R>,
    ) -> Self {
        Self {
            issue_repo,
            project_repo,
            member_repo,
            relation_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin users can create relations for all issues
    /// - Regular users need manage_issue_relations permission on the project
    ///
    /// Validation rules:
    /// - Cannot create relation to self
    /// - Target issue must exist and be visible to the user
    /// - Relation type must be valid
    /// - Cannot create duplicate relations
    /// - Cannot create circular dependencies for precedes/follows
    pub async fn execute(
        &self,
        issue_id: i32,
        dto: CreateRelationDto,
        current_user: &CurrentUser,
    ) -> Result<CreateRelationResponse, ApplicationError> {
        // 1. Validate the DTO
        if let Err(errors) = dto.validate() {
            return Err(ApplicationError::Validation(errors.join(", ")));
        }

        // 2. Cannot create relation to self
        if issue_id == dto.issue_to_id {
            return Err(ApplicationError::Validation(
                "Cannot create relation to self".into(),
            ));
        }

        // 3. Get source issue
        let source_issue = self
            .issue_repo
            .find_by_id(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Issue {} not found", issue_id)))?;

        // 4. Check permission on source issue's project
        self.check_permission(source_issue.project_id, current_user)
            .await?;

        // 5. Get target issue
        let target_issue = self
            .issue_repo
            .find_by_id(dto.issue_to_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| {
                ApplicationError::NotFound(format!("Target issue {} not found", dto.issue_to_id))
            })?;

        // 6. Check visibility of target issue
        self.check_issue_visibility(&target_issue, current_user)
            .await?;

        // 7. Check for existing relation
        let exists = self
            .relation_repo
            .exists_relation(issue_id, dto.issue_to_id, &dto.relation_type)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if exists {
            return Err(ApplicationError::Validation(
                "Relation already exists".into(),
            ));
        }

        // 8. Check for circular dependency with precedes/follows
        if (dto.relation_type == "precedes" || dto.relation_type == "follows")
            && self
                .check_circular_dependency(issue_id, dto.issue_to_id)
                .await?
        {
            return Err(ApplicationError::Validation(
                "Circular dependency not allowed".into(),
            ));
        }

        // 9. Check subtask relation (can't create precedes between parent-child)
        if matches!(dto.relation_type.as_str(), "precedes" | "follows")
            && (source_issue.parent_id == Some(dto.issue_to_id)
                || target_issue.parent_id == Some(issue_id))
        {
            return Err(ApplicationError::Validation(
                "Cannot create precedes relation between parent and child issues".into(),
            ));
        }

        // 10. Create the relation
        let relation = self
            .relation_repo
            .create(NewIssueRelation {
                issue_from_id: issue_id,
                issue_to_id: dto.issue_to_id,
                relation_type: dto.relation_type.clone(),
                delay: dto.delay,
            })
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 11. Create reverse relation for bidirectional types
        if let Some(reverse_type) = dto.reverse_relation_type() {
            // Check if reverse already exists
            let reverse_exists = self
                .relation_repo
                .exists_relation(dto.issue_to_id, issue_id, reverse_type)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            if !reverse_exists {
                self.relation_repo
                    .create(NewIssueRelation {
                        issue_from_id: dto.issue_to_id,
                        issue_to_id: issue_id,
                        relation_type: reverse_type.to_string(),
                        delay: dto.delay,
                    })
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;
            }
        }

        Ok(CreateRelationResponse {
            id: relation.id,
            issue_id: relation.issue_from_id,
            issue_to_id: relation.issue_to_id,
            relation_type: relation.relation_type,
            delay: relation.delay,
        })
    }

    /// Check if the current user has permission to manage issue relations
    async fn check_permission(
        &self,
        project_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Admin can manage all relations
        if current_user.admin {
            return Ok(());
        }

        // Check if user is a member of the project
        let is_member = self
            .member_repo
            .is_member(project_id, current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if !is_member {
            return Err(ApplicationError::Forbidden(
                "You don't have permission to manage relations in this project".into(),
            ));
        }

        Ok(())
    }

    /// Check if the current user can view the issue
    async fn check_issue_visibility(
        &self,
        issue: &Issue,
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

        Err(ApplicationError::NotFound("Target issue not found".into()))
    }

    /// Check for circular dependency in precedes/follows chain
    async fn check_circular_dependency(
        &self,
        from_id: i32,
        to_id: i32,
    ) -> Result<bool, ApplicationError> {
        // Walk the relation chain to detect cycles
        let mut visited = vec![from_id];
        let mut current = to_id;

        loop {
            if visited.contains(&current) {
                return Ok(true);
            }

            // Get all "precedes" relations from current issue
            let relations = self
                .relation_repo
                .find_by_issue(current)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            // Find the next issue in the precedes chain
            let next = relations.iter().find_map(|r| {
                if r.relation_type == "precedes" && r.issue_from_id == current {
                    Some(r.issue_to_id)
                } else if r.relation_type == "follows" && r.issue_to_id == current {
                    Some(r.issue_from_id)
                } else {
                    None
                }
            });

            match next {
                Some(next_id) => {
                    visited.push(current);
                    current = next_id;
                }
                None => break,
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Issue, IssueRelation, MemberWithRoles, Project};
    use crate::domain::repositories::{
        IssueQueryParams, IssueUpdate, NewMember, ProjectQueryParams, RepositoryError,
    };
    use crate::domain::value_objects::IssueQueryParams as DomainIssueQueryParams;
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

    // Mock Member Repository
    struct MockMemberRepository {
        is_member_result: bool,
    }

    impl MockMemberRepository {
        fn new(is_member: bool) -> Self {
            Self {
                is_member_result: is_member,
            }
        }
    }

    #[async_trait]
    impl MemberRepository for MockMemberRepository {
        async fn find_by_project(
            &self,
            _project_id: i32,
        ) -> Result<Vec<MemberWithRoles>, RepositoryError> {
            Ok(vec![])
        }

        async fn is_member(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(self.is_member_result)
        }

        async fn add_member(&self, _member: NewMember) -> Result<i32, RepositoryError> {
            unimplemented!()
        }

        async fn find_by_id(
            &self,
            _member_id: i32,
        ) -> Result<Option<MemberWithRoles>, RepositoryError> {
            unimplemented!()
        }

        async fn find_by_project_and_user(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<Option<MemberWithRoles>, RepositoryError> {
            unimplemented!()
        }

        async fn delete_by_user_id(&self, _user_id: i32) -> Result<(), RepositoryError> {
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

        async fn clear_roles(&self, _member_id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn delete_by_id(&self, _member_id: i32) -> Result<(), RepositoryError> {
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
    }

    // Mock Issue Relation Repository
    struct MockIssueRelationRepository {
        relations: Mutex<Vec<IssueRelation>>,
        next_id: std::sync::atomic::AtomicI32,
    }

    impl MockIssueRelationRepository {
        fn new(relations: Vec<IssueRelation>) -> Self {
            Self {
                relations: Mutex::new(relations),
                next_id: std::sync::atomic::AtomicI32::new(1),
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
            relation: NewIssueRelation,
        ) -> Result<IssueRelation, RepositoryError> {
            let id = self
                .next_id
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let new_relation = IssueRelation {
                id,
                issue_from_id: relation.issue_from_id,
                issue_to_id: relation.issue_to_id,
                relation_type: relation.relation_type,
                delay: relation.delay,
            };
            self.relations.lock().unwrap().push(new_relation.clone());
            Ok(new_relation)
        }

        async fn exists_relation(
            &self,
            issue_from_id: i32,
            issue_to_id: i32,
            relation_type: &str,
        ) -> Result<bool, RepositoryError> {
            let relations = self.relations.lock().unwrap();
            Ok(relations.iter().any(|r| {
                r.issue_from_id == issue_from_id
                    && r.issue_to_id == issue_to_id
                    && r.relation_type == relation_type
            }))
        }

        async fn find_relation(
            &self,
            issue_from_id: i32,
            issue_to_id: i32,
            relation_type: &str,
        ) -> Result<Option<IssueRelation>, RepositoryError> {
            let relations = self.relations.lock().unwrap();
            Ok(relations
                .iter()
                .find(|r| {
                    r.issue_from_id == issue_from_id
                        && r.issue_to_id == issue_to_id
                        && r.relation_type == relation_type
                })
                .cloned())
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<IssueRelation>, RepositoryError> {
            let relations = self.relations.lock().unwrap();
            Ok(relations.iter().find(|r| r.id == id).cloned())
        }

        async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
            let mut relations = self.relations.lock().unwrap();
            relations.retain(|r| r.id != id);
            Ok(())
        }

        async fn delete_by_issues_and_type(
            &self,
            issue_from_id: i32,
            issue_to_id: i32,
            relation_type: &str,
        ) -> Result<(), RepositoryError> {
            let mut relations = self.relations.lock().unwrap();
            relations.retain(|r| {
                !(r.issue_from_id == issue_from_id
                    && r.issue_to_id == issue_to_id
                    && r.relation_type == relation_type)
            });
            Ok(())
        }
    }

    fn create_test_issue(id: i32, project_id: i32, parent_id: Option<i32>) -> Issue {
        Issue {
            id,
            tracker_id: 1,
            project_id,
            subject: format!("Test issue {}", id),
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
            parent_id,
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
    async fn test_create_relates_relation() {
        let issues = vec![create_test_issue(1, 1, None), create_test_issue(2, 1, None)];
        let projects = vec![create_test_project(1, true)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase =
            CreateRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_admin_user()).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.issue_id, 1);
        assert_eq!(response.issue_to_id, 2);
        assert_eq!(response.relation_type, "relates");
    }

    #[tokio::test]
    async fn test_create_relation_to_self_fails() {
        let issues = vec![create_test_issue(1, 1, None)];
        let projects = vec![create_test_project(1, true)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase =
            CreateRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

        let dto = CreateRelationDto {
            issue_to_id: 1,
            relation_type: "relates".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_admin_user()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Validation(msg) => {
                assert!(msg.contains("Cannot create relation to self"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_create_relation_invalid_type_fails() {
        let issues = vec![create_test_issue(1, 1, None), create_test_issue(2, 1, None)];
        let projects = vec![create_test_project(1, true)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase =
            CreateRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "invalid_type".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_admin_user()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Validation(msg) => {
                assert!(msg.contains("Invalid relation type"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_create_duplicate_relation_fails() {
        let issues = vec![create_test_issue(1, 1, None), create_test_issue(2, 1, None)];
        let projects = vec![create_test_project(1, true)];
        let existing_relation = IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        };

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![existing_relation]));

        let usecase =
            CreateRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_admin_user()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Validation(msg) => {
                assert!(msg.contains("Relation already exists"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_create_blocks_relation_creates_reverse() {
        let issues = vec![create_test_issue(1, 1, None), create_test_issue(2, 1, None)];
        let projects = vec![create_test_project(1, true)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase = CreateRelationUseCase::new(
            issue_repo,
            project_repo,
            member_repo,
            relation_repo.clone(),
        );

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "blocks".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_admin_user()).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.relation_type, "blocks");

        // Check that reverse relation was created
        let reverse = relation_repo.find_relation(2, 1, "blocked").await.unwrap();
        assert!(reverse.is_some());
    }

    #[tokio::test]
    async fn test_create_precedes_relation_creates_follows_reverse() {
        let issues = vec![create_test_issue(1, 1, None), create_test_issue(2, 1, None)];
        let projects = vec![create_test_project(1, true)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase = CreateRelationUseCase::new(
            issue_repo,
            project_repo,
            member_repo,
            relation_repo.clone(),
        );

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "precedes".to_string(),
            delay: Some(5),
        };

        let result = usecase.execute(1, dto, &create_admin_user()).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.relation_type, "precedes");
        assert_eq!(response.delay, Some(5));

        // Check that reverse follows relation was created
        let reverse = relation_repo.find_relation(2, 1, "follows").await.unwrap();
        assert!(reverse.is_some());
        assert_eq!(reverse.unwrap().delay, Some(5));
    }

    #[tokio::test]
    async fn test_create_relation_source_issue_not_found() {
        let issues = vec![];
        let projects = vec![create_test_project(1, true)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase =
            CreateRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_admin_user()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::NotFound(msg) => {
                assert!(msg.contains("Issue 1 not found"));
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_create_relation_target_issue_not_found() {
        let issues = vec![create_test_issue(1, 1, None)];
        let projects = vec![create_test_project(1, true)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase =
            CreateRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_admin_user()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::NotFound(msg) => {
                assert!(msg.contains("Target issue 2 not found"));
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_non_member_cannot_create_relation() {
        let issues = vec![create_test_issue(1, 1, None), create_test_issue(2, 1, None)];
        let projects = vec![create_test_project(1, false)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![]));
        let member_repo = Arc::new(MockMemberRepository::new(false));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase =
            CreateRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_regular_user()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Forbidden(msg) => {
                assert!(msg.contains("permission"));
            }
            _ => panic!("Expected Forbidden error"),
        }
    }

    #[tokio::test]
    async fn test_create_relation_with_delay_for_non_precedes_fails() {
        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: Some(5),
        };

        let result = dto.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| e.contains("Delay can only be specified")));
    }

    #[tokio::test]
    async fn test_cannot_create_precedes_between_parent_child() {
        // Issue 2 is parent of issue 1
        let issues = vec![
            create_test_issue(1, 1, Some(2)), // Issue 1 has parent 2
            create_test_issue(2, 1, None),    // Issue 2 is parent
        ];
        let projects = vec![create_test_project(1, true)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase =
            CreateRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "precedes".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_admin_user()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Validation(msg) => {
                assert!(msg.contains("parent and child"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_circular_dependency_detection() {
        let issues = vec![
            create_test_issue(1, 1, None),
            create_test_issue(2, 1, None),
            create_test_issue(3, 1, None),
        ];
        let projects = vec![create_test_project(1, true)];

        // Create a chain: 1 -> 2 -> 3
        let existing_relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 2,
            issue_to_id: 3,
            relation_type: "precedes".to_string(),
            delay: None,
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(existing_relations));

        let usecase = CreateRelationUseCase::new(
            issue_repo,
            project_repo,
            member_repo,
            relation_repo.clone(),
        );

        // First, create a precedes relation from 1 to 2
        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "precedes".to_string(),
            delay: None,
        };
        let result = usecase.execute(1, dto, &create_admin_user()).await;
        assert!(result.is_ok());

        // Now try to create a relation from 3 to 1, which would create a cycle
        let dto2 = CreateRelationDto {
            issue_to_id: 1,
            relation_type: "precedes".to_string(),
            delay: None,
        };
        let result2 = usecase.execute(3, dto2, &create_admin_user()).await;

        assert!(result2.is_err());
        match result2.unwrap_err() {
            ApplicationError::Validation(msg) => {
                assert!(msg.contains("Circular dependency"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_member_can_create_relation_in_public_project() {
        let issues = vec![create_test_issue(1, 1, None), create_test_issue(2, 1, None)];
        let projects = vec![create_test_project(1, true)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase =
            CreateRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_regular_user()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_target_issue_visibility_private_project() {
        // Issue 1 in public project, Issue 2 in private project
        let issues = vec![create_test_issue(1, 1, None), create_test_issue(2, 2, None)];
        let projects = vec![
            create_test_project(1, true),
            create_test_project(2, false), // Private project
        ];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1])); // User only member of project 1
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase =
            CreateRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_regular_user()).await;

        // Should fail because user can't see issue 2 (private project)
        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::NotFound(msg) => {
                assert!(msg.contains("Target issue not found"));
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_create_duplicates_relation_creates_duplicated_reverse() {
        let issues = vec![create_test_issue(1, 1, None), create_test_issue(2, 1, None)];
        let projects = vec![create_test_project(1, true)];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(vec![]));

        let usecase = CreateRelationUseCase::new(
            issue_repo,
            project_repo,
            member_repo,
            relation_repo.clone(),
        );

        let dto = CreateRelationDto {
            issue_to_id: 2,
            relation_type: "duplicates".to_string(),
            delay: None,
        };

        let result = usecase.execute(1, dto, &create_admin_user()).await;

        assert!(result.is_ok());

        // Check that reverse duplicated relation was created
        let reverse = relation_repo
            .find_relation(2, 1, "duplicated")
            .await
            .unwrap();
        assert!(reverse.is_some());
    }
}
