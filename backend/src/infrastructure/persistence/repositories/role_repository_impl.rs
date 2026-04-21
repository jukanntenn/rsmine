use crate::domain::entities::Role;
use crate::domain::repositories::{NewRole, RepositoryError, RoleRepository};
use crate::infrastructure::persistence::entities::member_roles;
use crate::infrastructure::persistence::entities::members;
use crate::infrastructure::persistence::entities::prelude::{MemberRoles, Members, Roles};
use crate::infrastructure::persistence::entities::roles;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

/// SeaORM-based implementation of RoleRepository
pub struct RoleRepositoryImpl {
    db: DatabaseConnection,
}

impl RoleRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert database model to domain entity
    fn model_to_entity(model: roles::Model) -> Role {
        Role {
            id: model.id,
            name: model.name,
            position: model.position,
            assignable: model.assignable,
            builtin: model.builtin,
            permissions: model.permissions,
            issues_visibility: model.issues_visibility,
            users_visibility: model.users_visibility,
            time_entries_visibility: model.time_entries_visibility,
            all_roles_managed: model.all_roles_managed,
            settings: model.settings,
            default_time_entry_activity_id: model.default_time_entry_activity_id,
        }
    }
}

#[async_trait]
impl RoleRepository for RoleRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Role>, RepositoryError> {
        let role = Roles::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(role.map(Self::model_to_entity))
    }

    async fn find_all(&self) -> Result<Vec<Role>, RepositoryError> {
        let roles = Roles::find()
            .order_by_asc(roles::Column::Position)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(roles.into_iter().map(Self::model_to_entity).collect())
    }

    async fn find_custom(&self) -> Result<Vec<Role>, RepositoryError> {
        // Built-in roles have builtin > 0 (Non-Member = 1, Anonymous = 2)
        // Custom roles have builtin = 0
        let roles = Roles::find()
            .filter(roles::Column::Builtin.eq(0))
            .order_by_asc(roles::Column::Position)
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(roles.into_iter().map(Self::model_to_entity).collect())
    }

    async fn find_managed_by_user(&self, user_id: i32) -> Result<Vec<Role>, RepositoryError> {
        // Find the user's memberships with their roles
        // A user can manage all roles if they have a role with all_roles_managed = true
        // Otherwise, they can only assign roles that are marked as assignable

        // First, check if the user has any role with all_roles_managed = true
        let member_ids = Members::find()
            .filter(members::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let member_id_list: Vec<i32> = member_ids.iter().map(|m| m.id).collect();

        if member_id_list.is_empty() {
            // User has no memberships, return empty
            return Ok(Vec::new());
        }

        // Get roles for this user
        let user_role_ids = MemberRoles::find()
            .filter(member_roles::Column::MemberId.is_in(member_id_list.clone()))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let role_id_list: Vec<i32> = user_role_ids.iter().map(|mr| mr.role_id).collect();

        if role_id_list.is_empty() {
            return Ok(Vec::new());
        }

        // Check if any role has all_roles_managed = true
        let user_roles = Roles::find()
            .filter(roles::Column::Id.is_in(role_id_list.clone()))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let has_all_roles_managed = user_roles.iter().any(|r| r.all_roles_managed);

        if has_all_roles_managed {
            // User can manage all roles, return all
            return self.find_all().await;
        }

        // Otherwise, user can only assign assignable roles
        let manageable_roles = Roles::find()
            .filter(roles::Column::Assignable.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(manageable_roles
            .into_iter()
            .map(Self::model_to_entity)
            .collect())
    }

    async fn is_role_managed_by_user(
        &self,
        user_id: i32,
        role_id: i32,
    ) -> Result<bool, RepositoryError> {
        let manageable_roles = self.find_managed_by_user(user_id).await?;
        Ok(manageable_roles.iter().any(|r| r.id == role_id))
    }

    async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<Role>, RepositoryError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let roles = Roles::find()
            .filter(roles::Column::Id.is_in(ids.to_vec()))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(roles.into_iter().map(Self::model_to_entity).collect())
    }

    async fn create(&self, new_role: &NewRole) -> Result<Role, RepositoryError> {
        // Convert permissions to JSON string
        let permissions = new_role
            .permissions
            .as_ref()
            .map(|p| serde_json::to_string(p).unwrap_or_else(|_| "[]".to_string()));

        // Get the next position
        let max_position = Roles::find()
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let position = new_role.position.unwrap_or(1);

        let active_model = roles::ActiveModel {
            name: Set(new_role.name.clone()),
            position: Set(Some(position)),
            assignable: Set(new_role.assignable),
            builtin: Set(new_role.builtin),
            permissions: Set(permissions),
            issues_visibility: Set(new_role.issues_visibility.clone()),
            users_visibility: Set(new_role.users_visibility.clone()),
            time_entries_visibility: Set(new_role.time_entries_visibility.clone()),
            all_roles_managed: Set(new_role.all_roles_managed),
            settings: Set(None),
            default_time_entry_activity_id: Set(None),
            id: ActiveValue::NotSet,
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(model))
    }

    async fn update(&self, role: &Role) -> Result<Role, RepositoryError> {
        let existing = Roles::find_by_id(role.id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| {
                RepositoryError::NotFound(format!("Role with id {} not found", role.id))
            })?;

        // Convert permissions to JSON string if changed
        let permissions = role.permissions.clone();

        let mut active_model: roles::ActiveModel = existing.into();
        active_model.name = Set(role.name.clone());
        active_model.position = Set(role.position);
        active_model.assignable = Set(role.assignable);
        active_model.builtin = Set(role.builtin);
        active_model.permissions = Set(permissions);
        active_model.issues_visibility = Set(role.issues_visibility.clone());
        active_model.users_visibility = Set(role.users_visibility.clone());
        active_model.time_entries_visibility = Set(role.time_entries_visibility.clone());
        active_model.all_roles_managed = Set(role.all_roles_managed);
        active_model.settings = Set(role.settings.clone());
        active_model.default_time_entry_activity_id = Set(role.default_time_entry_activity_id);

        let model = active_model
            .update(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(model))
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        let role = Roles::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?
            .ok_or_else(|| RepositoryError::NotFound(format!("Role with id {} not found", id)))?;

        role.delete(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn exists_by_name(&self, name: &str) -> Result<bool, RepositoryError> {
        let count = Roles::find()
            .filter(roles::Column::Name.eq(name))
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
        let count = Roles::find()
            .filter(roles::Column::Name.eq(name))
            .filter(roles::Column::Id.ne(exclude_id))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn is_in_use(&self, id: i32) -> Result<bool, RepositoryError> {
        let count = MemberRoles::find()
            .filter(member_roles::Column::RoleId.eq(id))
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }
}
