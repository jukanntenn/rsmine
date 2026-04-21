use serde::{Deserialize, Serialize};

/// Enumeration type constants
pub const ENUM_TYPE_ISSUE_PRIORITY: &str = "IssuePriority";

/// Enumeration entity representing various enumeration types
/// (priorities, time entry activities, document categories, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enumeration {
    pub id: i32,
    pub name: String,
    pub position: Option<i32>,
    pub is_default: bool,
    pub enum_type: String,
    pub active: bool,
    pub project_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub position_name: Option<String>,
}

impl Enumeration {
    /// Create a new enumeration
    pub fn new(
        id: i32,
        name: String,
        position: Option<i32>,
        is_default: bool,
        enum_type: String,
        active: bool,
    ) -> Self {
        Self {
            id,
            name,
            position,
            is_default,
            enum_type,
            active,
            project_id: None,
            parent_id: None,
            position_name: None,
        }
    }
}
