use crate::domain::entities::Attachment;
use crate::domain::repositories::{AttachmentRepository, NewAttachment, RepositoryError};
use crate::infrastructure::persistence::entities::attachments;
use crate::infrastructure::persistence::entities::prelude::Attachments;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, Set,
};

/// SeaORM-based implementation of AttachmentRepository
pub struct AttachmentRepositoryImpl {
    db: DatabaseConnection,
}

impl AttachmentRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert SeaORM model to domain entity
    fn model_to_entity(model: attachments::Model) -> Attachment {
        Attachment {
            id: model.id,
            container_id: model.container_id,
            container_type: model.container_type,
            filename: model.filename,
            disk_filename: model.disk_filename,
            filesize: model.filesize,
            content_type: model.content_type,
            digest: model.digest,
            downloads: model.downloads,
            author_id: model.author_id,
            created_on: model
                .created_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            description: model.description,
            disk_directory: model.disk_directory,
        }
    }
}

#[async_trait]
impl AttachmentRepository for AttachmentRepositoryImpl {
    async fn find_by_container(
        &self,
        container_id: i32,
        container_type: &str,
    ) -> Result<Vec<Attachment>, RepositoryError> {
        let attachments = Attachments::find()
            .filter(
                Condition::all()
                    .add(attachments::Column::ContainerId.eq(container_id))
                    .add(attachments::Column::ContainerType.eq(container_type)),
            )
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(attachments.into_iter().map(Self::model_to_entity).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Attachment>, RepositoryError> {
        let attachment = Attachments::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(attachment.map(Self::model_to_entity))
    }

    async fn create(&self, new_attachment: NewAttachment) -> Result<Attachment, RepositoryError> {
        let now = Utc::now().naive_utc();

        let active_model = attachments::ActiveModel {
            id: NotSet,
            container_id: Set(new_attachment.container_id),
            container_type: Set(new_attachment.container_type),
            filename: Set(new_attachment.filename),
            disk_filename: Set(new_attachment.disk_filename),
            filesize: Set(new_attachment.filesize as i32),
            content_type: Set(new_attachment.content_type),
            digest: Set(new_attachment.digest),
            downloads: Set(0),
            author_id: Set(new_attachment.author_id),
            created_on: Set(Some(now)),
            description: Set(new_attachment.description),
            disk_directory: Set(new_attachment.disk_directory),
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(model))
    }

    async fn delete_by_container(
        &self,
        container_id: i32,
        container_type: &str,
    ) -> Result<(), RepositoryError> {
        Attachments::delete_many()
            .filter(
                Condition::all()
                    .add(attachments::Column::ContainerId.eq(container_id))
                    .add(attachments::Column::ContainerType.eq(container_type)),
            )
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        Attachments::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn increment_downloads(&self, id: i32) -> Result<(), RepositoryError> {
        // First get the current attachment
        let attachment = Attachments::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound(format!("Attachment {} not found", id)))?;

        // Update with incremented downloads
        let mut active_model: attachments::ActiveModel = attachment.into();
        active_model.downloads = Set(active_model.downloads.unwrap() + 1);

        active_model
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn has_other_with_digest(
        &self,
        digest: &str,
        exclude_id: i32,
    ) -> Result<bool, RepositoryError> {
        let count = Attachments::find()
            .filter(
                Condition::all()
                    .add(attachments::Column::Digest.eq(digest))
                    .add(attachments::Column::Id.ne(exclude_id)),
            )
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn update_description(
        &self,
        id: i32,
        description: Option<String>,
    ) -> Result<Attachment, RepositoryError> {
        // First get the current attachment
        let attachment = Attachments::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound(format!("Attachment {} not found", id)))?;

        // Update description
        let mut active_model: attachments::ActiveModel = attachment.into();
        active_model.description = Set(description);

        let model = active_model
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(model))
    }
}
