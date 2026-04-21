use super::RepositoryError;
use crate::domain::entities::Enumeration;
use async_trait::async_trait;

#[async_trait]
pub trait EnumerationRepository: Send + Sync {
    /// Find all enumerations of a specific type
    async fn find_by_type(&self, enum_type: &str) -> Result<Vec<Enumeration>, RepositoryError>;

    /// Find all active enumerations of a specific type
    async fn find_active_by_type(
        &self,
        enum_type: &str,
    ) -> Result<Vec<Enumeration>, RepositoryError>;

    /// Find the default enumeration of a specific type
    async fn find_default_by_type(
        &self,
        enum_type: &str,
    ) -> Result<Option<Enumeration>, RepositoryError>;

    /// Find an enumeration by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<Enumeration>, RepositoryError>;
}
