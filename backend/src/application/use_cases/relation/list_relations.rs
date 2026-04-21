use crate::application::errors::ApplicationError;
use crate::domain::entities::Issue;
use crate::domain::repositories::{IssueRelationRepository, IssueRepository, ProjectRepository};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response for a single issue relation
#[derive(Debug, Clone)]
pub struct RelationItem {
    pub id: i32,
    pub issue_id: i32,
    pub issue_to_id: i32,
    pub relation_type: String,
    pub delay: Option<i32>,
}

/// Response for the list relations endpoint
#[derive(Debug, Clone)]
pub struct RelationListResponse {
    pub relations: Vec<RelationItem>,
}

/// Use case for listing relations of an issue
pub struct ListRelationsUseCase<I, P, R>
where
    I: IssueRepository,
    P: ProjectRepository,
    R: IssueRelationRepository,
{
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    relation_repo: Arc<R>,
}

impl<I, P, R> ListRelationsUseCase<I, P, R>
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
    /// - Admin users can see relations of all issues
    /// - Regular users can see relations of issues in public projects
    /// - Regular users can see relations of issues in projects they are members of
    pub async fn execute(
        &self,
        issue_id: i32,
        current_user: &CurrentUser,
    ) -> Result<RelationListResponse, ApplicationError> {
        // Get the issue
        let issue = self
            .issue_repo
            .find_by_id(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Issue {} not found", issue_id)))?;

        // Check visibility
        self.check_issue_visibility(&issue, current_user).await?;

        // Get relations for the issue (both from and to)
        let relations = self
            .relation_repo
            .find_by_issue(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        // Convert to response format
        let relation_items: Vec<RelationItem> = relations
            .into_iter()
            .map(|r| RelationItem {
                id: r.id,
                issue_id: r.issue_from_id,
                issue_to_id: r.issue_to_id,
                relation_type: r.relation_type,
                delay: r.delay,
            })
            .collect();

        Ok(RelationListResponse {
            relations: relation_items,
        })
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

        Err(ApplicationError::Forbidden(
            "You don't have permission to view this issue".into(),
        ))
    }
}
