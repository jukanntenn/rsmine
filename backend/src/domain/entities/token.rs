use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Token domain entity (pure Rust struct, independent of any ORM)
/// Used for API keys, session tokens, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub action: String,
    pub value: String,
    pub validity_expires_on: Option<DateTime<Utc>>,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
}

/// Token action types (as used in Redmine)
pub const TOKEN_ACTION_API: &str = "api";
pub const TOKEN_ACTION_SESSION: &str = "session";
pub const TOKEN_ACTION_FEEDS: &str = "feeds";
pub const TOKEN_ACTION_RECOVERY: &str = "recovery";
pub const TOKEN_ACTION_AUTLOGIN: &str = "autologin";

/// Token action for blacklisted JWT tokens (logout)
pub const TOKEN_ACTION_BLACKLIST: &str = "blacklist";

impl Token {
    /// Check if this is an API token
    pub fn is_api_token(&self) -> bool {
        self.action == TOKEN_ACTION_API
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_on) = self.validity_expires_on {
            expires_on < Utc::now()
        } else {
            false // No expiration date means it doesn't expire
        }
    }
}
