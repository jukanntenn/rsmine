use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User status constants
pub const USER_STATUS_ACTIVE: i32 = 1;
pub const USER_STATUS_REGISTERED: i32 = 2;
pub const USER_STATUS_LOCKED: i32 = 3;

/// User domain entity (pure Rust struct, independent of any ORM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub hashed_password: Option<String>,
    pub firstname: String,
    pub lastname: String,
    pub admin: bool,
    pub status: i32,
    pub last_login_on: Option<DateTime<Utc>>,
    pub language: Option<String>,
    pub auth_source_id: Option<i32>,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
    pub r#type: Option<String>,
    pub mail_notification: String,
    pub salt: Option<String>,
    pub must_change_passwd: bool,
    pub passwd_changed_on: Option<DateTime<Utc>>,
    pub twofa_scheme: Option<String>,
    pub twofa_totp_key: Option<String>,
    pub twofa_totp_last_used_at: Option<i32>,
    pub twofa_required: bool,
}

impl User {
    /// Check if the user account is active
    pub fn is_active(&self) -> bool {
        self.status == USER_STATUS_ACTIVE
    }

    /// Check if the user account is locked
    pub fn is_locked(&self) -> bool {
        self.status == USER_STATUS_LOCKED
    }

    /// Check if the user account is registered (not yet activated)
    pub fn is_registered(&self) -> bool {
        self.status == USER_STATUS_REGISTERED
    }

    /// Get the full name of the user
    pub fn full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}
