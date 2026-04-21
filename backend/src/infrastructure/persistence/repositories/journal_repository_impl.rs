use crate::domain::entities::{Journal, JournalDetail};
use crate::domain::repositories::{
    JournalRepository, NewJournal, NewJournalDetail, RepositoryError,
};
use crate::infrastructure::persistence::entities::journal_details;
use crate::infrastructure::persistence::entities::journals;
use crate::infrastructure::persistence::entities::prelude::{JournalDetails, Journals};
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

/// SeaORM-based implementation of JournalRepository
pub struct JournalRepositoryImpl {
    db: DatabaseConnection,
}

impl JournalRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Convert SeaORM model to domain entity
    fn model_to_entity(model: journals::Model) -> Journal {
        Journal {
            id: model.id,
            journalized_id: model.journalized_id,
            journalized_type: model.journalized_type,
            user_id: model.user_id,
            notes: model.notes,
            created_on: chrono::DateTime::from_naive_utc_and_offset(model.created_on, Utc),
            private_notes: model.private_notes,
            updated_on: model
                .updated_on
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, Utc)),
            updated_by_id: model.updated_by_id,
        }
    }
}

#[async_trait]
impl JournalRepository for JournalRepositoryImpl {
    async fn find_by_journalized(
        &self,
        journalized_id: i32,
        journalized_type: &str,
    ) -> Result<Vec<Journal>, RepositoryError> {
        let journals = Journals::find()
            .filter(
                Condition::all()
                    .add(journals::Column::JournalizedId.eq(journalized_id))
                    .add(journals::Column::JournalizedType.eq(journalized_type)),
            )
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(journals.into_iter().map(Self::model_to_entity).collect())
    }

    async fn delete_by_journalized(
        &self,
        journalized_id: i32,
        journalized_type: &str,
    ) -> Result<(), RepositoryError> {
        Journals::delete_many()
            .filter(
                Condition::all()
                    .add(journals::Column::JournalizedId.eq(journalized_id))
                    .add(journals::Column::JournalizedType.eq(journalized_type)),
            )
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_details(&self, journal_id: i32) -> Result<Vec<JournalDetail>, RepositoryError> {
        let details = JournalDetails::find()
            .filter(journal_details::Column::JournalId.eq(journal_id))
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(details
            .into_iter()
            .map(|d| JournalDetail {
                id: d.id,
                journal_id: d.journal_id,
                property: d.property,
                prop_key: d.prop_key,
                old_value: d.old_value,
                value: d.value,
            })
            .collect())
    }

    async fn create(&self, journal: NewJournal) -> Result<Journal, RepositoryError> {
        let now = chrono::Utc::now().naive_utc();

        let active_model = journals::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            journalized_id: Set(journal.journalized_id),
            journalized_type: Set(journal.journalized_type),
            user_id: Set(journal.user_id),
            notes: Set(journal.notes),
            created_on: Set(now),
            private_notes: Set(journal.private_notes),
            updated_on: Set(None),
            updated_by_id: Set(None),
        };

        let result = active_model
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(Self::model_to_entity(result))
    }

    async fn create_detail(
        &self,
        detail: NewJournalDetail,
    ) -> Result<JournalDetail, RepositoryError> {
        let active_model = journal_details::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            journal_id: Set(detail.journal_id),
            property: Set(detail.property),
            prop_key: Set(detail.prop_key),
            old_value: Set(detail.old_value),
            value: Set(detail.value),
        };

        let result = active_model
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(JournalDetail {
            id: result.id,
            journal_id: result.journal_id,
            property: result.property,
            prop_key: result.prop_key,
            old_value: result.old_value,
            value: result.value,
        })
    }
}
