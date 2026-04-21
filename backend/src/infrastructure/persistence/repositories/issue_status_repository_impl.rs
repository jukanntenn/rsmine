use crate::domain::entities::IssueStatus;
use crate::domain::repositories::{IssueStatusRepository, NewIssueStatus, RepositoryError};
use crate::infrastructure::persistence::entities::issue_statuses;
use crate::infrastructure::persistence::entities::issues;
use crate::infrastructure::persistence::entities::prelude::IssueStatuses;
use crate::infrastructure::persistence::entities::prelude::Issues;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

/// SeaORM-based implementation of IssueStatusRepository
pub struct IssueStatusRepositoryImpl {
    db: DatabaseConnection,
}

impl IssueStatusRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert database model to domain entity
    fn model_to_entity(model: issue_statuses::Model) -> IssueStatus {
        IssueStatus {
            id: model.id,
            name: model.name,
            position: model.position,
            is_closed: model.is_closed,
            is_default: model.is_default,
            default_done_ratio: model.default_done_ratio,
        }
    }
}

#[async_trait]
impl IssueStatusRepository for IssueStatusRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<IssueStatus>, RepositoryError> {
        let statuses = IssueStatuses::find()
            .order_by_asc(issue_statuses::Column::Position)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(statuses.into_iter().map(Self::model_to_entity).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<IssueStatus>, RepositoryError> {
        let status = IssueStatuses::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(status.map(Self::model_to_entity))
    }

    async fn find_default(&self) -> Result<Option<IssueStatus>, RepositoryError> {
        let status = IssueStatuses::find()
            .filter(issue_statuses::Column::IsDefault.eq(true))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(status.map(Self::model_to_entity))
    }

    async fn find_open(&self) -> Result<Vec<IssueStatus>, RepositoryError> {
        let statuses = IssueStatuses::find()
            .filter(issue_statuses::Column::IsClosed.eq(false))
            .order_by_asc(issue_statuses::Column::Position)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(statuses.into_iter().map(Self::model_to_entity).collect())
    }

    async fn find_closed(&self) -> Result<Vec<IssueStatus>, RepositoryError> {
        let statuses = IssueStatuses::find()
            .filter(issue_statuses::Column::IsClosed.eq(true))
            .order_by_asc(issue_statuses::Column::Position)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(statuses.into_iter().map(Self::model_to_entity).collect())
    }

    async fn create(&self, status: &NewIssueStatus) -> Result<IssueStatus, RepositoryError> {
        // Get max position for ordering
        let max_position = IssueStatuses::find()
            .order_by_desc(issue_statuses::Column::Position)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .and_then(|s| s.position)
            .unwrap_or(0);

        let active_model = issue_statuses::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            name: Set(status.name.clone()),
            position: Set(status.position.or(Some(max_position + 1))),
            is_closed: Set(status.is_closed),
            is_default: Set(status.is_default),
            default_done_ratio: Set(status.default_done_ratio),
        };

        let result = active_model
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(result))
    }

    async fn update(&self, status: &IssueStatus) -> Result<IssueStatus, RepositoryError> {
        let existing = IssueStatuses::find_by_id(status.id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("Issue status with id {} not found", status.id))
            })?;

        let mut active_model: issue_statuses::ActiveModel = existing.into();
        active_model.name = Set(status.name.clone());
        active_model.position = Set(status.position);
        active_model.is_closed = Set(status.is_closed);
        active_model.is_default = Set(status.is_default);
        active_model.default_done_ratio = Set(status.default_done_ratio);

        let result = active_model
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(result))
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        let status = IssueStatuses::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("Issue status with id {} not found", id))
            })?;

        let active_model: issue_statuses::ActiveModel = status.into();
        active_model
            .delete(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn exists_by_name(&self, name: &str) -> Result<bool, RepositoryError> {
        let count = IssueStatuses::find()
            .filter(issue_statuses::Column::Name.eq(name))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn exists_by_name_excluding(
        &self,
        name: &str,
        exclude_id: i32,
    ) -> Result<bool, RepositoryError> {
        let count = IssueStatuses::find()
            .filter(issue_statuses::Column::Name.eq(name))
            .filter(issue_statuses::Column::Id.ne(exclude_id))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn clear_default(&self) -> Result<(), RepositoryError> {
        // Find all statuses with is_default = true and set them to false
        let default_statuses = IssueStatuses::find()
            .filter(issue_statuses::Column::IsDefault.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        for status in default_statuses {
            let mut active_model: issue_statuses::ActiveModel = status.into();
            active_model.is_default = Set(false);
            active_model
                .update(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;
        }

        Ok(())
    }

    async fn count_issues_by_status(&self, status_id: i32) -> Result<u64, RepositoryError> {
        let count = Issues::find()
            .filter(issues::Column::StatusId.eq(status_id))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count)
    }

    async fn reassign_issues_status(
        &self,
        from_status_id: i32,
        to_status_id: i32,
    ) -> Result<u64, RepositoryError> {
        // Find all issues with the old status
        let issues_to_update = Issues::find()
            .filter(issues::Column::StatusId.eq(from_status_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let count = issues_to_update.len() as u64;

        // Update each issue to the new status
        for issue in issues_to_update {
            let mut active_issue: issues::ActiveModel = issue.into();
            active_issue.status_id = Set(to_status_id);
            active_issue.updated_on = Set(Some(chrono::Utc::now().naive_utc()));
            active_issue
                .update(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;
        }

        Ok(count)
    }
}
