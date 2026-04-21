use crate::domain::entities::TempAttachment;
use crate::domain::repositories::{RepositoryError, TempAttachmentStore};
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory implementation of TempAttachmentStore
/// Uses a simple HashMap with RwLock for thread safety
pub struct InMemoryTempAttachmentStore {
    store: Arc<RwLock<HashMap<String, TempAttachment>>>,
}

impl InMemoryTempAttachmentStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryTempAttachmentStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TempAttachmentStore for InMemoryTempAttachmentStore {
    async fn store(&self, temp: TempAttachment) -> Result<(), RepositoryError> {
        let mut store = self.store.write().await;
        store.insert(temp.token.clone(), temp);
        Ok(())
    }

    async fn take(&self, token: &str) -> Result<Option<TempAttachment>, RepositoryError> {
        let mut store = self.store.write().await;
        // Check if token exists and hasn't expired
        if let Some(temp) = store.get(token) {
            if temp.expires_at > Utc::now() {
                return Ok(store.remove(token));
            }
            // Token expired, remove it
            store.remove(token);
        }
        Ok(None)
    }

    async fn cleanup_expired(&self) -> Result<usize, RepositoryError> {
        let mut store = self.store.write().await;
        let now = Utc::now();
        let initial_len = store.len();

        store.retain(|_, temp| temp.expires_at > now);

        Ok(initial_len - store.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_and_take() {
        let store = InMemoryTempAttachmentStore::new();

        let temp = TempAttachment {
            token: "test-token".to_string(),
            filename: "test.txt".to_string(),
            disk_directory: "2026/03".to_string(),
            disk_filename: "260322123456_test.txt".to_string(),
            filesize: 100,
            content_type: Some("text/plain".to_string()),
            digest: Some("abc123".to_string()),
            author_id: 1,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
        };

        store.store(temp.clone()).await.unwrap();

        let retrieved = store.take("test-token").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().filename, "test.txt");

        // Second take should return None
        let second = store.take("test-token").await.unwrap();
        assert!(second.is_none());
    }

    #[tokio::test]
    async fn test_expired_token() {
        let store = InMemoryTempAttachmentStore::new();

        let temp = TempAttachment {
            token: "expired-token".to_string(),
            filename: "test.txt".to_string(),
            disk_directory: "2026/03".to_string(),
            disk_filename: "260322123456_test.txt".to_string(),
            filesize: 100,
            content_type: Some("text/plain".to_string()),
            digest: Some("abc123".to_string()),
            author_id: 1,
            created_at: Utc::now() - chrono::Duration::hours(25),
            expires_at: Utc::now() - chrono::Duration::hours(1), // Expired
        };

        store.store(temp).await.unwrap();

        let retrieved = store.take("expired-token").await.unwrap();
        assert!(retrieved.is_none());
    }
}
