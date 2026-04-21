use crate::application::use_cases::{RelationItem, RelationListResponse};
use serde::Serialize;

/// JSON response wrapper for the relation list endpoint
#[derive(Debug, Serialize)]
pub struct RelationListJsonResponse {
    pub relations: Vec<RelationItemJsonResponse>,
}

/// JSON representation of a single issue relation
#[derive(Debug, Serialize)]
pub struct RelationItemJsonResponse {
    pub id: i32,
    pub issue_id: i32,
    pub issue_to_id: i32,
    pub relation_type: String,
    pub delay: Option<i32>,
}

impl From<RelationListResponse> for RelationListJsonResponse {
    fn from(response: RelationListResponse) -> Self {
        Self {
            relations: response
                .relations
                .into_iter()
                .map(RelationItemJsonResponse::from)
                .collect(),
        }
    }
}

impl From<RelationItem> for RelationItemJsonResponse {
    fn from(item: RelationItem) -> Self {
        Self {
            id: item.id,
            issue_id: item.issue_id,
            issue_to_id: item.issue_to_id,
            relation_type: item.relation_type,
            delay: item.delay,
        }
    }
}
