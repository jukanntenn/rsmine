use serde::{Deserialize, Serialize};

/// IssueStatus domain entity (pure Rust struct, independent of any ORM)
/// Represents issue statuses like New, In Progress, Resolved, Closed, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueStatus {
    pub id: i32,
    pub name: String,
    pub position: Option<i32>,
    pub is_closed: bool,
    pub is_default: bool,
    pub default_done_ratio: Option<i32>,
}

impl IssueStatus {
    /// Check if this status represents a closed issue
    pub fn is_issue_closed(&self) -> bool {
        self.is_closed
    }

    /// Get the default done ratio for this status
    pub fn done_ratio(&self) -> Option<i32> {
        self.default_done_ratio
    }
}
