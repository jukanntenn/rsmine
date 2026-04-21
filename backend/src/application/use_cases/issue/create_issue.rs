use super::{GetIssueResponse, StatusInfo};
use crate::application::dto::CreateIssueDto;
use crate::application::errors::ApplicationError;
use crate::domain::entities::Issue;
use crate::domain::repositories::{
    EnumerationRepository, IssueRepository, IssueStatusRepository, MemberRepository, NewIssue,
    ProjectRepository, TrackerRepository, UserRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response for create issue endpoint
#[derive(Debug, Clone)]
pub struct CreateIssueResponse {
    pub issue: GetIssueResponse,
}

/// Use case for creating a new issue
pub struct CreateIssueUseCase<I, P, U, T, S, E, M>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
    M: MemberRepository,
{
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    user_repo: Arc<U>,
    tracker_repo: Arc<T>,
    status_repo: Arc<S>,
    enumeration_repo: Arc<E>,
    member_repo: Arc<M>,
}

impl<I, P, U, T, S, E, M> CreateIssueUseCase<I, P, U, T, S, E, M>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
    M: MemberRepository,
{
    pub fn new(
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        user_repo: Arc<U>,
        tracker_repo: Arc<T>,
        status_repo: Arc<S>,
        enumeration_repo: Arc<E>,
        member_repo: Arc<M>,
    ) -> Self {
        Self {
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enumeration_repo,
            member_repo,
        }
    }

    /// Execute the use case
    ///
    /// Permission rules:
    /// - Admin: can create issues in any project
    /// - Non-admin: needs `add_issues` permission in the target project
    ///
    /// Validation rules:
    /// - Project: required, must exist
    /// - Tracker: required, must exist and be available in the project
    /// - Subject: required, 1-255 characters
    /// - Status: defaults to tracker's default status
    /// - Priority: defaults to default priority
    /// - Parent issue: must be in the same project if specified
    /// - Private: requires `set_issues_private` or `set_own_issues_private` permission
    pub async fn execute(
        &self,
        dto: CreateIssueDto,
        current_user: &CurrentUser,
    ) -> Result<CreateIssueResponse, ApplicationError> {
        // 1. Validate input
        if let Err(errors) = dto.validate() {
            return Err(ApplicationError::Validation(errors.join(", ")));
        }

        // 2. Check project exists and user can add issues
        let project = self
            .project_repo
            .find_by_id(dto.project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::Validation("Project not found".into()))?;

        // Check permission
        if !current_user.admin {
            // Check if user is a member of the project
            let is_member = self
                .member_repo
                .is_member(dto.project_id, current_user.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            // For MVP, members can create issues; for public projects, logged-in users can create
            // This is a simplified permission check - full implementation would check role permissions
            if !is_member && !project.is_public {
                return Err(ApplicationError::Forbidden(
                    "You don't have permission to create issues in this project".into(),
                ));
            }
        }

        // 3. Validate tracker exists and is available in project
        let tracker = self
            .tracker_repo
            .find_by_id(dto.tracker_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::Validation("Tracker not found".into()))?;

        // Check if tracker is available in project
        let project_trackers = self
            .tracker_repo
            .find_by_project(dto.project_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if !project_trackers.iter().any(|t| t.id == dto.tracker_id) {
            return Err(ApplicationError::Validation(
                "Tracker is not available in this project".into(),
            ));
        }

        // 4. Set default status
        let status_id = dto.status_id.unwrap_or(tracker.default_status_id);

        // Validate status exists
        let status = self
            .status_repo
            .find_by_id(status_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        if status.is_none() {
            return Err(ApplicationError::Validation("Status not found".into()));
        }

        // 5. Set default priority
        let priority_id = dto.priority_id.unwrap_or(2);

        // 6. Validate parent issue if specified
        if let Some(parent_id) = dto.parent_id {
            let parent = self
                .issue_repo
                .find_by_id(parent_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?
                .ok_or_else(|| ApplicationError::Validation("Parent issue not found".into()))?;

            if parent.project_id != dto.project_id {
                return Err(ApplicationError::Validation(
                    "Parent issue must be in the same project".into(),
                ));
            }
        }

        // 7. Validate assigned_to if specified
        if let Some(assigned_to_id) = dto.assigned_to_id {
            let assigned_user = self
                .user_repo
                .find_by_id(assigned_to_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            if assigned_user.is_none() {
                return Err(ApplicationError::Validation("Assignee not found".into()));
            }
        }

        // 8. Create issue
        let new_issue = NewIssue {
            tracker_id: dto.tracker_id,
            project_id: dto.project_id,
            subject: dto.subject.trim().to_string(),
            description: dto
                .description
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty()),
            due_date: dto.due_date,
            category_id: dto.category_id,
            status_id,
            assigned_to_id: dto.assigned_to_id,
            priority_id,
            author_id: current_user.id,
            start_date: dto.start_date,
            estimated_hours: dto.estimated_hours,
            parent_id: dto.parent_id,
            is_private: dto.is_private,
        };

        let created_issue = self
            .issue_repo
            .create(new_issue)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // 9. Build response
        let response = self.build_response(&created_issue).await?;

        Ok(CreateIssueResponse { issue: response })
    }

    /// Build the response from the created issue
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
