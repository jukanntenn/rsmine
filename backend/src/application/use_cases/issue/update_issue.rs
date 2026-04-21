use super::{GetIssueResponse, StatusInfo};
use crate::application::dto::UpdateIssueDto;
use crate::application::errors::ApplicationError;
use crate::domain::entities::Issue;
use crate::domain::repositories::{
    EnumerationRepository, IssueRepository, IssueStatusRepository, IssueUpdate, JournalRepository,
    MemberRepository, NewJournal, NewJournalDetail, ProjectRepository, TrackerRepository,
    UserRepository,
};
use crate::presentation::middleware::CurrentUser;
use chrono::Utc;
use std::sync::Arc;

/// Response for update issue endpoint
#[derive(Debug, Clone)]
pub struct UpdateIssueResponse {
    pub issue: GetIssueResponse,
}

/// Use case for updating an existing issue
pub struct UpdateIssueUseCase<I, P, U, T, S, E, J, M>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
    J: JournalRepository,
    M: MemberRepository,
{
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    user_repo: Arc<U>,
    tracker_repo: Arc<T>,
    status_repo: Arc<S>,
    enumeration_repo: Arc<E>,
    journal_repo: Arc<J>,
    member_repo: Arc<M>,
}

impl<I, P, U, T, S, E, J, M> UpdateIssueUseCase<I, P, U, T, S, E, J, M>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
    J: JournalRepository,
    M: MemberRepository,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        user_repo: Arc<U>,
        tracker_repo: Arc<T>,
        status_repo: Arc<S>,
        enumeration_repo: Arc<E>,
        journal_repo: Arc<J>,
        member_repo: Arc<M>,
    ) -> Self {
        Self {
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enumeration_repo,
            journal_repo,
            member_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin: can update any issue
    /// - Non-admin: needs `edit_issues` permission OR `edit_own_issues` for own issues
    ///
    /// Validation rules:
    /// - Subject: if provided, must be 1-255 chars and not blank
    /// - Status: if provided, must exist
    /// - Tracker: if provided, must exist and be available in the project
    /// - Priority: if provided, must exist
    /// - Assigned_to: if provided, must exist
    /// - Done ratio: if provided, must be 0-100
    /// - Parent issue: if provided, must be in the same project
    pub async fn execute(
        &self,
        issue_id: i32,
        dto: UpdateIssueDto,
        current_user: &CurrentUser,
    ) -> Result<UpdateIssueResponse, ApplicationError> {
        // 1. Validate input
        if let Err(errors) = dto.validate() {
            return Err(ApplicationError::Validation(errors.join(", ")));
        }

        // 2. Check if there are any changes to apply
        if !dto.has_changes() {
            return Err(ApplicationError::Validation("No changes provided".into()));
        }

        // 3. Get existing issue
        let issue = self
            .issue_repo
            .find_by_id(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Issue {} not found", issue_id)))?;

        // 4. Check edit permission
        self.check_edit_permission(&issue, current_user).await?;

        // 5. Track changes for journal
        let mut journal_details = Vec::new();
        let now = Utc::now();

        // 6. Validate and track changes
        let mut update = IssueUpdate {
            subject: None,
            description: None,
            status_id: None,
            priority_id: None,
            tracker_id: None,
            assigned_to_id: None,
            category_id: None,
            parent_id: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            done_ratio: None,
            is_private: None,
            updated_on: Some(now),
            closed_on: None,
        };

        // Track subject changes
        if let Some(subject) = &dto.subject {
            let subject = subject.trim();
            if subject != issue.subject {
                journal_details.push(NewJournalDetail {
                    journal_id: 0, // Will be set after journal creation
                    property: "attr".to_string(),
                    prop_key: "subject".to_string(),
                    old_value: Some(issue.subject.clone()),
                    value: Some(subject.to_string()),
                });
                update.subject = Some(subject.to_string());
            }
        }

        // Track description changes
        if let Some(description) = &dto.description {
            let new_desc = if description.trim().is_empty() {
                None
            } else {
                Some(description.clone())
            };
            if new_desc != issue.description {
                journal_details.push(NewJournalDetail {
                    journal_id: 0,
                    property: "attr".to_string(),
                    prop_key: "description".to_string(),
                    old_value: issue.description.clone(),
                    value: new_desc,
                });
                update.description = Some(description.clone());
            }
        }

        // Track status changes
        if let Some(status_id) = dto.status_id {
            if status_id != issue.status_id {
                // Validate status exists
                let status = self
                    .status_repo
                    .find_by_id(status_id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;

                if status.is_none() {
                    return Err(ApplicationError::Validation("Status is invalid".into()));
                }

                journal_details.push(NewJournalDetail {
                    journal_id: 0,
                    property: "attr".to_string(),
                    prop_key: "status_id".to_string(),
                    old_value: Some(issue.status_id.to_string()),
                    value: Some(status_id.to_string()),
                });
                update.status_id = Some(status_id);

                // Handle closed status - check is_closed before moving status
                let is_closed = status.as_ref().map(|s| s.is_closed).unwrap_or(false);
                if is_closed && issue.closed_on.is_none() {
                    update.closed_on = Some(Some(now));
                } else if !is_closed {
                    update.closed_on = Some(None);
                }
            }
        }

        // Track priority changes
        if let Some(priority_id) = dto.priority_id {
            if priority_id != issue.priority_id {
                // Validate priority exists
                let priority = self
                    .enumeration_repo
                    .find_by_id(priority_id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;

                if priority.is_none() {
                    return Err(ApplicationError::Validation("Priority is invalid".into()));
                }

                journal_details.push(NewJournalDetail {
                    journal_id: 0,
                    property: "attr".to_string(),
                    prop_key: "priority_id".to_string(),
                    old_value: Some(issue.priority_id.to_string()),
                    value: Some(priority_id.to_string()),
                });
                update.priority_id = Some(priority_id);
            }
        }

        // Track tracker changes
        if let Some(tracker_id) = dto.tracker_id {
            if tracker_id != issue.tracker_id {
                // Validate tracker exists and is available in project
                let tracker = self
                    .tracker_repo
                    .find_by_id(tracker_id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;

                if tracker.is_none() {
                    return Err(ApplicationError::Validation("Tracker is invalid".into()));
                }

                // Check if tracker is available in project
                let project_trackers = self
                    .tracker_repo
                    .find_by_project(issue.project_id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;

                if !project_trackers.iter().any(|t| t.id == tracker_id) {
                    return Err(ApplicationError::Validation(
                        "Tracker is not available in this project".into(),
                    ));
                }

                journal_details.push(NewJournalDetail {
                    journal_id: 0,
                    property: "attr".to_string(),
                    prop_key: "tracker_id".to_string(),
                    old_value: Some(issue.tracker_id.to_string()),
                    value: Some(tracker_id.to_string()),
                });
                update.tracker_id = Some(tracker_id);
            }
        }

        // Track assigned_to changes
        if let Some(assigned_to_id) = dto.assigned_to_id {
            let new_assignee = if assigned_to_id > 0 {
                Some(assigned_to_id)
            } else {
                None
            };
            if new_assignee != issue.assigned_to_id {
                // Validate assignee exists if provided
                if let Some(uid) = new_assignee {
                    let user = self
                        .user_repo
                        .find_by_id(uid)
                        .await
                        .map_err(|e| ApplicationError::Internal(e.to_string()))?;

                    if user.is_none() {
                        return Err(ApplicationError::Validation("Assignee is invalid".into()));
                    }
                }

                journal_details.push(NewJournalDetail {
                    journal_id: 0,
                    property: "attr".to_string(),
                    prop_key: "assigned_to_id".to_string(),
                    old_value: issue.assigned_to_id.map(|id| id.to_string()),
                    value: new_assignee.map(|id| id.to_string()),
                });
                update.assigned_to_id = Some(new_assignee);
            }
        }

        // Track done_ratio changes
        if let Some(done_ratio) = dto.done_ratio {
            if done_ratio != issue.done_ratio {
                journal_details.push(NewJournalDetail {
                    journal_id: 0,
                    property: "attr".to_string(),
                    prop_key: "done_ratio".to_string(),
                    old_value: Some(issue.done_ratio.to_string()),
                    value: Some(done_ratio.to_string()),
                });
                update.done_ratio = Some(done_ratio);
            }
        }

        // Track is_private changes
        if let Some(is_private) = dto.is_private {
            if is_private != issue.is_private {
                journal_details.push(NewJournalDetail {
                    journal_id: 0,
                    property: "attr".to_string(),
                    prop_key: "is_private".to_string(),
                    old_value: Some(issue.is_private.to_string()),
                    value: Some(is_private.to_string()),
                });
                update.is_private = Some(is_private);
            }
        }

        // Track due_date changes
        if dto.due_date.is_some() {
            let new_due_date = dto.due_date;
            if new_due_date != issue.due_date {
                journal_details.push(NewJournalDetail {
                    journal_id: 0,
                    property: "attr".to_string(),
                    prop_key: "due_date".to_string(),
                    old_value: issue.due_date.map(|d| d.to_string()),
                    value: new_due_date.map(|d| d.to_string()),
                });
                update.due_date = Some(new_due_date);
            }
        }

        // Track start_date changes
        if dto.start_date.is_some() {
            let new_start_date = dto.start_date;
            if new_start_date != issue.start_date {
                journal_details.push(NewJournalDetail {
                    journal_id: 0,
                    property: "attr".to_string(),
                    prop_key: "start_date".to_string(),
                    old_value: issue.start_date.map(|d| d.to_string()),
                    value: new_start_date.map(|d| d.to_string()),
                });
                update.start_date = Some(new_start_date);
            }
        }

        // Track estimated_hours changes
        if dto.estimated_hours.is_some() {
            let new_estimated = dto.estimated_hours;
            if new_estimated != issue.estimated_hours {
                journal_details.push(NewJournalDetail {
                    journal_id: 0,
                    property: "attr".to_string(),
                    prop_key: "estimated_hours".to_string(),
                    old_value: issue.estimated_hours.map(|h| h.to_string()),
                    value: new_estimated.map(|h| h.to_string()),
                });
                update.estimated_hours = Some(new_estimated);
            }
        }

        // 7. Update issue
        let updated_issue = self
            .issue_repo
            .update(issue_id, update)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 8. Create journal entry if there are changes or notes
        let has_notes = dto
            .notes
            .as_ref()
            .map(|n| !n.trim().is_empty())
            .unwrap_or(false);

        if has_notes || !journal_details.is_empty() {
            let journal = NewJournal {
                journalized_id: updated_issue.id,
                journalized_type: "Issue".to_string(),
                user_id: current_user.id,
                notes: dto.notes.filter(|n| !n.trim().is_empty()),
                private_notes: dto.private_notes,
            };

            let created_journal = self
                .journal_repo
                .create(journal)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            // Add journal details
            for mut detail in journal_details {
                detail.journal_id = created_journal.id;
                self.journal_repo
                    .create_detail(detail)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?;
            }
        }

        // 9. Build response
        let response = self.build_response(&updated_issue).await?;

        Ok(UpdateIssueResponse { issue: response })
    }

    /// Check if the current user can edit the issue
    async fn check_edit_permission(
        &self,
        issue: &Issue,
        current_user: &CurrentUser,
    ) -> Result<(), ApplicationError> {
        // Admin can edit any issue
        if current_user.admin {
            return Ok(());
        }

        // Check if user is a member of the project with edit permission
        // For MVP, we check if user is a member of the project
        // A full implementation would check role permissions for "edit_issues" or "edit_own_issues"
        let is_member = self
            .member_repo
            .is_member(issue.project_id, current_user.id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if !is_member {
            return Err(ApplicationError::Forbidden(
                "You don't have permission to edit this issue".into(),
            ));
        }

        // For MVP, any member can edit issues
        // In a full implementation, we would check:
        // - edit_issues permission: can edit any issue
        // - edit_own_issues permission: can only edit own issues (issue.author_id == current_user.id)

        Ok(())
    }

    /// Build the response from the updated issue
    async fn build_response(&self, issue: &Issue) -> Result<GetIssueResponse, ApplicationError> {
        use super::super::project::NamedId;

        // Get project
        let project = self
            .project_repo
            .find_by_id(issue.project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let project_info = project
            .map(|p| NamedId {
                id: p.id,
                name: p.name,
            })
            .unwrap_or(NamedId {
                id: issue.project_id,
                name: format!("Project {}", issue.project_id),
            });

        // Get tracker
        let tracker = self
            .tracker_repo
            .find_by_id(issue.tracker_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let tracker_info = tracker
            .map(|t| NamedId {
                id: t.id,
                name: t.name,
            })
            .unwrap_or(NamedId {
                id: issue.tracker_id,
                name: format!("Tracker {}", issue.tracker_id),
            });

        // Get status
        let status = self
            .status_repo
            .find_by_id(issue.status_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let status_info = status
            .map(|s| StatusInfo {
                id: s.id,
                name: s.name,
                is_closed: s.is_closed,
            })
            .unwrap_or(StatusInfo {
                id: issue.status_id,
                name: format!("Status {}", issue.status_id),
                is_closed: false,
            });

        // Get priority
        let priority = self
            .enumeration_repo
            .find_by_id(issue.priority_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let priority_info = priority
            .map(|p| NamedId {
                id: p.id,
                name: p.name,
            })
            .unwrap_or(NamedId {
                id: issue.priority_id,
                name: format!("Priority {}", issue.priority_id),
            });

        // Get author
        let author = self
            .user_repo
            .find_by_id(issue.author_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let author_info = author
            .map(|u| NamedId {
                id: u.id,
                name: format!("{} {}", u.firstname, u.lastname),
            })
            .unwrap_or(NamedId {
                id: issue.author_id,
                name: format!("User {}", issue.author_id),
            });

        // Get assigned_to if present
        let assigned_to_info = if let Some(assigned_to_id) = issue.assigned_to_id {
            let assigned_user = self
                .user_repo
                .find_by_id(assigned_to_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            assigned_user.map(|u| NamedId {
                id: u.id,
                name: format!("{} {}", u.firstname, u.lastname),
            })
        } else {
            None
        };

        Ok(GetIssueResponse {
            id: issue.id,
            project: project_info,
            tracker: tracker_info,
            status: status_info,
            priority: priority_info,
            author: author_info,
            assigned_to: assigned_to_info,
            subject: issue.subject.clone(),
            description: issue.description.clone(),
            start_date: issue.start_date.map(|d| d.to_string()),
            due_date: issue.due_date.map(|d| d.to_string()),
            done_ratio: issue.done_ratio,
            is_private: issue.is_private,
            estimated_hours: issue.estimated_hours,
            total_estimated_hours: issue.estimated_hours,
            spent_hours: 0.0,
            total_spent_hours: 0.0,
            created_on: issue.created_on.map(|d| d.to_rfc3339()),
            updated_on: issue.updated_on.map(|d| d.to_rfc3339()),
            closed_on: issue.closed_on.map(|d| d.to_rfc3339()),
            children: None,
            attachments: None,
            journals: None,
            relations: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{
        Enumeration, Issue, IssueStatus, Journal, JournalDetail, Project, Tracker, User,
    };
    use crate::domain::repositories::{
        IssueQueryParams, NewIssue, ProjectQueryParams, RepositoryError, UserQueryParams,
    };
    use std::collections::HashMap;

    // Mock implementations for testing
    struct MockIssueRepository {
        issues: std::sync::Mutex<HashMap<i32, Issue>>,
        next_id: std::sync::atomic::AtomicI32,
    }

    impl MockIssueRepository {
        fn new() -> Self {
            Self {
                issues: std::sync::Mutex::new(HashMap::new()),
                next_id: std::sync::atomic::AtomicI32::new(1),
            }
        }

        fn with_issue(issue: Issue) -> Self {
            let issue_id = issue.id;
            let mut issues = HashMap::new();
            issues.insert(issue.id, issue);
            Self {
                issues: std::sync::Mutex::new(issues),
                next_id: std::sync::atomic::AtomicI32::new(issue_id + 1),
            }
        }
    }

    #[async_trait::async_trait]
    impl IssueRepository for MockIssueRepository {
        async fn find_all(&self, _params: IssueQueryParams) -> Result<Vec<Issue>, RepositoryError> {
            Ok(self.issues.lock().unwrap().values().cloned().collect())
        }

        async fn count(&self, _params: &IssueQueryParams) -> Result<u32, RepositoryError> {
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

        async fn create(&self, _issue: NewIssue) -> Result<Issue, RepositoryError> {
            unimplemented!()
        }

        async fn update(
            &self,
            id: i32,
            update: crate::domain::repositories::IssueUpdate,
        ) -> Result<Issue, RepositoryError> {
            let mut issues = self.issues.lock().unwrap();
            let issue = issues
                .get_mut(&id)
                .ok_or_else(|| RepositoryError::NotFound(format!("Issue {} not found", id)))?;

            if let Some(subject) = update.subject {
                issue.subject = subject;
            }
            if let Some(description) = update.description {
                issue.description = Some(description);
            }
            if let Some(status_id) = update.status_id {
                issue.status_id = status_id;
            }
            if let Some(priority_id) = update.priority_id {
                issue.priority_id = priority_id;
            }
            if let Some(tracker_id) = update.tracker_id {
                issue.tracker_id = tracker_id;
            }
            if let Some(assigned_to_id) = update.assigned_to_id {
                issue.assigned_to_id = assigned_to_id;
            }
            if let Some(done_ratio) = update.done_ratio {
                issue.done_ratio = done_ratio;
            }
            if let Some(is_private) = update.is_private {
                issue.is_private = is_private;
            }
            if let Some(due_date) = update.due_date {
                issue.due_date = due_date;
            }
            if let Some(start_date) = update.start_date {
                issue.start_date = start_date;
            }
            if let Some(estimated_hours) = update.estimated_hours {
                issue.estimated_hours = estimated_hours;
            }
            if let Some(closed_on) = update.closed_on {
                issue.closed_on = closed_on;
            }
            if let Some(updated_on) = update.updated_on {
                issue.updated_on = Some(updated_on);
            }

            issue.lock_version += 1;
            Ok(issue.clone())
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

    struct MockProjectRepository {
        projects: HashMap<i32, Project>,
    }

    impl MockProjectRepository {
        fn new() -> Self {
            Self {
                projects: HashMap::new(),
            }
        }

        fn with_project(project: Project) -> Self {
            let mut projects = HashMap::new();
            projects.insert(project.id, project);
            Self { projects }
        }
    }

    #[async_trait::async_trait]
    impl ProjectRepository for MockProjectRepository {
        async fn find_all(
            &self,
            _params: ProjectQueryParams,
        ) -> Result<Vec<Project>, RepositoryError> {
            Ok(self.projects.values().cloned().collect())
        }

        async fn count(&self, _params: &ProjectQueryParams) -> Result<u32, RepositoryError> {
            Ok(self.projects.len() as u32)
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<Project>, RepositoryError> {
            Ok(self.projects.get(&id).cloned())
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
            _params: ProjectQueryParams,
        ) -> Result<Vec<Project>, RepositoryError> {
            Ok(self.projects.values().cloned().collect())
        }

        async fn count_visible_for_user(
            &self,
            _user_id: i32,
            _params: &ProjectQueryParams,
        ) -> Result<u32, RepositoryError> {
            Ok(self.projects.len() as u32)
        }

        async fn find_project_ids_by_user_membership(
            &self,
            _user_id: i32,
        ) -> Result<Vec<i32>, RepositoryError> {
            Ok(self.projects.keys().cloned().collect())
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

        async fn create(&self, _project: Project) -> Result<Project, RepositoryError> {
            unimplemented!()
        }

        async fn update(&self, _project: Project) -> Result<Project, RepositoryError> {
            unimplemented!()
        }

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
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
            Ok(Vec::new())
        }

        async fn clear_trackers(&self, _project_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    struct MockUserRepository {
        users: HashMap<i32, User>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: HashMap::new(),
            }
        }

        fn with_user(user: User) -> Self {
            let mut users = HashMap::new();
            users.insert(user.id, user);
            Self { users }
        }
    }

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, id: i32) -> Result<Option<User>, RepositoryError> {
            Ok(self.users.get(&id).cloned())
        }

        async fn find_by_login(&self, _login: &str) -> Result<Option<User>, RepositoryError> {
            Ok(None)
        }

        async fn find_all(&self, _params: UserQueryParams) -> Result<Vec<User>, RepositoryError> {
            Ok(self.users.values().cloned().collect())
        }

        async fn count(&self, _params: &UserQueryParams) -> Result<u32, RepositoryError> {
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

        async fn delete(&self, _id: i32) -> Result<(), RepositoryError> {
            unimplemented!()
        }

        async fn update_last_login(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn exists_by_login_excluding(
            &self,
            _login: &str,
            _exclude_user_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn find_all_admins(&self) -> Result<Vec<User>, RepositoryError> {
            Ok(self.users.values().filter(|u| u.admin).cloned().collect())
        }
    }

    struct MockTrackerRepository {
        trackers: HashMap<i32, Tracker>,
    }

    impl MockTrackerRepository {
        fn new() -> Self {
            Self {
                trackers: HashMap::new(),
            }
        }

        fn with_tracker(tracker: Tracker) -> Self {
            let mut trackers = HashMap::new();
            trackers.insert(tracker.id, tracker);
            Self { trackers }
        }
    }

    #[async_trait::async_trait]
    impl TrackerRepository for MockTrackerRepository {
        async fn find_all(&self) -> Result<Vec<Tracker>, RepositoryError> {
            Ok(self.trackers.values().cloned().collect())
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<Tracker>, RepositoryError> {
            Ok(self.trackers.get(&id).cloned())
        }

        async fn find_by_project(&self, _project_id: i32) -> Result<Vec<Tracker>, RepositoryError> {
            Ok(self.trackers.values().cloned().collect())
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

    struct MockIssueStatusRepository {
        statuses: HashMap<i32, IssueStatus>,
    }

    impl MockIssueStatusRepository {
        fn new() -> Self {
            Self {
                statuses: HashMap::new(),
            }
        }

        fn with_status(status: IssueStatus) -> Self {
            let mut statuses = HashMap::new();
            statuses.insert(status.id, status);
            Self { statuses }
        }
    }

    #[async_trait::async_trait]
    impl IssueStatusRepository for MockIssueStatusRepository {
        async fn find_all(&self) -> Result<Vec<IssueStatus>, RepositoryError> {
            Ok(self.statuses.values().cloned().collect())
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<IssueStatus>, RepositoryError> {
            Ok(self.statuses.get(&id).cloned())
        }

        async fn find_open(&self) -> Result<Vec<IssueStatus>, RepositoryError> {
            Ok(self
                .statuses
                .values()
                .filter(|s| !s.is_closed)
                .cloned()
                .collect())
        }

        async fn find_closed(&self) -> Result<Vec<IssueStatus>, RepositoryError> {
            Ok(self
                .statuses
                .values()
                .filter(|s| s.is_closed)
                .cloned()
                .collect())
        }

        async fn find_default(&self) -> Result<Option<IssueStatus>, RepositoryError> {
            Ok(self.statuses.values().find(|s| s.is_default).cloned())
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
            Ok(false)
        }

        async fn exists_by_name_excluding(
            &self,
            _name: &str,
            _exclude_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(false)
        }

        async fn clear_default(&self) -> Result<(), RepositoryError> {
            Ok(())
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

    struct MockEnumerationRepository {
        enumerations: HashMap<i32, Enumeration>,
    }

    impl MockEnumerationRepository {
        fn new() -> Self {
            Self {
                enumerations: HashMap::new(),
            }
        }

        fn with_enumeration(enum_item: Enumeration) -> Self {
            let mut enumerations = HashMap::new();
            enumerations.insert(enum_item.id, enum_item);
            Self { enumerations }
        }
    }

    #[async_trait::async_trait]
    impl EnumerationRepository for MockEnumerationRepository {
        async fn find_by_id(&self, id: i32) -> Result<Option<Enumeration>, RepositoryError> {
            Ok(self.enumerations.get(&id).cloned())
        }

        async fn find_by_type(
            &self,
            _enum_type: &str,
        ) -> Result<Vec<Enumeration>, RepositoryError> {
            Ok(self.enumerations.values().cloned().collect())
        }

        async fn find_default_by_type(
            &self,
            _enum_type: &str,
        ) -> Result<Option<Enumeration>, RepositoryError> {
            Ok(self.enumerations.values().next().cloned())
        }

        async fn find_active_by_type(
            &self,
            _enum_type: &str,
        ) -> Result<Vec<Enumeration>, RepositoryError> {
            Ok(self
                .enumerations
                .values()
                .filter(|e| e.active)
                .cloned()
                .collect())
        }
    }

    struct MockJournalRepository {
        journals: std::sync::Mutex<Vec<Journal>>,
        details: std::sync::Mutex<Vec<JournalDetail>>,
        next_id: std::sync::atomic::AtomicI32,
    }

    impl MockJournalRepository {
        fn new() -> Self {
            Self {
                journals: std::sync::Mutex::new(Vec::new()),
                details: std::sync::Mutex::new(Vec::new()),
                next_id: std::sync::atomic::AtomicI32::new(1),
            }
        }
    }

    #[async_trait::async_trait]
    impl JournalRepository for MockJournalRepository {
        async fn find_by_journalized(
            &self,
            _journalized_id: i32,
            _journalized_type: &str,
        ) -> Result<Vec<Journal>, RepositoryError> {
            Ok(self.journals.lock().unwrap().clone())
        }

        async fn find_details(
            &self,
            journal_id: i32,
        ) -> Result<Vec<JournalDetail>, RepositoryError> {
            Ok(self
                .details
                .lock()
                .unwrap()
                .iter()
                .filter(|d| d.journal_id == journal_id)
                .cloned()
                .collect())
        }

        async fn delete_by_journalized(
            &self,
            _journalized_id: i32,
            _journalized_type: &str,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn create(&self, journal: NewJournal) -> Result<Journal, RepositoryError> {
            let id = self
                .next_id
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let created = Journal {
                id,
                journalized_id: journal.journalized_id,
                journalized_type: journal.journalized_type,
                user_id: journal.user_id,
                notes: journal.notes,
                created_on: chrono::Utc::now(),
                private_notes: journal.private_notes,
                updated_on: None,
                updated_by_id: None,
            };
            self.journals.lock().unwrap().push(created.clone());
            Ok(created)
        }

        async fn create_detail(
            &self,
            detail: NewJournalDetail,
        ) -> Result<JournalDetail, RepositoryError> {
            let created = JournalDetail {
                id: self
                    .next_id
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                journal_id: detail.journal_id,
                property: detail.property,
                prop_key: detail.prop_key,
                old_value: detail.old_value,
                value: detail.value,
            };
            self.details.lock().unwrap().push(created.clone());
            Ok(created)
        }
    }

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

    #[async_trait::async_trait]
    impl MemberRepository for MockMemberRepository {
        async fn find_by_project(
            &self,
            _project_id: i32,
        ) -> Result<Vec<crate::domain::entities::MemberWithRoles>, RepositoryError> {
            Ok(Vec::new())
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

        async fn delete_by_user_id(&self, _user_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn is_member(
            &self,
            _project_id: i32,
            _user_id: i32,
        ) -> Result<bool, RepositoryError> {
            Ok(self.is_member_result)
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

        async fn clear_roles(&self, _member_id: i32) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn delete_by_id(&self, _member_id: i32) -> Result<(), RepositoryError> {
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
    }

    // Helper functions to create test entities
    fn create_test_issue(id: i32, project_id: i32, author_id: i32) -> Issue {
        Issue {
            id,
            tracker_id: 1,
            project_id,
            subject: "Test Issue".to_string(),
            description: Some("Test description".to_string()),
            due_date: None,
            category_id: None,
            status_id: 1,
            assigned_to_id: None,
            priority_id: 2,
            fixed_version_id: None,
            author_id,
            lock_version: 0,
            created_on: Some(chrono::Utc::now()),
            updated_on: Some(chrono::Utc::now()),
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
            status: 1,
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
        }
    }

    fn create_test_tracker(id: i32) -> Tracker {
        Tracker {
            id,
            name: format!("Tracker {}", id),
            position: Some(id),
            is_in_roadmap: true,
            fields_bits: None,
            default_status_id: 1,
        }
    }

    fn create_test_status(id: i32, is_closed: bool) -> IssueStatus {
        IssueStatus {
            id,
            name: if is_closed {
                "Closed".to_string()
            } else {
                "Open".to_string()
            },
            position: Some(id),
            is_closed,
            is_default: id == 1,
            default_done_ratio: None,
        }
    }

    fn create_test_priority(id: i32) -> Enumeration {
        Enumeration {
            id,
            name: format!("Priority {}", id),
            position: Some(id),
            is_default: id == 2,
            enum_type: "IssuePriority".to_string(),
            active: true,
            project_id: None,
            parent_id: None,
            position_name: None,
        }
    }

    fn create_current_user(id: i32, admin: bool) -> CurrentUser {
        CurrentUser {
            id,
            login: format!("user{}", id),
            admin,
        }
    }

    #[tokio::test]
    async fn test_update_issue_as_admin() {
        // Setup
        let issue = create_test_issue(1, 1, 2);
        let project = create_test_project(1);
        let tracker = create_test_tracker(1);
        let status = create_test_status(1, false);
        let priority = create_test_priority(2);
        let author = create_test_user(2, false);

        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let project_repo = Arc::new(MockProjectRepository::with_project(project));
        let user_repo = Arc::new(MockUserRepository::with_user(author));
        let tracker_repo = Arc::new(MockTrackerRepository::with_tracker(tracker));
        let status_repo = Arc::new(MockIssueStatusRepository::with_status(status));
        let enum_repo = Arc::new(MockEnumerationRepository::with_enumeration(priority));
        let journal_repo = Arc::new(MockJournalRepository::new());
        let member_repo = Arc::new(MockMemberRepository::new(false));

        let usecase = UpdateIssueUseCase::new(
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enum_repo,
            journal_repo,
            member_repo,
        );

        let current_user = create_current_user(1, true);
        let dto = UpdateIssueDto {
            subject: Some("Updated Subject".to_string()),
            description: None,
            status_id: None,
            priority_id: None,
            tracker_id: None,
            assigned_to_id: None,
            category_id: None,
            parent_id: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            done_ratio: Some(50),
            is_private: None,
            notes: Some("Progress update".to_string()),
            private_notes: false,
            uploads: None,
        };

        // Execute
        let result = usecase.execute(1, dto, &current_user).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.issue.subject, "Updated Subject");
        assert_eq!(response.issue.done_ratio, 50);
    }

    #[tokio::test]
    async fn test_update_issue_as_member() {
        // Setup
        let issue = create_test_issue(1, 1, 2);
        let project = create_test_project(1);
        let tracker = create_test_tracker(1);
        let status = create_test_status(1, false);
        let priority = create_test_priority(2);
        let author = create_test_user(2, false);

        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let project_repo = Arc::new(MockProjectRepository::with_project(project));
        let user_repo = Arc::new(MockUserRepository::with_user(author));
        let tracker_repo = Arc::new(MockTrackerRepository::with_tracker(tracker));
        let status_repo = Arc::new(MockIssueStatusRepository::with_status(status));
        let enum_repo = Arc::new(MockEnumerationRepository::with_enumeration(priority));
        let journal_repo = Arc::new(MockJournalRepository::new());
        let member_repo = Arc::new(MockMemberRepository::new(true)); // User is a member

        let usecase = UpdateIssueUseCase::new(
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enum_repo,
            journal_repo,
            member_repo,
        );

        let current_user = create_current_user(1, false); // Non-admin
        let dto = UpdateIssueDto {
            subject: Some("Updated by Member".to_string()),
            description: None,
            status_id: None,
            priority_id: None,
            tracker_id: None,
            assigned_to_id: None,
            category_id: None,
            parent_id: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            done_ratio: None,
            is_private: None,
            notes: None,
            private_notes: false,
            uploads: None,
        };

        // Execute
        let result = usecase.execute(1, dto, &current_user).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.issue.subject, "Updated by Member");
    }

    #[tokio::test]
    async fn test_update_issue_non_member_forbidden() {
        // Setup
        let issue = create_test_issue(1, 1, 2);
        let project = create_test_project(1);
        let tracker = create_test_tracker(1);
        let status = create_test_status(1, false);
        let priority = create_test_priority(2);
        let author = create_test_user(2, false);

        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let project_repo = Arc::new(MockProjectRepository::with_project(project));
        let user_repo = Arc::new(MockUserRepository::with_user(author));
        let tracker_repo = Arc::new(MockTrackerRepository::with_tracker(tracker));
        let status_repo = Arc::new(MockIssueStatusRepository::with_status(status));
        let enum_repo = Arc::new(MockEnumerationRepository::with_enumeration(priority));
        let journal_repo = Arc::new(MockJournalRepository::new());
        let member_repo = Arc::new(MockMemberRepository::new(false)); // User is NOT a member

        let usecase = UpdateIssueUseCase::new(
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enum_repo,
            journal_repo,
            member_repo,
        );

        let current_user = create_current_user(1, false); // Non-admin, non-member
        let dto = UpdateIssueDto {
            subject: Some("Should Fail".to_string()),
            description: None,
            status_id: None,
            priority_id: None,
            tracker_id: None,
            assigned_to_id: None,
            category_id: None,
            parent_id: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            done_ratio: None,
            is_private: None,
            notes: None,
            private_notes: false,
            uploads: None,
        };

        // Execute
        let result = usecase.execute(1, dto, &current_user).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Forbidden(msg) => {
                assert!(msg.contains("permission"));
            }
            _ => panic!("Expected Forbidden error"),
        }
    }

    #[tokio::test]
    async fn test_update_issue_not_found() {
        // Setup
        let project = create_test_project(1);
        let tracker = create_test_tracker(1);
        let status = create_test_status(1, false);
        let priority = create_test_priority(2);

        let issue_repo = Arc::new(MockIssueRepository::new()); // Empty repo
        let project_repo = Arc::new(MockProjectRepository::with_project(project));
        let user_repo = Arc::new(MockUserRepository::new());
        let tracker_repo = Arc::new(MockTrackerRepository::with_tracker(tracker));
        let status_repo = Arc::new(MockIssueStatusRepository::with_status(status));
        let enum_repo = Arc::new(MockEnumerationRepository::with_enumeration(priority));
        let journal_repo = Arc::new(MockJournalRepository::new());
        let member_repo = Arc::new(MockMemberRepository::new(false));

        let usecase = UpdateIssueUseCase::new(
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enum_repo,
            journal_repo,
            member_repo,
        );

        let current_user = create_current_user(1, true);
        let dto = UpdateIssueDto {
            subject: Some("Updated".to_string()),
            description: None,
            status_id: None,
            priority_id: None,
            tracker_id: None,
            assigned_to_id: None,
            category_id: None,
            parent_id: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            done_ratio: None,
            is_private: None,
            notes: None,
            private_notes: false,
            uploads: None,
        };

        // Execute
        let result = usecase.execute(999, dto, &current_user).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::NotFound(msg) => {
                assert!(msg.contains("not found"));
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_update_issue_validation_blank_subject() {
        // Setup
        let issue = create_test_issue(1, 1, 2);
        let project = create_test_project(1);
        let tracker = create_test_tracker(1);
        let status = create_test_status(1, false);
        let priority = create_test_priority(2);

        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let project_repo = Arc::new(MockProjectRepository::with_project(project));
        let user_repo = Arc::new(MockUserRepository::new());
        let tracker_repo = Arc::new(MockTrackerRepository::with_tracker(tracker));
        let status_repo = Arc::new(MockIssueStatusRepository::with_status(status));
        let enum_repo = Arc::new(MockEnumerationRepository::with_enumeration(priority));
        let journal_repo = Arc::new(MockJournalRepository::new());
        let member_repo = Arc::new(MockMemberRepository::new(false));

        let usecase = UpdateIssueUseCase::new(
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enum_repo,
            journal_repo,
            member_repo,
        );

        let current_user = create_current_user(1, true);
        let dto = UpdateIssueDto {
            subject: Some("   ".to_string()), // Blank subject
            description: None,
            status_id: None,
            priority_id: None,
            tracker_id: None,
            assigned_to_id: None,
            category_id: None,
            parent_id: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            done_ratio: None,
            is_private: None,
            notes: None,
            private_notes: false,
            uploads: None,
        };

        // Execute
        let result = usecase.execute(1, dto, &current_user).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Validation(msg) => {
                assert!(msg.contains("blank"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_update_issue_validation_invalid_done_ratio() {
        // Setup
        let issue = create_test_issue(1, 1, 2);
        let project = create_test_project(1);
        let tracker = create_test_tracker(1);
        let status = create_test_status(1, false);
        let priority = create_test_priority(2);

        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let project_repo = Arc::new(MockProjectRepository::with_project(project));
        let user_repo = Arc::new(MockUserRepository::new());
        let tracker_repo = Arc::new(MockTrackerRepository::with_tracker(tracker));
        let status_repo = Arc::new(MockIssueStatusRepository::with_status(status));
        let enum_repo = Arc::new(MockEnumerationRepository::with_enumeration(priority));
        let journal_repo = Arc::new(MockJournalRepository::new());
        let member_repo = Arc::new(MockMemberRepository::new(false));

        let usecase = UpdateIssueUseCase::new(
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enum_repo,
            journal_repo,
            member_repo,
        );

        let current_user = create_current_user(1, true);
        let dto = UpdateIssueDto {
            subject: None,
            description: None,
            status_id: None,
            priority_id: None,
            tracker_id: None,
            assigned_to_id: None,
            category_id: None,
            parent_id: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            done_ratio: Some(150), // Invalid: > 100
            is_private: None,
            notes: None,
            private_notes: false,
            uploads: None,
        };

        // Execute
        let result = usecase.execute(1, dto, &current_user).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Validation(msg) => {
                assert!(msg.contains("0 and 100") || msg.contains("between"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_update_issue_no_changes() {
        // Setup
        let issue = create_test_issue(1, 1, 2);
        let project = create_test_project(1);
        let tracker = create_test_tracker(1);
        let status = create_test_status(1, false);
        let priority = create_test_priority(2);

        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let project_repo = Arc::new(MockProjectRepository::with_project(project));
        let user_repo = Arc::new(MockUserRepository::new());
        let tracker_repo = Arc::new(MockTrackerRepository::with_tracker(tracker));
        let status_repo = Arc::new(MockIssueStatusRepository::with_status(status));
        let enum_repo = Arc::new(MockEnumerationRepository::with_enumeration(priority));
        let journal_repo = Arc::new(MockJournalRepository::new());
        let member_repo = Arc::new(MockMemberRepository::new(false));

        let usecase = UpdateIssueUseCase::new(
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enum_repo,
            journal_repo,
            member_repo,
        );

        let current_user = create_current_user(1, true);
        let dto = UpdateIssueDto {
            subject: None,
            description: None,
            status_id: None,
            priority_id: None,
            tracker_id: None,
            assigned_to_id: None,
            category_id: None,
            parent_id: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            done_ratio: None,
            is_private: None,
            notes: None,
            private_notes: false,
            uploads: None,
        };

        // Execute
        let result = usecase.execute(1, dto, &current_user).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Validation(msg) => {
                assert!(msg.contains("No changes"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_update_issue_invalid_status() {
        // Setup
        let issue = create_test_issue(1, 1, 2);
        let project = create_test_project(1);
        let tracker = create_test_tracker(1);
        let status = create_test_status(1, false);
        let priority = create_test_priority(2);

        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let project_repo = Arc::new(MockProjectRepository::with_project(project));
        let user_repo = Arc::new(MockUserRepository::new());
        let tracker_repo = Arc::new(MockTrackerRepository::with_tracker(tracker));
        let status_repo = Arc::new(MockIssueStatusRepository::with_status(status));
        let enum_repo = Arc::new(MockEnumerationRepository::with_enumeration(priority));
        let journal_repo = Arc::new(MockJournalRepository::new());
        let member_repo = Arc::new(MockMemberRepository::new(false));

        let usecase = UpdateIssueUseCase::new(
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enum_repo,
            journal_repo,
            member_repo,
        );

        let current_user = create_current_user(1, true);
        let dto = UpdateIssueDto {
            subject: None,
            description: None,
            status_id: Some(999), // Non-existent status
            priority_id: None,
            tracker_id: None,
            assigned_to_id: None,
            category_id: None,
            parent_id: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            done_ratio: None,
            is_private: None,
            notes: None,
            private_notes: false,
            uploads: None,
        };

        // Execute
        let result = usecase.execute(1, dto, &current_user).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            ApplicationError::Validation(msg) => {
                assert!(msg.contains("Status is invalid"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_update_issue_with_notes_only() {
        // Setup - only notes, no other changes
        let issue = create_test_issue(1, 1, 2);
        let project = create_test_project(1);
        let author = create_test_user(2, false);
        let tracker = create_test_tracker(1);
        let status = create_test_status(1, false);
        let priority = create_test_priority(2);

        let issue_repo = Arc::new(MockIssueRepository::with_issue(issue));
        let project_repo = Arc::new(MockProjectRepository::with_project(project));
        let user_repo = Arc::new(MockUserRepository::with_user(author));
        let tracker_repo = Arc::new(MockTrackerRepository::with_tracker(tracker));
        let status_repo = Arc::new(MockIssueStatusRepository::with_status(status));
        let enum_repo = Arc::new(MockEnumerationRepository::with_enumeration(priority));
        let journal_repo = Arc::new(MockJournalRepository::new());
        let member_repo = Arc::new(MockMemberRepository::new(false));

        let usecase = UpdateIssueUseCase::new(
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enum_repo,
            journal_repo,
            member_repo,
        );

        let current_user = create_current_user(1, true);
        let dto = UpdateIssueDto {
            subject: None,
            description: None,
            status_id: None,
            priority_id: None,
            tracker_id: None,
            assigned_to_id: None,
            category_id: None,
            parent_id: None,
            start_date: None,
            due_date: None,
            estimated_hours: None,
            done_ratio: None,
            is_private: None,
            notes: Some("Just a comment".to_string()),
            private_notes: false,
            uploads: None,
        };

        // Execute
        let result = usecase.execute(1, dto, &current_user).await;

        // Assert - should succeed with just a note
        assert!(result.is_ok());
    }
}
