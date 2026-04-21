use crate::domain::entities::EmailAddress;
use crate::domain::repositories::{EmailAddressRepository, RepositoryError};
use crate::infrastructure::persistence::entities::email_addresses;
use crate::infrastructure::persistence::entities::prelude::EmailAddresses;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

/// SeaORM-based implementation of EmailAddressRepository
pub struct EmailAddressRepositoryImpl {
    db: DatabaseConnection,
}

impl EmailAddressRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert SeaORM model to domain entity
    fn model_to_entity(model: email_addresses::Model) -> EmailAddress {
        EmailAddress {
            id: model.id,
            user_id: model.user_id,
            address: model.address,
            is_default: model.is_default,
            notify: model.notify,
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
impl EmailAddressRepository for EmailAddressRepositoryImpl {
    async fn find_default_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<EmailAddress>, RepositoryError> {
        let email = EmailAddresses::find()
            .filter(email_addresses::Column::UserId.eq(user_id))
            .filter(email_addresses::Column::IsDefault.eq(true))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(email.map(Self::model_to_entity))
    }

    async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<EmailAddress>, RepositoryError> {
        let emails = EmailAddresses::find()
            .filter(email_addresses::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(emails.into_iter().map(Self::model_to_entity).collect())
    }

    async fn update_address(&self, email_id: i32, address: &str) -> Result<(), RepositoryError> {
        let email = EmailAddresses::find_by_id(email_id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("Email address with id {} not found", email_id))
            })?;

        let mut active_email: email_addresses::ActiveModel = email.into();
        active_email.address = Set(address.to_string());
        active_email.updated_on = Set(Some(Utc::now().naive_utc()));

        active_email
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn create(&self, email: EmailAddress) -> Result<EmailAddress, RepositoryError> {
        let active_email = email_addresses::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: Set(email.user_id),
            address: Set(email.address),
            is_default: Set(email.is_default),
            notify: Set(email.notify),
            created_on: Set(email.created_on.map(|dt| dt.naive_utc())),
            updated_on: Set(email.updated_on.map(|dt| dt.naive_utc())),
        };

        let inserted = active_email
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(inserted))
    }

    async fn exists_by_address(&self, address: &str) -> Result<bool, RepositoryError> {
        let count = EmailAddresses::find()
            .filter(email_addresses::Column::Address.eq(address))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn exists_by_address_excluding_user(
        &self,
        address: &str,
        exclude_user_id: i32,
    ) -> Result<bool, RepositoryError> {
        let count = EmailAddresses::find()
            .filter(email_addresses::Column::Address.eq(address))
            .filter(email_addresses::Column::UserId.ne(exclude_user_id))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn delete_by_user_id(&self, user_id: i32) -> Result<(), RepositoryError> {
        EmailAddresses::delete_many()
            .filter(email_addresses::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
