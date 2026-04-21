use serde::{Deserialize, Serialize};

/// Issue relation entity representing relationships between issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueRelation {
    pub id: i32,
    pub issue_from_id: i32,
    pub issue_to_id: i32,
    pub relation_type: String,
    pub delay: Option<i32>,
}
