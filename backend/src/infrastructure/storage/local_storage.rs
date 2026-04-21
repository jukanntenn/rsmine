use chrono::Utc;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use thiserror::Error;

/// Error type for storage operations
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid filename: {0}")]
    InvalidFilename(String),

    #[error("File not found: {0}")]
    NotFound(String),
}

/// Result of saving a file to storage
#[derive(Debug, Clone)]
pub struct SaveResult {
    /// The directory path relative to base_path (e.g., "2026/03")
    pub disk_directory: String,
    /// The sanitized filename for disk storage
    pub disk_filename: String,
    /// SHA256 digest of the file content
    pub digest: String,
}

/// Local file storage implementation
pub struct LocalFileStorage {
    base_path: PathBuf,
}

impl LocalFileStorage {
    /// Create a new LocalFileStorage instance
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    /// Save file content and return disk_directory, disk_filename, and digest
    pub fn save(&self, filename: &str, content: &[u8]) -> Result<SaveResult, StorageError> {
        // 1. Create target directory based on current date
        let now = Utc::now();
        let disk_directory = format!("{}/{}", now.format("%Y"), now.format("%m"));
        let dir_path = self.base_path.join(&disk_directory);

        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
        }

        // 2. Calculate digest first (for deduplication)
        let digest = self.calculate_digest(content);

        // 3. Check for duplicate content (deduplication)
        if let Some(existing) = self.find_by_digest(&digest, &disk_directory)? {
            // Reuse existing file - return the existing disk_filename
            // The digest already matches, so we can reuse
            return Ok(SaveResult {
                disk_directory,
                disk_filename: existing,
                digest,
            });
        }

        // 4. Generate disk filename
        let timestamp = now.format("%y%m%d%H%M%S").to_string();
        let sanitized = self.sanitize_filename(filename);
        let disk_filename = format!("{}_{}", timestamp, sanitized);

        // 5. Write file
        let file_path = dir_path.join(&disk_filename);
        let mut file = File::create(&file_path)?;
        file.write_all(content)?;

