use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Project status constants
pub const PROJECT_STATUS_ACTIVE: i32 = 1;
pub const PROJECT_STATUS_CLOSED: i32 = 5;
pub const PROJECT_STATUS_ARCHIVED: i32 = 9;

/// Project domain entity (pure Rust struct, independent of any ORM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub is_public: bool,
    pub parent_id: Option<i32>,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
    pub identifier: Option<String>,
    pub status: i32,
    pub lft: Option<i32>,
    pub rgt: Option<i32>,
    pub inherit_members: bool,
    pub default_version_id: Option<i32>,
    pub default_assigned_to_id: Option<i32>,
}

impl Project {
    /// Check if the project is active
    pub fn is_active(&self) -> bool {
        self.status == PROJECT_STATUS_ACTIVE
    }

    /// Check if the project is closed
    pub fn is_closed(&self) -> bool {
        self.status == PROJECT_STATUS_CLOSED
    }

    /// Check if the project is archived
    pub fn is_archived(&self) -> bool {
        self.status == PROJECT_STATUS_ARCHIVED
    }
}
