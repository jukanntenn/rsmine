use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Member domain entity (pure Rust struct, independent of any ORM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub created_on: Option<DateTime<Utc>>,
    pub mail_notification: bool,
}

/// MemberRole domain entity (pure Rust struct, independent of any ORM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberRole {
    pub id: i32,
    pub member_id: i32,
    pub role_id: i32,
    pub inherited_from: Option<i32>,
}

/// Role domain entity (pure Rust struct, independent of any ORM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub position: Option<i32>,
    pub assignable: bool,
    pub builtin: i32,
    pub permissions: Option<String>,
    pub issues_visibility: String,
    pub users_visibility: String,
    pub time_entries_visibility: String,
    pub all_roles_managed: bool,
    pub settings: Option<String>,
    pub default_time_entry_activity_id: Option<i32>,
}

/// Role with inheritance information for member membership
#[derive(Debug, Clone)]
pub struct RoleWithInheritance {
    pub role: Role,
    pub inherited_from: Option<i32>,
}

impl RoleWithInheritance {
    /// Check if this role is inherited from a parent project
    pub fn is_inherited(&self) -> bool {
        self.inherited_from.is_some()
    }
}

/// Composite structure for member with user and roles (with inheritance info)
#[derive(Debug, Clone)]
pub struct MemberWithRoles {
    pub member: Member,
    pub user: crate::domain::entities::User,
    pub roles: Vec<RoleWithInheritance>,
}
