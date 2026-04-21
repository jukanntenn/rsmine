use crate::application::errors::ApplicationError;
use crate::domain::repositories::{
    AttachmentRepository, IssueCategoryRepository, IssueRelationRepository, IssueRepository,
    JournalRepository, MemberRepository, ProjectRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Use case for deleting a project
pub struct DeleteProjectUseCase<
    P: ProjectRepository,
    I: IssueRepository,
    M: MemberRepository,
    A: AttachmentRepository,
    C: IssueCategoryRepository,
    J: JournalRepository,
    R: IssueRelationRepository,
> {
    project_repo: Arc<P>,
    issue_repo: Arc<I>,
    member_repo: Arc<M>,
    attachment_repo: Arc<A>,
    category_repo: Arc<C>,
    journal_repo: Arc<J>,
    relation_repo: Arc<R>,
}

impl<
        P: ProjectRepository,
        I: IssueRepository,
        M: MemberRepository,
        A: AttachmentRepository,
        C: IssueCategoryRepository,
        J: JournalRepository,
        R: IssueRelationRepository,
    > DeleteProjectUseCase<P, I, M, A, C, J, R>
{
    pub fn new(
        project_repo: Arc<P>,
        issue_repo: Arc<I>,
        member_repo: Arc<M>,
        attachment_repo: Arc<A>,
        category_repo: Arc<C>,
        journal_repo: Arc<J>,
        relation_repo: Arc<R>,
    ) -> Self {
        Self {
            project_repo,
            issue_repo,
            member_repo,
            attachment_repo,
            category_repo,
            journal_repo,
            relation_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin: can delete any project
    /// - Non-admin: needs membership with delete permission
    ///
    /// Validation rules:
    /// - Cannot delete project with subprojects
    ///
    /// Cascade delete (in order):
    /// 1. Issue relations (for all issues in project)
    /// 2. Journals (for all issues in project)
    /// 3. Attachments (for all issues in project)
    /// 4. Issues
    /// 5. Issue categories
    /// 6. Members
    /// 7. Project-tracker associations
    /// 8. Project itself
    pub async fn execute(
        &self,
        project_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // 1. Get the project
        let project = self
            .project_repo
            .find_by_id(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Project not found".into()))?;

        // 2. Check permission
        if !current_user.admin {
            // For non-admin, check if user is a member with delete permission
            // For MVP, we check membership - a more fine-grained permission check can be added later
            let is_member = self
                .member_repo
                .is_member(project.id, current_user.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            if !is_member {
                return Err(ApplicationError::Forbidden(
                    "You don't have permission to delete this project".into(),
                ));
            }
        }

        // 3. Check for subprojects
        let subprojects = self
            .project_repo
            .find_children(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if !subprojects.is_empty() {
            return Err(ApplicationError::Validation(
                "Cannot delete project with subprojects. Delete or move subprojects first.".into(),
            ));
        }

        // 4. Delete related data in proper order (respecting foreign key relationships)
        self.delete_project_data(project_id).await?;

        // 5. Delete the project
        self.project_repo
            .delete(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(())
    }

    /// Delete all data associated with a project
    async fn delete_project_data(&self, project_id: i32) -> Result<(), ApplicationError> {
        // Get all issues for this project
        let issues = self
            .issue_repo
            .find_by_project(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Delete issue-related data for each issue
        for issue in &issues {
            // Delete issue relations (both from and to this issue)
            self.relation_repo
                .delete_by_issue(issue.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            // Delete journals for this issue
            self.journal_repo
                .delete_by_journalized(issue.id, "Issue")
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            // Delete attachments for this issue
            self.attachment_repo
                .delete_by_container(issue.id, "Issue")
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
        }

        // Delete all issues for this project
        self.issue_repo
            .delete_by_project(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Delete issue categories
        self.category_repo
            .delete_by_project(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Delete all members for this project
        self.member_repo
            .delete_by_project(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Delete project-tracker associations
        self.project_repo
            .clear_trackers(project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{
        Attachment, Issue, IssueCategory, IssueRelation, Journal, JournalDetail, Project,
        PROJECT_STATUS_ACTIVE,
    };
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

        async fn find_children(&self, project_id: i32) -> Result<Vec<Project>, RepositoryError> {
            Ok(self
                .projects
                .iter()
                .filter(|p| p.parent_id == Some(project_id))
                .cloned()
                .collect())
        }

        async fn delete(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn clear_trackers(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockIssueRepository {
        issues: Vec<Issue>,
    }

    #[async_trait::async_trait]
    impl IssueRepository for MockIssueRepository {
        async fn find_all(
            &self,
            _params: crate::domain::value_objects::IssueQueryParams,
        ) -> Result<Vec<Issue>, RepositoryError> {
            Ok(self.issues.clone())
        }

        async fn count(
            &self,
            _params: &crate::domain::value_objects::IssueQueryParams,
        ) -> Result<u32, RepositoryError> {
            Ok(self.issues.len() as u32)
        }

        async fn find_by_project(&self, project_id: i32) -> Result<Vec<Issue>, RepositoryError> {
            Ok(self
                .issues
                .iter()
                .filter(|i| i.project_id == project_id)
                .cloned()
                .collect())
        }

        async fn delete_by_project(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<Issue>, RepositoryError> {
            Ok(self.issues.iter().find(|i| i.id == id).cloned())
        }

        async fn clear_assignee_in_project(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn create(
            &self,
            _issue: crate::domain::repositories::NewIssue,
        ) -> Result<Issue, RepositoryError> {
            unimplemented!()
        }

        async fn update(
            &self,
            _id: i32,
            _update: crate::domain::repositories::IssueUpdate,
        ) -> Result<Issue, RepositoryError> {
            unimplemented!()
        }

        async fn find_children(&self, _parent_id: i32) -> Result<Vec<Issue>, RepositoryError> {
            Ok(Vec::new())
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
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

    struct MockAttachmentRepository;

    #[async_trait::async_trait]
    impl AttachmentRepository for MockAttachmentRepository {
        async fn find_by_container(
            &self,
            _container_id: i32,
            _container_type: &str,
        ) -> Result<Vec<Attachment>, RepositoryError> {
            Ok(vec![])
        }

        async fn find_by_id(&self, _id: i32) -> Result<Option<Attachment>, RepositoryError> {
            Ok(None)
        }

        async fn create(
            &self,
            _attachment: crate::domain::repositories::NewAttachment,
        ) -> Result<Attachment, RepositoryError> {
            unimplemented!()
        }

        async fn delete_by_container(
            &self,
            _container_id: i32,
            _container_type: &str,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn increment_downloads(&self, _id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn has_other_with_digest(
            &self,
            _digest: &str,
            _exclude_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn update_description(
            &self,
            _id: i32,
            _description: Option<String>,
        ) -> Result<Attachment, RepositoryError> {
            unimplemented!()
        }
    }

    struct MockIssueCategoryRepository;

    #[async_trait::async_trait]
    impl IssueCategoryRepository for MockIssueCategoryRepository {
        async fn find_by_project(
            &self,
            _project_id: i32,
        ) -> Result<Vec<IssueCategory>, RepositoryError> {
            Ok(vec![])
        }

        async fn find_by_id(&self, _id: i32) -> Result<Option<IssueCategory>, RepositoryError> {
            Ok(None)
        }

        async fn create(
            &self,
            _category: &crate::domain::repositories::NewIssueCategory,
        ) -> Result<IssueCategory, RepositoryError> {
            unimplemented!()
        }

        async fn update(
            &self,
            _id: i32,
            _category: &crate::domain::repositories::IssueCategoryUpdate,
        ) -> Result<IssueCategory, RepositoryError> {
            unimplemented!()
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
            _project_id: i32,
            _name: &str,
            _exclude_id: Option<i32>,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
        }
    }

    struct MockJournalRepository;

    #[async_trait::async_trait]
    impl JournalRepository for MockJournalRepository {
        async fn find_by_journalized(
            &self,
            _journalized_id: i32,
            _journalized_type: &str,
        ) -> Result<Vec<Journal>, RepositoryError> {
            Ok(vec![])
        }

        async fn delete_by_journalized(
            &self,
            _journalized_id: i32,
            _journalized_type: &str,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn find_details(
            &self,
            _journal_id: i32,
        ) -> Result<Vec<JournalDetail>, RepositoryError> {
            Ok(vec![])
        }

        async fn create(
            &self,
            _journal: crate::domain::repositories::NewJournal,
        ) -> Result<Journal, RepositoryError> {
            unimplemented!()
        }

        async fn create_detail(
            &self,
            _detail: crate::domain::repositories::NewJournalDetail,
        ) -> Result<JournalDetail, RepositoryError> {
            unimplemented!()
        }
    }

    struct MockIssueRelationRepository;

    #[async_trait::async_trait]
    impl IssueRelationRepository for MockIssueRelationRepository {
        async fn find_by_issue(
            &self,
            _issue_id: i32,
        ) -> Result<Vec<IssueRelation>, RepositoryError> {
            Ok(vec![])
        }

        async fn delete_by_issue(&self, _issue_id: i32) -> Result<(), RepositoryError> {
            Ok(())
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
            Ok(false)
        }

        async fn find_relation(
            &self,
            _issue_from_id: i32,
            _issue_to_id: i32,
            _relation_type: &str,
        ) -> Result<Option<IssueRelation>, RepositoryError> {
            Ok(None)
        }

        async fn find_by_id(&self, _id: i32) -> Result<Option<IssueRelation>, RepositoryError> {
            Ok(None)
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn delete_by_issues_and_type(
            &self,
            _issue_from_id: i32,
            _issue_to_id: i32,
            _relation_type: &str,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    fn create_test_project(id: i32, identifier: &str, parent_id: Option<i32>) -> Project {
        Project {
            id,
            name: format!("Project {}", id),
            description: Some(format!("Description for project {}", id)),
            homepage: None,
            is_public: true,
            parent_id,
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

    fn create_test_issue(id: i32, project_id: i32) -> Issue {
        Issue {
            id,
            tracker_id: 1,
            project_id,
            subject: format!("Issue {}", id),
            description: None,
            due_date: None,
            category_id: None,
            status_id: 1,
            assigned_to_id: None,
            priority_id: 2,
            fixed_version_id: None,
            author_id: 1,
            lock_version: 0,
            created_on: Some(Utc::now()),
            updated_on: Some(Utc::now()),
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

    #[tokio::test]
    async fn test_admin_can_delete_project() {
        let projects = vec![create_test_project(1, "proj1", None)];
        let issues = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let issue_repo = Arc::new(MockIssueRepository { issues });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let category_repo = Arc::new(MockIssueCategoryRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);

        let usecase = DeleteProjectUseCase::new(
            project_repo,
            issue_repo,
            member_repo,
            attachment_repo,
            category_repo,
            journal_repo,
            relation_repo,
        );
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_member_can_delete_project() {
        let projects = vec![create_test_project(1, "proj1", None)];
        let issues = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let issue_repo = Arc::new(MockIssueRepository { issues });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let category_repo = Arc::new(MockIssueCategoryRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);

        let usecase = DeleteProjectUseCase::new(
            project_repo,
            issue_repo,
            member_repo,
            attachment_repo,
            category_repo,
            journal_repo,
            relation_repo,
        );
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
            admin: false,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_non_member_cannot_delete_project() {
        let projects = vec![create_test_project(1, "proj1", None)];
        let issues = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let issue_repo = Arc::new(MockIssueRepository { issues });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let category_repo = Arc::new(MockIssueCategoryRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);

        let usecase = DeleteProjectUseCase::new(
            project_repo,
            issue_repo,
            member_repo,
            attachment_repo,
            category_repo,
            journal_repo,
            relation_repo,
        );
        let current_user = CurrentUser {
            id: 1,
            login: "user".to_string(),
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
    async fn test_cannot_delete_project_with_subprojects() {
        let projects = vec![
            create_test_project(1, "proj1", None),
            create_test_project(2, "proj2", Some(1)), // proj2 is a subproject of proj1
        ];
        let issues = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let issue_repo = Arc::new(MockIssueRepository { issues });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let category_repo = Arc::new(MockIssueCategoryRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);

        let usecase = DeleteProjectUseCase::new(
            project_repo,
            issue_repo,
            member_repo,
            attachment_repo,
            category_repo,
            journal_repo,
            relation_repo,
        );
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ApplicationError::Validation(_)
        ));
    }

    #[tokio::test]
    async fn test_delete_project_not_found() {
        let projects = vec![];
        let issues = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let issue_repo = Arc::new(MockIssueRepository { issues });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let category_repo = Arc::new(MockIssueCategoryRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);

        let usecase = DeleteProjectUseCase::new(
            project_repo,
            issue_repo,
            member_repo,
            attachment_repo,
            category_repo,
            journal_repo,
            relation_repo,
        );
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
    async fn test_delete_project_with_issues() {
        let projects = vec![create_test_project(1, "proj1", None)];
        let issues = vec![create_test_issue(1, 1), create_test_issue(2, 1)];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let issue_repo = Arc::new(MockIssueRepository { issues });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let category_repo = Arc::new(MockIssueCategoryRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);

        let usecase = DeleteProjectUseCase::new(
            project_repo,
            issue_repo,
            member_repo,
            attachment_repo,
            category_repo,
            journal_repo,
            relation_repo,
        );
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        let result = usecase.execute(1, &current_user).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_can_delete_subproject() {
        let projects = vec![
            create_test_project(1, "proj1", None),
            create_test_project(2, "proj2", Some(1)), // proj2 is a subproject of proj1
        ];
        let issues = vec![];

        let project_repo = Arc::new(MockProjectRepository { projects });
        let issue_repo = Arc::new(MockIssueRepository { issues });
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![2],
        });
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let category_repo = Arc::new(MockIssueCategoryRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);

        let usecase = DeleteProjectUseCase::new(
            project_repo,
            issue_repo,
            member_repo,
            attachment_repo,
            category_repo,
            journal_repo,
            relation_repo,
        );
        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        // Deleting proj2 should succeed because it has no subprojects
        let result = usecase.execute(2, &current_user).await;
        assert!(result.is_ok());
    }
}
