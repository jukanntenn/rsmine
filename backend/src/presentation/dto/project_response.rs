use crate::application::use_cases::project::NamedId;
use crate::application::use_cases::{
    CreateProjectResponse, GetProjectResponse, ProjectDetail, ProjectListResponse, ProjectSummary,
    UpdateProjectResponse,
};
use serde::Serialize;

/// JSON response for project list endpoint
#[derive(Debug, Serialize)]
pub struct ProjectListJsonResponse {
    pub projects: Vec<ProjectItemJsonResponse>,
    pub total_count: u32,
    pub offset: u32,
    pub limit: u32,
}

/// JSON representation of a single project in list responses
#[derive(Debug, Serialize)]
pub struct ProjectItemJsonResponse {
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

/// JSON representation of a named ID (e.g., default assignee)
#[derive(Debug, Serialize)]
pub struct NamedIdJsonResponse {
    pub id: i32,
    pub name: String,
}

impl From<NamedId> for NamedIdJsonResponse {
    fn from(named_id: NamedId) -> Self {
        Self {
            id: named_id.id,
            name: named_id.name,
        }
    }
}

/// JSON representation of a single project detail for get endpoint
#[derive(Debug, Serialize)]
pub struct ProjectDetailJsonResponse {
    pub id: i32,
    pub name: String,
    pub identifier: String,
    pub description: String,
    pub homepage: String,
    pub status: i32,
    pub is_public: bool,
    pub inherit_members: bool,
    pub default_assignee: Option<NamedIdJsonResponse>,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
}

/// JSON response for get project endpoint
#[derive(Debug, Serialize)]
pub struct GetProjectJsonResponse {
    pub project: ProjectDetailJsonResponse,
}

/// JSON response for create project endpoint
#[derive(Debug, Serialize)]
pub struct CreateProjectJsonResponse {
    pub project: ProjectDetailJsonResponse,
}

impl From<ProjectListResponse> for ProjectListJsonResponse {
    fn from(response: ProjectListResponse) -> Self {
        Self {
            projects: response
                .projects
                .into_iter()
                .map(ProjectItemJsonResponse::from)
                .collect(),
            total_count: response.total_count,
            offset: response.offset,
            limit: response.limit,
        }
    }
}

impl From<ProjectSummary> for ProjectItemJsonResponse {
    fn from(item: ProjectSummary) -> Self {
        Self {
            id: item.id,
            name: item.name,
            identifier: item.identifier,
            description: item.description,
            homepage: item.homepage,
            status: item.status,
            is_public: item.is_public,
            inherit_members: item.inherit_members,
            created_on: item.created_on,
            updated_on: item.updated_on,
        }
    }
}

impl From<ProjectDetail> for ProjectDetailJsonResponse {
    fn from(detail: ProjectDetail) -> Self {
        Self {
            id: detail.id,
            name: detail.name,
            identifier: detail.identifier,
            description: detail.description,
            homepage: detail.homepage,
            status: detail.status,
            is_public: detail.is_public,
            inherit_members: detail.inherit_members,
            default_assignee: detail.default_assignee.map(NamedIdJsonResponse::from),
            created_on: detail.created_on,
            updated_on: detail.updated_on,
        }
    }
}

impl From<GetProjectResponse> for GetProjectJsonResponse {
    fn from(response: GetProjectResponse) -> Self {
        Self {
            project: ProjectDetailJsonResponse::from(response.project),
        }
    }
}

impl From<CreateProjectResponse> for CreateProjectJsonResponse {
    fn from(response: CreateProjectResponse) -> Self {
        Self {
            project: ProjectDetailJsonResponse::from(response.project),
        }
    }
}

/// JSON response for update project endpoint
#[derive(Debug, Serialize)]
pub struct UpdateProjectJsonResponse {
    pub project: ProjectDetailJsonResponse,
}

impl From<UpdateProjectResponse> for UpdateProjectJsonResponse {
    fn from(response: UpdateProjectResponse) -> Self {
        Self {
            project: ProjectDetailJsonResponse::from(response.project),
        }
    }
}
