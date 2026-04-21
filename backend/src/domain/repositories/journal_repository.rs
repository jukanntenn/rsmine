use super::RepositoryError;
use crate::domain::entities::{Journal, JournalDetail};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// Data for creating a new journal entry
#[derive(Debug, Clone)]
pub struct NewJournal {
    pub journalized_id: i32,
    pub journalized_type: String,
    pub user_id: i32,
    pub notes: Option<String>,
    pub private_notes: bool,
}

/// Data for creating a new journal detail
#[derive(Debug, Clone)]
pub struct NewJournalDetail {
    pub journal_id: i32,
    pub property: String,
    pub prop_key: String,
    pub old_value: Option<String>,
    pub value: Option<String>,
}

#[async_trait]
pub trait JournalRepository: Send + Sync {
    /// Find all journals for a journalized entity (e.g., issue)
    async fn find_by_journalized(
        &self,
        journalized_id: i32,
        journalized_type: &str,
    ) -> Result<Vec<Journal>, RepositoryError>;

    /// Find all journal details for a specific journal
    async fn find_details(&self, journal_id: i32) -> Result<Vec<JournalDetail>, RepositoryError>;

    /// Delete all journals for a journalized entity
    async fn delete_by_journalized(
        &self,
        journalized_id: i32,
        journalized_type: &str,
    ) -> Result<(), RepositoryError>;

    /// Create a new journal entry
    async fn create(&self, journal: NewJournal) -> Result<Journal, RepositoryError>;

    /// Create a new journal detail
    async fn create_detail(
        &self,
        detail: NewJournalDetail,
    ) -> Result<JournalDetail, RepositoryError>;
}
