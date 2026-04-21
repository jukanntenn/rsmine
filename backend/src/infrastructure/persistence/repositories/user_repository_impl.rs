use crate::domain::entities::User;
use crate::domain::repositories::{RepositoryError, UserQueryParams, UserRepository};
use crate::infrastructure::persistence::entities::prelude::Users;
use crate::infrastructure::persistence::entities::users;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};

/// SeaORM-based implementation of UserRepository
pub struct UserRepositoryImpl {
    db: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert SeaORM model to domain entity
    fn model_to_entity(model: users::Model) -> User {
        User {
            id: model.id,
            login: model.login,
            hashed_password: model.hashed_password,
            firstname: model.firstname,
            lastname: model.lastname,
            admin: model.admin,
            status: model.status,
            last_login_on: model
                .last_login_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            language: model.language,
            auth_source_id: model.auth_source_id,
            created_on: model
                .created_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            updated_on: model
                .updated_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            r#type: model.r#type,
            mail_notification: model.mail_notification,
            salt: model.salt,
            must_change_passwd: model.must_change_passwd,
            passwd_changed_on: model
                .passwd_changed_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            twofa_scheme: model.twofa_scheme,
            twofa_totp_key: model.twofa_totp_key,
            twofa_totp_last_used_at: model.twofa_totp_last_used_at,
            twofa_required: model.twofa_required,
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_login(&self, login: &str) -> Result<Option<User>, RepositoryError> {
        // Case-insensitive login lookup (Redmine compatibility)
        let login_lower = login.to_lowercase();

        let user = Users::find()
            .filter(users::Column::Login.eq(login_lower))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        // If not found with lowercase, try exact match (fallback)
        let user = if user.is_none() {
            Users::find()
                .filter(users::Column::Login.eq(login))
                .one(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?
        } else {
            user
        };

        Ok(user.map(Self::model_to_entity))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, RepositoryError> {
        let user = Users::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(user.map(Self::model_to_entity))
    }

    async fn update_last_login(&self, user_id: i32) -> Result<(), RepositoryError> {
        let user = Users::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("User with id {} not found", user_id))
            })?;

        let mut active_user: users::ActiveModel = user.into();
        active_user.last_login_on = Set(Some(Utc::now().naive_utc()));

        active_user
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_all(&self, params: UserQueryParams) -> Result<Vec<User>, RepositoryError> {
        let mut query = Users::find();

        // Build filter condition
        let mut condition = Condition::all();

        // Filter by status
        if let Some(status) = params.status {
            condition = condition.add(users::Column::Status.eq(status));
        }

        // Filter by name (fuzzy search on firstname or lastname)
        if let Some(ref name) = params.name {
            let name_pattern = format!("%{}%", name);
            condition = condition.add(
                Condition::any()
                    .add(users::Column::Firstname.like(&name_pattern))
                    .add(users::Column::Lastname.like(&name_pattern))
                    .add(users::Column::Login.like(&name_pattern)),
            );
        }

        query = query.filter(condition);

        // Order by id
        query = query.order_by_asc(users::Column::Id);

        // Apply pagination
        query = query
            .offset(params.offset as u64)
            .limit(params.limit as u64);

        let users = query
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(users.into_iter().map(Self::model_to_entity).collect())
    }

    async fn count(&self, params: &UserQueryParams) -> Result<u32, RepositoryError> {
        let mut query = Users::find();

        // Build filter condition (same as find_all)
        let mut condition = Condition::all();

        // Filter by status
        if let Some(status) = params.status {
            condition = condition.add(users::Column::Status.eq(status));
        }

        // Filter by name (fuzzy search on firstname or lastname)
        if let Some(ref name) = params.name {
            let name_pattern = format!("%{}%", name);
            condition = condition.add(
                Condition::any()
                    .add(users::Column::Firstname.like(&name_pattern))
                    .add(users::Column::Lastname.like(&name_pattern))
                    .add(users::Column::Login.like(&name_pattern)),
            );
        }

        query = query.filter(condition);

        let count = query
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count as u32)
    }

    async fn update(&self, user: User) -> Result<User, RepositoryError> {
        let existing_user = Users::find_by_id(user.id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("User with id {} not found", user.id))
            })?;

        let mut active_user: users::ActiveModel = existing_user.into();
        active_user.login = Set(user.login);
        active_user.firstname = Set(user.firstname);
        active_user.lastname = Set(user.lastname);
        active_user.admin = Set(user.admin);
        active_user.status = Set(user.status);
        active_user.language = Set(user.language);
        active_user.hashed_password = Set(user.hashed_password);
        active_user.salt = Set(user.salt);
        active_user.must_change_passwd = Set(user.must_change_passwd);
        active_user.passwd_changed_on = Set(user.passwd_changed_on.map(|dt| dt.naive_utc()));
        active_user.updated_on = Set(user.updated_on.map(|dt| dt.naive_utc()));

        let updated = active_user
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(updated))
    }

    async fn create(&self, user: User) -> Result<User, RepositoryError> {
        let active_user = users::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            login: Set(user.login),
            hashed_password: Set(user.hashed_password),
            firstname: Set(user.firstname),
            lastname: Set(user.lastname),
            admin: Set(user.admin),
            status: Set(user.status),
            last_login_on: Set(user.last_login_on.map(|dt| dt.naive_utc())),
            language: Set(user.language),
            auth_source_id: Set(user.auth_source_id),
            created_on: Set(user.created_on.map(|dt| dt.naive_utc())),
            updated_on: Set(user.updated_on.map(|dt| dt.naive_utc())),
            r#type: Set(user.r#type),
            mail_notification: Set(user.mail_notification),
            salt: Set(user.salt),
            must_change_passwd: Set(user.must_change_passwd),
            passwd_changed_on: Set(user.passwd_changed_on.map(|dt| dt.naive_utc())),
            twofa_scheme: Set(user.twofa_scheme),
            twofa_totp_key: Set(user.twofa_totp_key),
            twofa_totp_last_used_at: Set(user.twofa_totp_last_used_at),
            twofa_required: Set(user.twofa_required),
        };

        let inserted = active_user
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(inserted))
    }

    async fn exists_by_login(&self, login: &str) -> Result<bool, RepositoryError> {
        let login_lower = login.to_lowercase();

        let count = Users::find()
            .filter(users::Column::Login.eq(login_lower))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn exists_by_login_excluding(
        &self,
        login: &str,
        exclude_user_id: i32,
    ) -> Result<bool, RepositoryError> {
        let login_lower = login.to_lowercase();

        let count = Users::find()
            .filter(users::Column::Login.eq(login_lower))
            .filter(users::Column::Id.ne(exclude_user_id))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn find_all_admins(&self) -> Result<Vec<User>, RepositoryError> {
        let admins = Users::find()
            .filter(users::Column::Admin.eq(true))
            .filter(users::Column::Status.eq(1)) // Only active admins
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(admins.into_iter().map(Self::model_to_entity).collect())
    }

    async fn delete(&self, user_id: i32) -> Result<(), RepositoryError> {
        let user = Users::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("User with id {} not found", user_id))
            })?;

        user.delete(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
