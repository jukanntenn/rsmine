use crate::application::errors::ApplicationError;
use crate::config::StorageConfig;
use crate::domain::entities::TempAttachment;
use crate::domain::repositories::TempAttachmentStore;
use crate::infrastructure::storage::{LocalFileStorage, StorageError};
use crate::presentation::middleware::CurrentUser;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Response for a successful file upload
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub upload: UploadTokenResponse,
}

/// The upload token in the response
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadTokenResponse {
    pub token: String,
}

/// Use case for uploading a file and getting a token
pub struct UploadFileUseCase {
    storage: Arc<LocalFileStorage>,
    temp_store: Arc<dyn TempAttachmentStore>,
    config: StorageConfig,
}

impl UploadFileUseCase {
    pub fn new(
        storage: Arc<LocalFileStorage>,
        temp_store: Arc<dyn TempAttachmentStore>,
        config: StorageConfig,
    ) -> Self {
        Self {
            storage,
            temp_store,
            config,
        }
    }

    /// Execute the file upload
    /// Returns an upload token that can be used when creating/updating issues
    pub async fn execute(
        &self,
        filename: String,
        content: Vec<u8>,
        content_type: Option<String>,
        current_user: &CurrentUser,
    ) -> Result<UploadResponse, ApplicationError> {
        // 1. Validate file size
        if content.len() as u64 > self.config.max_file_size {
            return Err(ApplicationError::Validation(format!(
                "File size exceeds maximum of {} bytes",
                self.config.max_file_size
            )));
        }

        // 2. Save file to storage
        let save_result = self
            .storage
            .save(&filename, &content)
            .map_err(|e| match e {
                StorageError::Io(io_err) => {
                    ApplicationError::Internal(format!("Failed to save file: {}", io_err))
                }
                StorageError::InvalidFilename(msg) => ApplicationError::Validation(msg),
                StorageError::NotFound(msg) => ApplicationError::Internal(msg),
            })?;

        // 3. Generate unique token
        let token = Uuid::new_v4().to_string().replace("-", "");

        // 4. Store temporary attachment info
        let temp_attachment = TempAttachment {
            token: token.clone(),
            filename,
            disk_directory: save_result.disk_directory,
            disk_filename: save_result.disk_filename,
            filesize: content.len() as i64,
            content_type,
            digest: Some(save_result.digest),
            author_id: current_user.id,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(24),
        };

        self.temp_store.store(temp_attachment).await.map_err(|e| {
            ApplicationError::Internal(format!("Failed to store upload token: {}", e))
        })?;

        Ok(UploadResponse {
            upload: UploadTokenResponse { token },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::persistence::repositories::InMemoryTempAttachmentStore;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_upload_file_success() {
        // Create temp directory for testing
        let temp_dir = std::env::temp_dir().join("rsmine_test_uploads");
        std::fs::create_dir_all(&temp_dir).ok();

        let storage = Arc::new(LocalFileStorage::new(temp_dir.clone()));
        let temp_store = Arc::new(InMemoryTempAttachmentStore::new());
        let config = StorageConfig {
            path: temp_dir.to_string_lossy().to_string(),
            max_file_size: 10485760,
        };

        let usecase = UploadFileUseCase::new(storage, temp_store, config);
        let current_user = CurrentUser {
            id: 1,
            login: "test".to_string(),
            admin: false,
        };

        let result = usecase
            .execute(
                "test.txt".to_string(),
                b"Hello, World!".to_vec(),
                Some("text/plain".to_string()),
                &current_user,
            )
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.upload.token.len(), 32); // UUID without hyphens

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[tokio::test]
    async fn test_upload_file_too_large() {
        let temp_dir = std::env::temp_dir().join("rsmine_test_uploads_large");
        std::fs::create_dir_all(&temp_dir).ok();

        let storage = Arc::new(LocalFileStorage::new(temp_dir.clone()));
        let temp_store = Arc::new(InMemoryTempAttachmentStore::new());
        let config = StorageConfig {
            path: temp_dir.to_string_lossy().to_string(),
            max_file_size: 10, // Very small limit
        };

        let usecase = UploadFileUseCase::new(storage, temp_store, config);
        let current_user = CurrentUser {
            id: 1,
            login: "test".to_string(),
            admin: false,
        };

        let result = usecase
            .execute(
                "test.txt".to_string(),
                b"This content is longer than 10 bytes".to_vec(),
                Some("text/plain".to_string()),
                &current_user,
            )
            .await;

        assert!(result.is_err());

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }
}
