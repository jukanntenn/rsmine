use serde::{Deserialize, Serialize};

/// JournalDetail entity representing individual field changes in a journal entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalDetail {
    pub id: i32,
    pub journal_id: i32,
    pub property: String, // "attr", "cf", "attachment"
    pub prop_key: String, // Field name (e.g., "status_id", "subject")
    pub old_value: Option<String>,
    pub value: Option<String>,
}
