use crate::domain::entities::Enumeration;
use crate::domain::repositories::{EnumerationRepository, RepositoryError};
use crate::infrastructure::persistence::entities::enumerations;
use crate::infrastructure::persistence::entities::prelude::Enumerations;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

/// SeaORM-based implementation of EnumerationRepository
pub struct EnumerationRepositoryImpl {
    db: DatabaseConnection,
}

impl EnumerationRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert database model to domain entity
    fn model_to_entity(model: enumerations::Model) -> Enumeration {
        Enumeration {
            id: model.id,
            name: model.name,
            position: model.position,
            is_default: model.is_default,
            enum_type: model.r#type,
            active: model.active,
            project_id: model.project_id,
            parent_id: model.parent_id,
            position_name: model.position_name,
        }
    }
}

#[async_trait]
impl EnumerationRepository for EnumerationRepositoryImpl {
    async fn find_by_type(&self, enum_type: &str) -> Result<Vec<Enumeration>, RepositoryError> {
        let enumerations = Enumerations::find()
            .filter(enumerations::Column::Type.eq(enum_type))
            .order_by_asc(enumerations::Column::Position)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(enumerations
            .into_iter()
            .map(Self::model_to_entity)
            .collect())
    }

    async fn find_active_by_type(
        &self,
        enum_type: &str,
    ) -> Result<Vec<Enumeration>, RepositoryError> {
        let enumerations = Enumerations::find()
            .filter(enumerations::Column::Type.eq(enum_type))
            .filter(enumerations::Column::Active.eq(true))
            .order_by_asc(enumerations::Column::Position)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(enumerations
            .into_iter()
            .map(Self::model_to_entity)
            .collect())
    }

    async fn find_default_by_type(
        &self,
        enum_type: &str,
    ) -> Result<Option<Enumeration>, RepositoryError> {
        let enumeration = Enumerations::find()
            .filter(enumerations::Column::Type.eq(enum_type))
            .filter(enumerations::Column::IsDefault.eq(true))
            .filter(enumerations::Column::Active.eq(true))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(enumeration.map(Self::model_to_entity))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Enumeration>, RepositoryError> {
        let enumeration = Enumerations::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(enumeration.map(Self::model_to_entity))
    }
}
