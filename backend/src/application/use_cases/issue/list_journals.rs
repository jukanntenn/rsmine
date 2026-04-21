use super::get_issue::{JournalDetailResponse, JournalResponse};
use crate::application::errors::ApplicationError;
use crate::application::use_cases::project::NamedId;
use crate::domain::entities::Issue;
use crate::domain::repositories::{
    IssueRepository, JournalRepository, ProjectRepository, UserRepository,
};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Response for the list journals endpoint
#[derive(Debug, Clone)]
pub struct ListJournalsResponse {
    pub journals: Vec<JournalResponse>,
}

/// Use case for listing journals (change history) for an issue
pub struct ListJournalsUseCase<I, P, U, J>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    J: JournalRepository,
{
    issue_repo: Arc<I>,
    project_repo: Arc<P>,
    user_repo: Arc<U>,
    journal_repo: Arc<J>,
}

impl<I, P, U, J> ListJournalsUseCase<I, P, U, J>
where
    I: IssueRepository,
    P: ProjectRepository,
    U: UserRepository,
    J: JournalRepository,
{
    pub fn new(
        issue_repo: Arc<I>,
        project_repo: Arc<P>,
        user_repo: Arc<U>,
        journal_repo: Arc<J>,
    ) -> Self {
        Self {
            issue_repo,
            project_repo,
            user_repo,
            journal_repo,
        }
    }

    /// Execute the use case
    ///
    /// Visibility rules:
    /// - Admin users can see all journals
    /// - Regular users can see journals for issues in public projects
    /// - Regular users can see journals for issues in projects they are members of
    /// - Private notes are only visible to admin users (MVP)
    pub async fn execute(
        &self,
        issue_id: i32,
        current_user: &CurrentUser,
    ) -> Result<ListJournalsResponse, ApplicationError> {
        // Get the issue
        let issue = self
            .issue_repo
            .find_by_id(issue_id)
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound(format!("Issue {} not found", issue_id)))?;

        // Check visibility
        self.check_issue_visibility(&issue, current_user).await?;

        // Get journals for the issue
        let journals = self.get_journals(issue_id, current_user).await?;

        Ok(ListJournalsResponse { journals })
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

    /// Get journals (change history) for the issue
    async fn get_journals(
        &self,
        issue_id: i32,
        current_user: &CurrentUser,
    ) -> Result<Vec<JournalResponse>, ApplicationError> {
        let journals = self
            .journal_repo
            .find_by_journalized(issue_id, "Issue")
            .await
            .map_err(|e| ApplicationError::Internal(e.to_string()))?;

        let mut responses = Vec::with_capacity(journals.len());
        for journal in journals {
            // Skip private notes for non-admin users (simplified for MVP)
            if journal.private_notes && !current_user.admin {
                continue;
            }

            let user = self
                .user_repo
                .find_by_id(journal.user_id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            let user_info = user
                .map(|u| NamedId {
                    id: u.id,
                    name: format!("{} {}", u.firstname, u.lastname),
                })
                .unwrap_or(NamedId {
                    id: journal.user_id,
                    name: format!("User {}", journal.user_id),
                });

            let details = self
                .journal_repo
                .find_details(journal.id)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;

            responses.push(JournalResponse {
                id: journal.id,
                user: user_info,
                notes: journal.notes,
                created_on: journal.created_on.to_rfc3339(),
                updated_on: journal.updated_on.map(|d| d.to_rfc3339()),
                private_notes: journal.private_notes,
                details: details
                    .into_iter()
                    .map(JournalDetailResponse::from)
                    .collect(),
            });
        }

        Ok(responses)
    }
}
