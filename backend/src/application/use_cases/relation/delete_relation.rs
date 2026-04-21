use crate::application::errors::ApplicationError;
use crate::domain::repositories::{
    IssueRelationRepository, IssueRepository, MemberRepository, ProjectRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Use case for deleting an issue relation
pub struct DeleteRelationUseCase<I, P, M, R>
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

impl<I, P, M, R> DeleteRelationUseCase<I, P, M, R>
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
    /// - Admin users can delete any relation
    /// - Regular users need manage_issue_relations permission on at least one
    ///   of the related issues' projects
    ///
    /// Bidirectional deletion:
    /// - When deleting a relation, the reverse relation is also deleted
    ///   (e.g., deleting "blocks" also deletes the corresponding "blocked")
    pub async fn execute(
        &self,
        relation_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
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

        // 4. Check permission on at least one of the issues' projects
        let can_manage_source = self
            .check_permission(source_issue.project_id, current_user)
            .await?;
        let can_manage_target = self
            .check_permission(target_issue.project_id, current_user)
            .await?;

        if !can_manage_source && !can_manage_target {
            return Err(ApplicationError::Forbidden(
                "You don't have permission to delete this relation".into(),
            ));
        }

        // 5. Delete the relation
        self.relation_repo
            .delete(relation_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 6. Delete the reverse relation for bidirectional types
        if let Some(reverse_type) = Self::get_reverse_type(&relation.relation_type) {
            self.relation_repo
                .delete_by_issues_and_type(
                    relation.issue_to_id,
                    relation.issue_from_id,
                    reverse_type,
                )
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
        }

        Ok(())
    }

    /// Check if the current user has permission to manage issue relations
    async fn check_permission(
        &self,
        project_id: i32,
        current_user: &CurrentUser,
    ) -> Result<bool, ApplicationError> {
        // Admin can manage all relations
        if current_user.admin {
            return Ok(true);
        }

        // Check if user is a member of the project
        let is_member = self
            .member_repo
            .is_member(project_id, current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(is_member)
    }

    /// Get the reverse relation type for bidirectional relations
    fn get_reverse_type(relation_type: &str) -> Option<&'static str> {
        match relation_type {
            "relates" => Some("relates"),
            "duplicates" => Some("duplicated"),
            "duplicated" => Some("duplicates"),
            "blocks" => Some("blocked"),
            "blocked" => Some("blocks"),
            "precedes" => Some("follows"),
            "follows" => Some("precedes"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Issue, IssueRelation, MemberWithRoles, Project};
    use crate::domain::repositories::{
        IssueQueryParams, IssueUpdate, NewIssueRelation, NewMember, ProjectQueryParams,
        RepositoryError,
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
            _relation: NewIssueRelation,
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

    fn create_test_issue(id: i32, project_id: i32) -> Issue {
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
            parent_id: None,
            root_id: None,
            lft: None,
            rgt: None,
            is_private: false,
            closed_on: None,
        }
    }

    fn create_test_project(id: i32, _is_public: bool) -> Project {
        Project {
            id,
            name: format!("Test project {}", id),
            description: None,
            homepage: None,
            is_public: true,
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
    async fn test_delete_relation_success() {
        let issues = vec![create_test_issue(1, 1), create_test_issue(2, 1)];
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
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = DeleteRelationUseCase::new(
            issue_repo,
            project_repo,
            member_repo,
            relation_repo.clone(),
        );

        let result = usecase.execute(1, &create_admin_user()).await;
        assert!(result.is_ok());

        // Verify relation was deleted
        let deleted = relation_repo.find_by_id(1).await.unwrap();
        assert!(deleted.is_none());
    }

    #[tokio::test]
    async fn test_delete_relation_not_found() {
        let issues = vec![create_test_issue(1, 1), create_test_issue(2, 1)];
        let projects = vec![create_test_project(1, true)];
        let relations: Vec<IssueRelation> = vec![];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase =
            DeleteRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

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
    async fn test_delete_relation_permission_denied() {
        let issues = vec![create_test_issue(1, 1), create_test_issue(2, 1)];
        let projects = vec![create_test_project(1, true)];
        let relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![]));
        let member_repo = Arc::new(MockMemberRepository::new(false)); // Not a member
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase =
            DeleteRelationUseCase::new(issue_repo, project_repo, member_repo, relation_repo);

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
    async fn test_delete_blocks_relation_deletes_blocked_reverse() {
        let issues = vec![create_test_issue(1, 1), create_test_issue(2, 1)];
        let projects = vec![create_test_project(1, true)];
        let relations = vec![
            IssueRelation {
                id: 1,
                issue_from_id: 1,
                issue_to_id: 2,
                relation_type: "blocks".to_string(),
                delay: None,
            },
            IssueRelation {
                id: 2,
                issue_from_id: 2,
                issue_to_id: 1,
                relation_type: "blocked".to_string(),
                delay: None,
            },
        ];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = DeleteRelationUseCase::new(
            issue_repo,
            project_repo,
            member_repo,
            relation_repo.clone(),
        );

        let result = usecase.execute(1, &create_admin_user()).await;
        assert!(result.is_ok());

        // Verify both relations were deleted
        let blocks = relation_repo.find_by_id(1).await.unwrap();
        assert!(blocks.is_none());
        let blocked = relation_repo.find_by_id(2).await.unwrap();
        assert!(blocked.is_none());
    }

    #[tokio::test]
    async fn test_delete_precedes_relation_deletes_follows_reverse() {
        let issues = vec![create_test_issue(1, 1), create_test_issue(2, 1)];
        let projects = vec![create_test_project(1, true)];
        let relations = vec![
            IssueRelation {
                id: 1,
                issue_from_id: 1,
                issue_to_id: 2,
                relation_type: "precedes".to_string(),
                delay: Some(5),
            },
            IssueRelation {
                id: 2,
                issue_from_id: 2,
                issue_to_id: 1,
                relation_type: "follows".to_string(),
                delay: Some(5),
            },
        ];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1]));
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = DeleteRelationUseCase::new(
            issue_repo,
            project_repo,
            member_repo,
            relation_repo.clone(),
        );

        let result = usecase.execute(1, &create_admin_user()).await;
        assert!(result.is_ok());

        // Verify both relations were deleted
        let precedes = relation_repo.find_by_id(1).await.unwrap();
        assert!(precedes.is_none());
        let follows = relation_repo.find_by_id(2).await.unwrap();
        assert!(follows.is_none());
    }

    #[tokio::test]
    async fn test_delete_relation_with_permission_on_source_project() {
        let issues = vec![
            create_test_issue(1, 1), // Project 1
            create_test_issue(2, 2), // Project 2
        ];
        let projects = vec![create_test_project(1, true), create_test_project(2, true)];
        let relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![1])); // Member of project 1 only
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = DeleteRelationUseCase::new(
            issue_repo,
            project_repo,
            member_repo,
            relation_repo.clone(),
        );

        let result = usecase.execute(1, &create_regular_user()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_relation_with_permission_on_target_project() {
        let issues = vec![
            create_test_issue(1, 1), // Project 1
            create_test_issue(2, 2), // Project 2
        ];
        let projects = vec![create_test_project(1, true), create_test_project(2, true)];
        let relations = vec![IssueRelation {
            id: 1,
            issue_from_id: 1,
            issue_to_id: 2,
            relation_type: "relates".to_string(),
            delay: None,
        }];

        let issue_repo = Arc::new(MockIssueRepository::new(issues));
        let project_repo = Arc::new(MockProjectRepository::new(projects, vec![2])); // Member of project 2 only
        let member_repo = Arc::new(MockMemberRepository::new(true));
        let relation_repo = Arc::new(MockIssueRelationRepository::new(relations));

        let usecase = DeleteRelationUseCase::new(
            issue_repo,
            project_repo,
            member_repo,
            relation_repo.clone(),
        );

        let result = usecase.execute(1, &create_regular_user()).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_reverse_type() {
        assert_eq!(
            DeleteRelationUseCase::<
                MockIssueRepository,
                MockProjectRepository,
                MockMemberRepository,
                MockIssueRelationRepository,
            >::get_reverse_type("relates"),
            Some("relates")
        );
        assert_eq!(
            DeleteRelationUseCase::<
                MockIssueRepository,
                MockProjectRepository,
                MockMemberRepository,
                MockIssueRelationRepository,
            >::get_reverse_type("duplicates"),
            Some("duplicated")
        );
        assert_eq!(
            DeleteRelationUseCase::<
                MockIssueRepository,
                MockProjectRepository,
                MockMemberRepository,
                MockIssueRelationRepository,
            >::get_reverse_type("duplicated"),
            Some("duplicates")
        );
        assert_eq!(
            DeleteRelationUseCase::<
                MockIssueRepository,
                MockProjectRepository,
                MockMemberRepository,
                MockIssueRelationRepository,
            >::get_reverse_type("blocks"),
            Some("blocked")
        );
        assert_eq!(
            DeleteRelationUseCase::<
                MockIssueRepository,
                MockProjectRepository,
                MockMemberRepository,
                MockIssueRelationRepository,
            >::get_reverse_type("blocked"),
            Some("blocks")
        );
        assert_eq!(
            DeleteRelationUseCase::<
                MockIssueRepository,
                MockProjectRepository,
                MockMemberRepository,
                MockIssueRelationRepository,
            >::get_reverse_type("precedes"),
            Some("follows")
        );
        assert_eq!(
            DeleteRelationUseCase::<
                MockIssueRepository,
                MockProjectRepository,
                MockMemberRepository,
                MockIssueRelationRepository,
            >::get_reverse_type("follows"),
            Some("precedes")
        );
        assert_eq!(
            DeleteRelationUseCase::<
                MockIssueRepository,
                MockProjectRepository,
                MockMemberRepository,
                MockIssueRelationRepository,
            >::get_reverse_type("unknown"),
            None
        );
    }
}
