use crate::application::errors::ApplicationError;
use crate::domain::repositories::{
    AttachmentRepository, IssueRelationRepository, IssueRepository, JournalRepository,
    MemberRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Use case for deleting an issue
pub struct DeleteIssueUseCase<I, A, J, R, M>
where
    I: IssueRepository,
    A: AttachmentRepository,
    J: JournalRepository,
    R: IssueRelationRepository,
    M: MemberRepository,
{
    issue_repo: Arc<I>,
    attachment_repo: Arc<A>,
    journal_repo: Arc<J>,
    relation_repo: Arc<R>,
    member_repo: Arc<M>,
}

impl<I, A, J, R, M> DeleteIssueUseCase<I, A, J, R, M>
where
    I: IssueRepository,
    A: AttachmentRepository,
    J: JournalRepository,
    R: IssueRelationRepository,
    M: MemberRepository,
{
    pub fn new(
        issue_repo: Arc<I>,
        attachment_repo: Arc<A>,
        journal_repo: Arc<J>,
        relation_repo: Arc<R>,
        member_repo: Arc<M>,
    ) -> Self {
        Self {
            issue_repo,
            attachment_repo,
            journal_repo,
            relation_repo,
            member_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin: can delete any issue
    /// - Non-admin: needs membership with delete_issues permission
    ///
    /// Validation rules:
    /// - Cannot delete issue with subtasks
    ///
    /// Cascade delete (in order):
    /// 1. Issue relations
    /// 2. Journals
    /// 3. Attachments
    /// 4. Issue itself
    pub async fn execute(
        &self,
        issue_id: i32,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // 1. Get the issue
        let issue = self
            .issue_repo
            .find_by_id(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Issue not found".into()))?;

        // 2. Check permission
        if !current_user.admin {
            // For non-admin, check if user is a member with delete permission
            // For MVP, we check membership - a more fine-grained permission check can be added later
            let is_member = self
                .member_repo
                .is_member(issue.project_id, current_user.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            if !is_member {
                return Err(ApplicationError::Forbidden(
                    "You don't have permission to delete this issue".into(),
                ));
            }
        }

        // 3. Check for subtasks
        let children = self
            .issue_repo
            .find_children(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if !children.is_empty() {
            return Err(ApplicationError::Validation(
                "Cannot delete an issue with subtasks. Delete subtasks first.".into(),
            ));
        }

        // 4. Delete related data in proper order (respecting foreign key relationships)
        // Delete issue relations (both from and to this issue)
        self.relation_repo
            .delete_by_issue(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Delete journals for this issue
        self.journal_repo
            .delete_by_journalized(issue_id, "Issue")
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Delete attachments for this issue
        self.attachment_repo
            .delete_by_container(issue_id, "Issue")
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 5. Delete the issue
        self.issue_repo
            .delete(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Attachment, Issue, IssueRelation, Journal, JournalDetail};
    use crate::domain::repositories::RepositoryError;
    use chrono::Utc;
    use std::collections::HashMap;

    // Mock implementations for testing
    struct MockIssueRepository {
        issues: std::sync::Mutex<HashMap<i32, Issue>>,
    }

    impl MockIssueRepository {
        fn new() -> Self {
            Self {
                issues: std::sync::Mutex::new(HashMap::new()),
            }
        }

        fn with_issue(issue: Issue) -> Self {
            let issue_id = issue.id;
            let mut issues = HashMap::new();
            issues.insert(issue.id, issue);
            Self {
                issues: std::sync::Mutex::new(issues),
            }
        }
    }

    #[async_trait::async_trait]
    impl IssueRepository for MockIssueRepository {
        async fn find_all(
            &self,
            _params: crate::domain::value_objects::IssueQueryParams,
        ) -> Result<Vec<Issue>, RepositoryError> {
            Ok(self.issues.lock().unwrap().values().cloned().collect())
        }

        async fn count(
            &self,
            _params: &crate::domain::value_objects::IssueQueryParams,
        ) -> Result<u32, RepositoryError> {
            Ok(self.issues.lock().unwrap().len() as u32)
        }

        async fn find_by_project(&self, _project_id: i32) -> Result<Vec<Issue>, RepositoryError> {
            Ok(self.issues.lock().unwrap().values().cloned().collect())
        }

        async fn delete_by_project(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<Issue>, RepositoryError> {
            Ok(self.issues.lock().unwrap().get(&id).cloned())
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

        async fn find_children(&self, parent_id: i32) -> Result<Vec<Issue>, RepositoryError> {
            let issues = self.issues.lock().unwrap();
            Ok(issues
                .values()
                .filter(|i| i.parent_id == Some(parent_id))
                .cloned()
                .collect())
        }

        async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
            self.issues.lock().unwrap().remove(&id);
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

    fn create_test_issue(id: i32, project_id: i32, parent_id: Option<i32>) -> Issue {
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
            parent_id,
            root_id: None,
            lft: None,
            rgt: None,
            is_private: false,
            closed_on: None,
        }
    }

    #[tokio::test]
    async fn test_admin_can_delete_issue() {
        let issue = create_test_issue(1, 1, None);
        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });

        let usecase = DeleteIssueUseCase::new(
            issue_repo,
            attachment_repo,
            journal_repo,
            relation_repo,
            member_repo,
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
    async fn test_member_can_delete_issue() {
        let issue = create_test_issue(1, 1, None);
        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });

        let usecase = DeleteIssueUseCase::new(
            issue_repo,
            attachment_repo,
            journal_repo,
            relation_repo,
            member_repo,
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
    async fn test_non_member_cannot_delete_issue() {
        let issue = create_test_issue(1, 1, None);
        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });

        let usecase = DeleteIssueUseCase::new(
            issue_repo,
            attachment_repo,
            journal_repo,
            relation_repo,
            member_repo,
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
    async fn test_cannot_delete_issue_with_subtasks() {
        // Create parent issue
        let parent = create_test_issue(1, 1, None);
        // Create child issue
        let child = create_test_issue(2, 1, Some(1));

        let mut issues = HashMap::new();
        issues.insert(1, parent);
        issues.insert(2, child);

        let issue_repo = Arc::new(MockIssueRepository {
            issues: std::sync::Mutex::new(issues),
        });
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });

        let usecase = DeleteIssueUseCase::new(
            issue_repo,
            attachment_repo,
            journal_repo,
            relation_repo,
            member_repo,
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
    async fn test_delete_issue_not_found() {
        let issue_repo = Arc::new(MockIssueRepository::new());
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![],
        });

        let usecase = DeleteIssueUseCase::new(
            issue_repo,
            attachment_repo,
            journal_repo,
            relation_repo,
            member_repo,
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
    async fn test_can_delete_subtask() {
        // Create parent issue
        let parent = create_test_issue(1, 1, None);
        // Create child issue (subtask)
        let child = create_test_issue(2, 1, Some(1));

        let mut issues = HashMap::new();
        issues.insert(1, parent);
        issues.insert(2, child);

        let issue_repo = Arc::new(MockIssueRepository {
            issues: std::sync::Mutex::new(issues),
        });
        let attachment_repo = Arc::new(MockAttachmentRepository);
        let journal_repo = Arc::new(MockJournalRepository);
        let relation_repo = Arc::new(MockIssueRelationRepository);
        let member_repo = Arc::new(MockMemberRepository {
            member_project_ids: vec![1],
        });

        let usecase = DeleteIssueUseCase::new(
            issue_repo,
            attachment_repo,
            journal_repo,
            relation_repo,
            member_repo,
        );

        let current_user = CurrentUser {
            id: 1,
            login: "admin".to_string(),
            admin: true,
        };

        // Deleting the subtask (id=2) should succeed because it has no children
        let result = usecase.execute(2, &current_user).await;
        assert!(result.is_ok());
    }
}
