use crate::application::use_cases::{
    CreateIssueStatusResponse, GetIssueStatusResponse, IssueStatusItem, IssueStatusListResponse,
    UpdateIssueStatusResponse,
};
use serde::{Deserialize, Serialize};

/// Issue status JSON response
#[derive(Debug, Serialize, Deserialize)]
pub struct IssueStatusJson {
    pub id: i32,
    pub name: String,
    pub is_closed: bool,
    pub is_default: Option<bool>,
    pub default_done_ratio: Option<i32>,
    pub description: Option<String>,
}

impl From<IssueStatusItem> for IssueStatusJson {
    fn from(item: IssueStatusItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            is_closed: item.is_closed,
            is_default: Some(item.is_default),
            default_done_ratio: item.default_done_ratio,
            description: None,
        }
    }
}

impl From<&IssueStatusItem> for IssueStatusJson {
    fn from(item: &IssueStatusItem) -> Self {
        Self {
            id: item.id,
            name: item.name.clone(),
            is_closed: item.is_closed,
            is_default: Some(item.is_default),
            default_done_ratio: item.default_done_ratio,
            description: None,
        }
    }
}

/// Response for GET /api/v1/issue_statuses.json
#[derive(Debug, Serialize, Deserialize)]
pub struct IssueStatusListJsonResponse {
    pub issue_statuses: Vec<IssueStatusJson>,
}

impl From<IssueStatusListResponse> for IssueStatusListJsonResponse {
    fn from(response: IssueStatusListResponse) -> Self {
        Self {
            issue_statuses: response
                .issue_statuses
                .into_iter()
                .map(IssueStatusJson::from)
                .collect(),
        }
    }
}

/// Response for GET /api/v1/issue_statuses/:id.json
#[derive(Debug, Serialize, Deserialize)]
pub struct GetIssueStatusJsonResponse {
    pub issue_status: IssueStatusJson,
}

impl From<GetIssueStatusResponse> for GetIssueStatusJsonResponse {
    fn from(response: GetIssueStatusResponse) -> Self {
        Self {
            issue_status: IssueStatusJson::from(response.issue_status),
        }
    }
}

/// Response for POST /api/v1/issue_statuses.json
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIssueStatusJsonResponse {
    pub issue_status: IssueStatusJson,
}

impl From<CreateIssueStatusResponse> for CreateIssueStatusJsonResponse {
    fn from(response: CreateIssueStatusResponse) -> Self {
        Self {
            issue_status: IssueStatusJson::from(response.issue_status),
        }
    }
}

/// Response for PUT /api/v1/issue_statuses/:id.json
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIssueStatusJsonResponse {
    pub issue_status: IssueStatusJson,
}

impl From<UpdateIssueStatusResponse> for UpdateIssueStatusJsonResponse {
    fn from(response: UpdateIssueStatusResponse) -> Self {
        Self {
            issue_status: IssueStatusJson::from(response.issue_status),
        }
    }
}
