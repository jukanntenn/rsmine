use crate::domain::entities::{Member, MemberWithRoles, Role, RoleWithInheritance, User};
use crate::domain::repositories::{MemberRepository, NewMember, RepositoryError};
use crate::infrastructure::persistence::entities::member_roles;
use crate::infrastructure::persistence::entities::members;
use crate::infrastructure::persistence::entities::prelude::{MemberRoles, Members, Roles, Users};
use crate::infrastructure::persistence::entities::roles;
use crate::infrastructure::persistence::entities::users;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter,
};

/// SeaORM-based implementation of MemberRepository
pub struct MemberRepositoryImpl {
    db: DatabaseConnection,
}

impl MemberRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MemberRepository for MemberRepositoryImpl {
    async fn find_by_project(
        &self,
        project_id: i32,
    ) -> Result<Vec<MemberWithRoles>, RepositoryError> {
        // Get all members for the project
        let member_models = Members::find()
            .filter(members::Column::ProjectId.eq(project_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let mut result = Vec::new();

        for member_model in member_models {
            // Get user for this member
            let user_model = Users::find()
                .filter(users::Column::Id.eq(member_model.user_id))
                .one(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;

            let user_model = match user_model {
                Some(u) => u,
                None => continue, // Skip if user doesn't exist
            };

            // Get member_roles for this member
            let member_role_models = MemberRoles::find()
                .filter(member_roles::Column::MemberId.eq(member_model.id))
                .all(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;

            // Get all roles for this member
            let role_ids: Vec<i32> = member_role_models.iter().map(|mr| mr.role_id).collect();

            let role_models = if role_ids.is_empty() {
                Vec::new()
            } else {
                Roles::find()
                    .filter(roles::Column::Id.is_in(role_ids))
                    .all(&self.db)
                    .await
                    .map_err(|e| RepositoryError::Database(e.to_string()))?
            };

            // Convert to domain entities
            let member = Member {
                id: member_model.id,
                user_id: member_model.user_id,
                project_id: member_model.project_id,
                created_on: member_model
                    .created_on
                    .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
                mail_notification: member_model.mail_notification,
            };

            let user = User {
                id: user_model.id,
                login: user_model.login,
                hashed_password: user_model.hashed_password,
                firstname: user_model.firstname,
                lastname: user_model.lastname,
                admin: user_model.admin,
                status: user_model.status,
                last_login_on: user_model
                    .last_login_on
                    .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
                language: user_model.language,
                auth_source_id: user_model.auth_source_id,
                created_on: user_model
                    .created_on
                    .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
                updated_on: user_model
                    .updated_on
                    .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
                r#type: user_model.r#type,
                mail_notification: user_model.mail_notification,
                salt: user_model.salt,
                must_change_passwd: user_model.must_change_passwd,
                passwd_changed_on: user_model
                    .passwd_changed_on
                    .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
                twofa_scheme: user_model.twofa_scheme,
                twofa_totp_key: user_model.twofa_totp_key,
                twofa_totp_last_used_at: user_model.twofa_totp_last_used_at,
                twofa_required: user_model.twofa_required,
            };

            let roles: Vec<RoleWithInheritance> = {
                // Create a map of role_id -> role model
                let role_map: std::collections::HashMap<i32, _> =
                    role_models.into_iter().map(|r| (r.id, r)).collect();

                // Create RoleWithInheritance for each member_role
                member_role_models
                    .into_iter()
                    .filter_map(|mr| {
                        role_map.get(&mr.role_id).map(|r| RoleWithInheritance {
                            role: Role {
                                id: r.id,
                                name: r.name.clone(),
                                position: r.position,
                                assignable: r.assignable,
                                builtin: r.builtin,
                                permissions: r.permissions.clone(),
                                issues_visibility: r.issues_visibility.clone(),
                                users_visibility: r.users_visibility.clone(),
                                time_entries_visibility: r.time_entries_visibility.clone(),
                                all_roles_managed: r.all_roles_managed,
                                settings: r.settings.clone(),
                                default_time_entry_activity_id: r.default_time_entry_activity_id,
                            },
                            inherited_from: mr.inherited_from,
                        })
                    })
                    .collect()
            };

            result.push(MemberWithRoles {
                member,
                user,
                roles,
            });
        }

        Ok(result)
    }

    async fn delete_by_user_id(&self, user_id: i32) -> Result<(), RepositoryError> {
        Members::delete_many()
            .filter(members::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn is_member(&self, project_id: i32, user_id: i32) -> Result<bool, RepositoryError> {
        let condition = Condition::all()
            .add(members::Column::ProjectId.eq(project_id))
            .add(members::Column::UserId.eq(user_id));

        let count = Members::find()
            .filter(condition)
            .count(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(count > 0)
    }

    async fn add_member(&self, member: NewMember) -> Result<i32, RepositoryError> {
        let new_member = members::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: sea_orm::ActiveValue::Set(member.user_id),
            project_id: sea_orm::ActiveValue::Set(member.project_id),
            created_on: sea_orm::ActiveValue::Set(Some(Utc::now().naive_utc())),
            mail_notification: sea_orm::ActiveValue::Set(member.mail_notification),
        };

        let inserted = new_member
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(inserted.id)
    }

    async fn add_member_role(
        &self,
        member_id: i32,
        role_id: i32,
        inherited_from: Option<i32>,
    ) -> Result<(), RepositoryError> {
        let new_role = member_roles::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            member_id: sea_orm::ActiveValue::Set(member_id),
            role_id: sea_orm::ActiveValue::Set(role_id),
            inherited_from: sea_orm::ActiveValue::Set(inherited_from),
        };

        new_role
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn add_manager(&self, project_id: i32, user_id: i32) -> Result<(), RepositoryError> {
        // Check if user is already a member
        let existing_member = Members::find()
            .filter(
                Condition::all()
                    .add(members::Column::ProjectId.eq(project_id))
                    .add(members::Column::UserId.eq(user_id)),
            )
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let member_id = if let Some(member) = existing_member {
            member.id
        } else {
            // Create new member
            self.add_member(NewMember {
                user_id,
                project_id,
                mail_notification: false,
            })
            .await?
        };

        // Add manager role (role_id = 3 is typically "Manager" in Redmine)
        // We need to check if the role already exists for this member
        let existing_role = MemberRoles::find()
            .filter(
                Condition::all()
                    .add(member_roles::Column::MemberId.eq(member_id))
                    .add(member_roles::Column::RoleId.eq(3)),
            )
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        if existing_role.is_none() {
            self.add_member_role(member_id, 3, None).await?;
        }

        Ok(())
    }

    async fn inherit_from_parent(
        &self,
        project_id: i32,
        parent_id: i32,
    ) -> Result<(), RepositoryError> {
        // Get all members from parent project
        let parent_members = Members::find()
            .filter(members::Column::ProjectId.eq(parent_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        for parent_member in parent_members {
            // Check if user is already a member of the new project
            let existing_member = Members::find()
                .filter(
                    Condition::all()
                        .add(members::Column::ProjectId.eq(project_id))
                        .add(members::Column::UserId.eq(parent_member.user_id)),
                )
                .one(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;

            let member_id = if let Some(member) = existing_member {
                member.id
            } else {
                // Create new member
                self.add_member(NewMember {
                    user_id: parent_member.user_id,
                    project_id,
                    mail_notification: parent_member.mail_notification,
                })
                .await?
            };

            // Get roles from parent member
            let parent_roles = MemberRoles::find()
                .filter(member_roles::Column::MemberId.eq(parent_member.id))
                .all(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?;

            for parent_role in parent_roles {
                // Check if role already exists
                let existing_role = MemberRoles::find()
                    .filter(
                        Condition::all()
                            .add(member_roles::Column::MemberId.eq(member_id))
                            .add(member_roles::Column::RoleId.eq(parent_role.role_id)),
                    )
                    .one(&self.db)
                    .await
                    .map_err(|e| RepositoryError::Database(e.to_string()))?;

                if existing_role.is_none() {
                    // Add role with inherited_from pointing to parent role
                    self.add_member_role(member_id, parent_role.role_id, Some(parent_role.id))
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn delete_by_project(&self, project_id: i32) -> Result<(), RepositoryError> {
        Members::delete_many()
            .filter(members::Column::ProjectId.eq(project_id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, member_id: i32) -> Result<Option<MemberWithRoles>, RepositoryError> {
        // Get the member
        let member_model = Members::find()
            .filter(members::Column::Id.eq(member_id))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let member_model = match member_model {
            Some(m) => m,
            None => return Ok(None),
        };

        // Get user for this member
        let user_model = Users::find()
            .filter(users::Column::Id.eq(member_model.user_id))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let user_model = match user_model {
            Some(u) => u,
            None => return Ok(None),
        };

        // Get ALL member_roles for this member (including inherited ones)
        let member_role_models = MemberRoles::find()
            .filter(member_roles::Column::MemberId.eq(member_model.id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        // Get all roles for this member
        let role_ids: Vec<i32> = member_role_models.iter().map(|mr| mr.role_id).collect();

        let role_models = if role_ids.is_empty() {
            Vec::new()
        } else {
            Roles::find()
                .filter(roles::Column::Id.is_in(role_ids))
                .all(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?
        };

        // Convert to domain entities
        let member = Member {
            id: member_model.id,
            user_id: member_model.user_id,
            project_id: member_model.project_id,
            created_on: member_model
                .created_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            mail_notification: member_model.mail_notification,
        };

        let user = User {
            id: user_model.id,
            login: user_model.login,
            hashed_password: user_model.hashed_password,
            firstname: user_model.firstname,
            lastname: user_model.lastname,
            admin: user_model.admin,
            status: user_model.status,
            last_login_on: user_model
                .last_login_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            language: user_model.language,
            auth_source_id: user_model.auth_source_id,
            created_on: user_model
                .created_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            updated_on: user_model
                .updated_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            r#type: user_model.r#type,
            mail_notification: user_model.mail_notification,
            salt: user_model.salt,
            must_change_passwd: user_model.must_change_passwd,
            passwd_changed_on: user_model
                .passwd_changed_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            twofa_scheme: user_model.twofa_scheme,
            twofa_totp_key: user_model.twofa_totp_key,
            twofa_totp_last_used_at: user_model.twofa_totp_last_used_at,
            twofa_required: user_model.twofa_required,
        };

        let roles: Vec<RoleWithInheritance> = {
            // Create a map of role_id -> role model
            let role_map: std::collections::HashMap<i32, _> =
                role_models.into_iter().map(|r| (r.id, r)).collect();

            // Create RoleWithInheritance for each member_role
            member_role_models
                .into_iter()
                .filter_map(|mr| {
                    role_map.get(&mr.role_id).map(|r| RoleWithInheritance {
                        role: Role {
                            id: r.id,
                            name: r.name.clone(),
                            position: r.position,
                            assignable: r.assignable,
                            builtin: r.builtin,
                            permissions: r.permissions.clone(),
                            issues_visibility: r.issues_visibility.clone(),
                            users_visibility: r.users_visibility.clone(),
                            time_entries_visibility: r.time_entries_visibility.clone(),
                            all_roles_managed: r.all_roles_managed,
                            settings: r.settings.clone(),
                            default_time_entry_activity_id: r.default_time_entry_activity_id,
                        },
                        inherited_from: mr.inherited_from,
                    })
                })
                .collect()
        };

        Ok(Some(MemberWithRoles {
            member,
            user,
            roles,
        }))
    }

    async fn find_by_project_and_user(
        &self,
        project_id: i32,
        user_id: i32,
    ) -> Result<Option<MemberWithRoles>, RepositoryError> {
        // Get the member
        let member_model = Members::find()
            .filter(members::Column::ProjectId.eq(project_id))
            .filter(members::Column::UserId.eq(user_id))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let member_model = match member_model {
            Some(m) => m,
            None => return Ok(None),
        };

        // Get user for this member
        let user_model = Users::find()
            .filter(users::Column::Id.eq(member_model.user_id))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let user_model = match user_model {
            Some(u) => u,
            None => return Ok(None),
        };

        // Get ALL member_roles for this member (including inherited ones)
        let member_role_models = MemberRoles::find()
            .filter(member_roles::Column::MemberId.eq(member_model.id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        // Get all roles for this member
        let role_ids: Vec<i32> = member_role_models.iter().map(|mr| mr.role_id).collect();

        let role_models = if role_ids.is_empty() {
            Vec::new()
        } else {
            Roles::find()
                .filter(roles::Column::Id.is_in(role_ids))
                .all(&self.db)
                .await
                .map_err(|e| RepositoryError::Database(e.to_string()))?
        };

        // Convert to domain entities
        let member = Member {
            id: member_model.id,
            user_id: member_model.user_id,
            project_id: member_model.project_id,
            created_on: member_model
                .created_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            mail_notification: member_model.mail_notification,
        };

        let user = User {
            id: user_model.id,
            login: user_model.login,
            hashed_password: user_model.hashed_password,
            firstname: user_model.firstname,
            lastname: user_model.lastname,
            admin: user_model.admin,
            status: user_model.status,
            last_login_on: user_model
                .last_login_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            language: user_model.language,
            auth_source_id: user_model.auth_source_id,
            created_on: user_model
                .created_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            updated_on: user_model
                .updated_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            r#type: user_model.r#type,
            mail_notification: user_model.mail_notification,
            salt: user_model.salt,
            must_change_passwd: user_model.must_change_passwd,
            passwd_changed_on: user_model
                .passwd_changed_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            twofa_scheme: user_model.twofa_scheme,
            twofa_totp_key: user_model.twofa_totp_key,
            twofa_totp_last_used_at: user_model.twofa_totp_last_used_at,
            twofa_required: user_model.twofa_required,
        };

        let roles: Vec<RoleWithInheritance> = {
            // Create a map of role_id -> role model
            let role_map: std::collections::HashMap<i32, _> =
                role_models.into_iter().map(|r| (r.id, r)).collect();

            // Create RoleWithInheritance for each member_role
            member_role_models
                .into_iter()
                .filter_map(|mr| {
                    role_map.get(&mr.role_id).map(|r| RoleWithInheritance {
                        role: Role {
                            id: r.id,
                            name: r.name.clone(),
                            position: r.position,
                            assignable: r.assignable,
                            builtin: r.builtin,
                            permissions: r.permissions.clone(),
                            issues_visibility: r.issues_visibility.clone(),
                            users_visibility: r.users_visibility.clone(),
                            time_entries_visibility: r.time_entries_visibility.clone(),
                            all_roles_managed: r.all_roles_managed,
                            settings: r.settings.clone(),
                            default_time_entry_activity_id: r.default_time_entry_activity_id,
                        },
                        inherited_from: mr.inherited_from,
                    })
                })
                .collect()
        };

        Ok(Some(MemberWithRoles {
            member,
            user,
            roles,
        }))
    }

    async fn clear_roles(&self, member_id: i32) -> Result<(), RepositoryError> {
        // Only clear non-inherited roles
        MemberRoles::delete_many()
            .filter(member_roles::Column::MemberId.eq(member_id))
            .filter(member_roles::Column::InheritedFrom.is_null())
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_id(&self, member_id: i32) -> Result<(), RepositoryError> {
        // First delete all member_roles
        MemberRoles::delete_many()
            .filter(member_roles::Column::MemberId.eq(member_id))
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        // Then delete the member
        Members::delete_by_id(member_id)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }
}
