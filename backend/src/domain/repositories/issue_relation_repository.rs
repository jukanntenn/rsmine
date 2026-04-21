use super::RepositoryError;
use crate::domain::entities::IssueRelation;
use async_trait::async_trait;

/// Data for creating a new issue relation
#[derive(Debug, Clone)]
pub struct NewIssueRelation {
    pub issue_from_id: i32,
    pub issue_to_id: i32,
    pub relation_type: String,
    pub delay: Option<i32>,
}

#[async_trait]
pub trait IssueRelationRepository: Send + Sync {
    /// Find all relations for an issue (both from and to)
    async fn find_by_issue(&self, issue_id: i32) -> Result<Vec<IssueRelation>, RepositoryError>;

    /// Delete all relations involving an issue (both from and to)
    async fn delete_by_issue(&self, issue_id: i32) -> Result<(), RepositoryError>;

    /// Create a new issue relation
    async fn create(&self, relation: NewIssueRelation) -> Result<IssueRelation, RepositoryError>;

    /// Check if a relation already exists between two issues
    async fn exists_relation(
        &self,
        issue_from_id: i32,
        issue_to_id: i32,
        relation_type: &str,
    ) -> Result<bool, RepositoryError>;

    /// Find a specific relation between two issues
    async fn find_relation(
        &self,
        issue_from_id: i32,
        issue_to_id: i32,
        relation_type: &str,
    ) -> Result<Option<IssueRelation>, RepositoryError>;

    /// Find a relation by its ID
    async fn find_by_id(&self, id: i32) -> Result<Option<IssueRelation>, RepositoryError>;

    /// Delete a relation by its ID
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;

    /// Delete a relation between two issues with a specific type
    async fn delete_by_issues_and_type(
        &self,
        issue_from_id: i32,
        issue_to_id: i32,
        relation_type: &str,
    ) -> Result<(), RepositoryError>;
}
