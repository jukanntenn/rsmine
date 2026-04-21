use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Issue entity representing a task or bug in a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: i32,
    pub tracker_id: i32,
    pub project_id: i32,
    pub subject: String,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDate>,
    pub category_id: Option<i32>,
    pub status_id: i32,
    pub assigned_to_id: Option<i32>,
    pub priority_id: i32,
    pub fixed_version_id: Option<i32>,
    pub author_id: i32,
    pub lock_version: i32,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
    pub start_date: Option<chrono::NaiveDate>,
    pub done_ratio: i32,
    pub estimated_hours: Option<f64>,
    pub parent_id: Option<i32>,
    pub root_id: Option<i32>,
    pub lft: Option<i32>,
    pub rgt: Option<i32>,
    pub is_private: bool,
    pub closed_on: Option<DateTime<Utc>>,
}
