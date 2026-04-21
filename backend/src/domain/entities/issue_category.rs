use serde::{Deserialize, Serialize};

/// Issue category entity for grouping issues within a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCategory {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub assigned_to_id: Option<i32>,
}
