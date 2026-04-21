use crate::application::errors::ApplicationError;
use crate::domain::entities::Project;
use crate::domain::repositories::{ProjectQueryParams, ProjectRepository};
use crate::presentation::middleware::CurrentUser;
use std::sync::Arc;

/// Project summary for list responses
#[derive(Debug, Clone)]
pub struct ProjectSummary {
    pub id: i32,
    pub name: String,
    pub identifier: String,
    pub description: String,
    pub homepage: String,
    pub status: i32,
    pub is_public: bool,
    pub inherit_members: bool,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
}

impl From<Project> for ProjectSummary {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            name: project.name,
            identifier: project.identifier.unwrap_or_default(),
            description: project.description.unwrap_or_default(),
            homepage: project.homepage.unwrap_or_default(),
            status: project.status,
            is_public: project.is_public,
            inherit_members: project.inherit_members,
            created_on: project.created_on.map(|d| d.to_rfc3339()),
            updated_on: project.updated_on.map(|d| d.to_rfc3339()),
        }
    }
}

/// Response for project list endpoint
#[derive(Debug, Clone)]
pub struct ProjectListResponse {
    pub projects: Vec<ProjectSummary>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

/// Use case for listing projects
pub struct ListProjectsUseCase<P: ProjectRepository> {
    project_repo: Arc<P>,
}

impl<P: ProjectRepository> ListProjectsUseCase<P> {
    pub fn new(project_repo: Arc<P>) -> Self {
        Self { project_repo }
    }

    /// Execute the use case
    ///
    /// Visibility rules:
    /// - Admin users can see all projects
    /// - Regular users can see public projects + projects they are a member of
    pub async fn execute(
        &self,
        params: ProjectQueryParams,
        current_user: &CurrentUser,
    ) -> Result<ProjectListResponse, ApplicationError> {
        // Admin can see all projects
        // Regular users can see public projects + projects they are member of
        let (projects, total_count) = if current_user.admin {
            let projects = self
                .project_repo
                .find_all(params.clone())
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
            let count = self
                .project_repo
                .count(&params)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
            (projects, count)
        } else {
            let projects = self
                .project_repo
                .find_visible_for_user(current_user.id, params.clone())
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
            let count = self
                .project_repo
                .count_visible_for_user(current_user.id, &params)
                .await
                .map_err(|e| ApplicationError::Internal(e.to_string()))?;
            (projects, count)
        };

        let project_summaries: Vec<ProjectSummary> =
            projects.into_iter().map(ProjectSummary::from).collect();

        Ok(ProjectListResponse {
            projects: project_summaries,
            total_count,
            offset: params.offset,
            limit: params.limit,
        })
    }
}
