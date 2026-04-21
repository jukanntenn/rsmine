use crate::application::errors::ApplicationError;
use crate::application::use_cases::project::NamedId;
use crate::domain::entities::Issue;
use crate::domain::repositories::{
    EnumerationRepository, IssueQueryParams, IssueRepository, IssueStatusRepository,
    ProjectRepository, TrackerRepository, UserRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Status info with is_closed flag
#[derive(Debug, Clone)]
pub struct StatusInfo {
    pub id: i32,
    pub name: String,
    pub is_closed: bool,
}

/// Issue item for list responses with related entity info
#[derive(Debug, Clone)]
pub struct IssueItem {
    pub id: i32,
    pub project: NamedId,
    pub tracker: NamedId,
    pub status: StatusInfo,
    pub priority: NamedId,
    pub author: NamedId,
    pub assigned_to: Option<NamedId>,
    pub subject: String,
    pub description: String,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub done_ratio: i32,
    pub is_private: bool,
    pub estimated_hours: Option<f64>,
    pub total_estimated_hours: Option<f64>,
    pub spent_hours: f64,
    pub total_spent_hours: f64,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
    pub closed_on: Option<String>,
}

/// Response for issue list endpoint
#[derive(Debug, Clone)]
pub struct IssueListResponse {
    pub issues: Vec<IssueItem>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

/// Use case for listing issues
pub struct ListIssuesUseCase<I, P, U, T, S, E>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
{
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    user_repo: Arc<U>,
    tracker_repo: Arc<T>,
    status_repo: Arc<S>,
    enumeration_repo: Arc<E>,
}

impl<I, P, U, T, S, E> ListIssuesUseCase<I, P, U, T, S, E>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    T: TrackerRepository,
    S: IssueStatusRepository,
    E: EnumerationRepository,
{
    pub fn new(
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        user_repo: Arc<U>,
        tracker_repo: Arc<T>,
        status_repo: Arc<S>,
        enumeration_repo: Arc<E>,
    ) -> Self {
        Self {
            issue_repo,
            project_repo,
            user_repo,
            tracker_repo,
            status_repo,
            enumeration_repo,
        }
    }

    /// Execute the use case
    ///
    /// Visibility rules:
    /// - Admin users can see all issues
    /// - Regular users can see issues in projects they have access to
    pub async fn execute(
        &self,
        mut params: IssueQueryParams,
        current_user: &CurrentUser,
    ) -> Result<IssueListResponse, ApplicationError> {
        // Handle "me" for assigned_to_id
        if let Some(ref assigned) = params.assigned_to_id {
            if assigned == "me" {
                params.assigned_to_id = Some(current_user.id.to_string());
            }
        }

        // Check project access if project_id is specified
        if let Some(project_id) = params.project_id {
            if !current_user.admin {
                let project = self
                    .project_repo
                    .find_by_id(project_id)
                    .await
                    .map_err(|e| ApplicationError::Internal(e.to_string()))?
                    .ok_or_else(|| {
                        ApplicationError::NotFound(format!("Project {} not found", project_id))
                    })?;

                // Check if user has access to this project
                if !project.is_public {
                    let member_project_ids = self
                        .project_repo
                        .find_project_ids_by_user_membership(current_user.id)
                        .await
                        .map_err(|e| ApplicationError::Internal(e.to_string()))?;

                    if !member_project_ids.contains(&project_id) {
                        return Err(ApplicationError::Forbidden(
                            "You don't have access to this project".into(),
                        ));
                    }
                }
            }
        }

        // Get issues
        let issues = self
            .issue_repo
            .find_all(params.clone())
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Get total count
        let total_count = self
            .issue_repo
            .count(&params)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Build response with related entities
        let issue_items = self.build_issue_items(issues).await?;

        Ok(IssueListResponse {
            issues: issue_items,
            total_count,
            offset: params.offset,
            limit: params.limit,
        })
    }

    /// Build issue items with related entity information
    async fn build_issue_items(
        &self,
        issues: Vec<Issue>,
    ) -> Result<Vec<IssueItem>, ApplicationError> {
        let mut items = Vec::with_capacity(issues.len());

        for issue in issues {
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

            items.push(IssueItem {
                id: issue.id,
                project: project_info,
                tracker: tracker_info,
                status: status_info,
                priority: priority_info,
                author: author_info,
                assigned_to: assigned_to_info,
                subject: issue.subject,
                description: issue.description.unwrap_or_default(),
                start_date: issue.start_date.map(|d| d.to_string()),
                due_date: issue.due_date.map(|d| d.to_string()),
                done_ratio: issue.done_ratio,
                is_private: issue.is_private,
                estimated_hours: issue.estimated_hours,
                // For MVP, total_estimated_hours = estimated_hours (no subtask calculation)
                total_estimated_hours: issue.estimated_hours,
                // For MVP, spent_hours is 0 (no time tracking)
                spent_hours: 0.0,
                total_spent_hours: 0.0,
                created_on: issue.created_on.map(|d| d.to_rfc3339()),
                updated_on: issue.updated_on.map(|d| d.to_rfc3339()),
                closed_on: issue.closed_on.map(|d| d.to_rfc3339()),
            });
        }

        Ok(items)
    }
}
