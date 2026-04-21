use crate::domain::entities::Token;
use crate::domain::repositories::{CreateTokenDto, RepositoryError, TokenRepository};
use crate::infrastructure::persistence::entities::prelude::Tokens;
use crate::infrastructure::persistence::entities::tokens;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

/// SeaORM-based implementation of TokenRepository
pub struct TokenRepositoryImpl {
    db: DatabaseConnection,
}

impl TokenRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert SeaORM model to domain entity
    fn model_to_entity(model: tokens::Model) -> Token {
        Token {
            id: model.id,
            user_id: model.user_id,
            action: model.action,
            value: model.value,
            validity_expires_on: model
                .validity_expires_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            created_on: model
                .created_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            updated_on: model
                .updated_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
        }
    }
}

#[async_trait]
impl TokenRepository for TokenRepositoryImpl {
    async fn find_by_user_and_action(
        &self,
        user_id: i32,
        action: &str,
    ) -> Result<Option<Token>, RepositoryError> {
        let token = Tokens::find()
            .filter(tokens::Column::UserId.eq(user_id))
            .filter(tokens::Column::Action.eq(action))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(token.map(Self::model_to_entity))
    }

    async fn find_by_value(&self, value: &str) -> Result<Option<Token>, RepositoryError> {
        let token = Tokens::find()
            .filter(tokens::Column::Value.eq(value))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(token.map(Self::model_to_entity))
    }

    async fn delete_by_user_id(&self, user_id: i32) -> Result<(), RepositoryError> {
        Tokens::delete_many()
            .filter(tokens::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn create(&self, dto: CreateTokenDto) -> Result<Token, RepositoryError> {
        let now = Utc::now().naive_utc();
        let expires_on = dto.validity_expires_on.map(|dt| dt.naive_utc());

        let token = tokens::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: sea_orm::ActiveValue::Set(dto.user_id),
            action: sea_orm::ActiveValue::Set(dto.action),
            value: sea_orm::ActiveValue::Set(dto.value),
            validity_expires_on: sea_orm::ActiveValue::Set(expires_on),
            created_on: sea_orm::ActiveValue::Set(Some(now)),
            updated_on: sea_orm::ActiveValue::Set(Some(now)),
        };

        let inserted = token
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(inserted))
    }

    async fn delete_expired(&self) -> Result<u64, RepositoryError> {
        let now = Utc::now().naive_utc();

        let result = Tokens::delete_many()
            .filter(tokens::Column::ValidityExpiresOn.is_not_null())
            .filter(tokens::Column::ValidityExpiresOn.lt(now))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(result.rows_affected)
    }
}
