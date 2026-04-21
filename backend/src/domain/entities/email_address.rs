use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// EmailAddress domain entity (pure Rust struct, independent of any ORM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress {
    pub id: i32,
    pub user_id: i32,
    pub address: String,
    pub is_default: bool,
    pub notify: bool,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
}

impl EmailAddress {
    /// Check if this is the default email address for the user
    pub fn is_primary(&self) -> bool {
        self.is_default
    }
}
