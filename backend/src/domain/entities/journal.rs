use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Journal entity representing change history and notes for issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
    pub id: i32,
    pub journalized_id: i32,
    pub journalized_type: String,
    pub user_id: i32,
    pub notes: Option<String>,
    pub created_on: DateTime<Utc>,
    pub private_notes: bool,
    pub updated_on: Option<DateTime<Utc>>,
    pub updated_by_id: Option<i32>,
}