        Ok(SaveResult {
            disk_directory,
            disk_filename,
            digest,
        })
    }

    /// Get the full path to a file
    pub fn get_full_path(&self, disk_directory: &str, disk_filename: &str) -> PathBuf {
        self.base_path.join(disk_directory).join(disk_filename)
    }

    /// Delete a file from storage
    pub fn delete(&self, disk_directory: &str, disk_filename: &str) -> Result<(), StorageError> {
        let file_path = self.get_full_path(disk_directory, disk_filename);
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }

    /// Load file content from storage
    pub fn load(&self, disk_directory: &str, disk_filename: &str) -> Result<Vec<u8>, StorageError> {
        let file_path = if disk_directory.is_empty() {
            self.base_path.join(disk_filename)
        } else {
            self.base_path.join(disk_directory).join(disk_filename)
        };

        if !file_path.exists() {
            return Err(StorageError::NotFound(format!(
                "File not found: {}",
                file_path.display()
            )));
        }

        let content = fs::read(&file_path)?;
        Ok(content)
    }

    /// Check if a file exists
    pub fn exists(&self, disk_directory: &str, disk_filename: &str) -> bool {
        let file_path = if disk_directory.is_empty() {
            self.base_path.join(disk_filename)
        } else {
            self.base_path.join(disk_directory).join(disk_filename)
        };
        file_path.exists()
    }

    /// Sanitize filename for disk storage
    fn sanitize_filename(&self, filename: &str) -> String {
        // Extract only the filename (remove path components)
        let name = std::path::Path::new(filename)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");

        // Replace invalid characters
        let invalid_chars = [
            '/', '?', '%', '*', ':', '|', '"', '\'', '<', '>', '\n', '\r',
        ];
        let sanitized: String = name
            .chars()
            .map(|c| if invalid_chars.contains(&c) { '_' } else { c })
            .collect();

        // Check if it's safe ASCII only
        let is_safe = sanitized
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.');

        if is_safe && sanitized.len() <= 50 {
            sanitized
        } else {
            // Hash the filename
            let mut hasher = Sha256::new();
            hasher.update(name.as_bytes());
            let hash = hasher.finalize();
            let hash_str = hex::encode(&hash[..8]);

            // Get extension
            let ext = std::path::Path::new(name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            if ext.is_empty() {
                hash_str
            } else {
                format!("{}.{}", hash_str, ext)
            }
        }
    }

    /// Calculate SHA256 digest of content
    fn calculate_digest(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        hex::encode(hasher.finalize())
    }

    /// Find an existing file by digest in a specific directory
    fn find_by_digest(
        &self,
        digest: &str,
        disk_directory: &str,
    ) -> Result<Option<String>, StorageError> {
        let dir_path = self.base_path.join(disk_directory);

        if !dir_path.exists() {
            return Ok(None);
        }

        // Read directory and check each file's digest
        for entry in fs::read_dir(&dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Ok(content) = fs::read(&path) {
                    let file_digest = self.calculate_digest(&content);
                    if file_digest == digest {
                        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                            return Ok(Some(filename.to_string()));
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_sanitize_filename_simple() {
        let storage = LocalFileStorage::new(PathBuf::from("/tmp"));
        assert_eq!(storage.sanitize_filename("document.pdf"), "document.pdf");
    }

    #[test]
    fn test_sanitize_filename_with_spaces() {
        let storage = LocalFileStorage::new(PathBuf::from("/tmp"));
        let result = storage.sanitize_filename("my file (1).txt");
        // Filename with spaces/parens gets hashed because spaces/parens are converted to underscores
        // and the result contains consecutive underscores which fail the ASCII alphanumeric check
        // or the original contains non-ASCII-safe characters
        assert!(result.ends_with(".txt"));
        assert!(!result.contains(" "));
    }

    #[test]
    fn test_sanitize_filename_with_invalid_chars() {
        let storage = LocalFileStorage::new(PathBuf::from("/tmp"));
        let result = storage.sanitize_filename("test/file:name?.txt");
        assert!(result.contains("_"));
    }

    #[test]
    fn test_calculate_digest() {
        let storage = LocalFileStorage::new(PathBuf::from("/tmp"));
        let content = b"hello world";
        let digest = storage.calculate_digest(content);
        // SHA256 of "hello world" is a known value
        assert_eq!(digest.len(), 64);
        assert!(digest.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_save_file_success() {
        let temp_dir = std::env::temp_dir().join("rsmine_test_save_file");
        std::fs::create_dir_all(&temp_dir).ok();

        let storage = LocalFileStorage::new(temp_dir.clone());
        let content = b"Test file content for save operation";

        let result = storage.save("test_document.txt", content);
        assert!(result.is_ok());

        let save_result = result.unwrap();
        assert!(!save_result.disk_directory.is_empty());
        assert!(!save_result.disk_filename.is_empty());
        assert!(save_result.disk_filename.ends_with("_test_document.txt"));
        assert_eq!(save_result.digest.len(), 64);

        // Verify file exists
        let file_path =
            storage.get_full_path(&save_result.disk_directory, &save_result.disk_filename);
        assert!(file_path.exists());

        // Verify content
        let saved_content = std::fs::read(&file_path).unwrap();
        assert_eq!(saved_content, content);

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_deduplication_same_content_reuses_file() {
        let temp_dir = std::env::temp_dir().join("rsmine_test_dedup");
        std::fs::create_dir_all(&temp_dir).ok();

        let storage = LocalFileStorage::new(temp_dir.clone());
        let content = b"Identical content for deduplication test";

        // Save first file
        let result1 = storage.save("first_file.txt", content).unwrap();

        // Wait a bit to ensure different timestamps
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Save second file with same content but different name
        let result2 = storage.save("second_file.txt", content).unwrap();

        // Both should have the same digest
        assert_eq!(result1.digest, result2.digest);

        // Both should point to the same physical file (deduplication)
        assert_eq!(result1.disk_filename, result2.disk_filename);
        assert_eq!(result1.disk_directory, result2.disk_directory);

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_different_content_creates_different_files() {
        let temp_dir = std::env::temp_dir().join("rsmine_test_diff_content");
        std::fs::create_dir_all(&temp_dir).ok();

        let storage = LocalFileStorage::new(temp_dir.clone());

        // Save two different files
        let result1 = storage.save("file1.txt", b"Content A").unwrap();
        let result2 = storage.save("file2.txt", b"Content B").unwrap();

        // Different digests
        assert_ne!(result1.digest, result2.digest);

        // Different filenames
        assert_ne!(result1.disk_filename, result2.disk_filename);

        // Both files should exist
        let path1 = storage.get_full_path(&result1.disk_directory, &result1.disk_filename);
        let path2 = storage.get_full_path(&result2.disk_directory, &result2.disk_filename);
        assert!(path1.exists());
        assert!(path2.exists());

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_sanitize_filename_non_ascii() {
        let storage = LocalFileStorage::new(PathBuf::from("/tmp"));

        // Chinese characters should be hashed
        let result = storage.sanitize_filename("中文文件.pdf");
        assert!(result.ends_with(".pdf"));
        assert!(!result.contains("中文"));

        // Emoji should be hashed
        let result = storage.sanitize_filename("file😀test.txt");
        assert!(result.ends_with(".txt"));
        assert!(!result.contains("😀"));
    }

    #[test]
    fn test_sanitize_filename_long_name() {
        let storage = LocalFileStorage::new(PathBuf::from("/tmp"));

        // Very long filename should be hashed
        let long_name = "a".repeat(100) + ".txt";
        let result = storage.sanitize_filename(&long_name);
        assert!(result.ends_with(".txt"));
        // Result should be shorter than original
        assert!(result.len() < long_name.len());
    }

    #[test]
    fn test_delete_file() {
        let temp_dir = std::env::temp_dir().join("rsmine_test_delete");
        std::fs::create_dir_all(&temp_dir).ok();

        let storage = LocalFileStorage::new(temp_dir.clone());

        // Save a file
        let result = storage.save("to_delete.txt", b"Delete me").unwrap();
        let file_path = storage.get_full_path(&result.disk_directory, &result.disk_filename);
        assert!(file_path.exists());

        // Delete the file
        let delete_result = storage.delete(&result.disk_directory, &result.disk_filename);
        assert!(delete_result.is_ok());
        assert!(!file_path.exists());

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_delete_nonexistent_file() {
        let temp_dir = std::env::temp_dir().join("rsmine_test_delete_nonexist");
        std::fs::create_dir_all(&temp_dir).ok();

        let storage = LocalFileStorage::new(temp_dir.clone());

        // Deleting a nonexistent file should succeed (idempotent)
        let result = storage.delete("2026/03", "nonexistent.txt");
        assert!(result.is_ok());

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_load_file_success() {
        let temp_dir = std::env::temp_dir().join("rsmine_test_load");
        std::fs::create_dir_all(&temp_dir).ok();

        let storage = LocalFileStorage::new(temp_dir.clone());
        let content = b"Test content for load operation";

        // Save a file
        let save_result = storage.save("test_load.txt", content).unwrap();

        // Load the file back
        let loaded_content = storage
            .load(&save_result.disk_directory, &save_result.disk_filename)
            .unwrap();
        assert_eq!(loaded_content, content);

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_load_file_not_found() {
        let temp_dir = std::env::temp_dir().join("rsmine_test_load_not_found");
        std::fs::create_dir_all(&temp_dir).ok();

        let storage = LocalFileStorage::new(temp_dir.clone());

        // Loading a nonexistent file should return NotFound error
        let result = storage.load("2026/03", "nonexistent.txt");
        assert!(result.is_err());

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_exists_file() {
        let temp_dir = std::env::temp_dir().join("rsmine_test_exists");
        std::fs::create_dir_all(&temp_dir).ok();

        let storage = LocalFileStorage::new(temp_dir.clone());

        // Save a file
        let save_result = storage.save("test_exists.txt", b"content").unwrap();

        // Check exists
        assert!(storage.exists(&save_result.disk_directory, &save_result.disk_filename));
        assert!(!storage.exists(&save_result.disk_directory, "nonexistent.txt"));

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }
}
