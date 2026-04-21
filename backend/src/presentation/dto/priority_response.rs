use crate::application::use_cases::{PriorityItem, PriorityListResponse};
use serde::{Deserialize, Serialize};

/// Priority JSON response
#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityJson {
    pub id: i32,
    pub name: String,
    pub is_default: bool,
    pub active: bool,
}

impl From<PriorityItem> for PriorityJson {
    fn from(item: PriorityItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            is_default: item.is_default,
            active: item.active,
        }
    }
}

/// Response for GET /api/v1/enumerations/issue_priorities.json
#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityListJsonResponse {
    pub issue_priorities: Vec<PriorityJson>,
}

impl From<PriorityListResponse> for PriorityListJsonResponse {
    fn from(response: PriorityListResponse) -> Self {
        Self {
            issue_priorities: response
                .issue_priorities
                .into_iter()
                .map(PriorityJson::from)
                .collect(),
        }
    }
}
