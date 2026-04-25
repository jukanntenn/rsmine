use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Insert roles
        // Role builtin values: 0=custom, 1=non-member, 2=anonymous
        // issues_visibility: all, default, own
        db.execute_unprepared(
            "INSERT INTO roles (name, position, builtin, issues_visibility, assignable, users_visibility, time_entries_visibility, all_roles_managed) VALUES
            ('Manager', 1, 0, 'all', true, 'all', 'all', true),
            ('Developer', 2, 0, 'all', true, 'members_of_visible_projects', 'all', true),
            ('Reporter', 3, 0, 'default', true, 'members_of_visible_projects', 'all', true),
            ('Non-Member', 4, 1, 'default', true, 'members_of_visible_projects', 'all', true),
            ('Anonymous', 5, 2, 'default', false, '-', '-', false)"
        ).await?;

        // Insert trackers
        db.execute_unprepared(
            "INSERT INTO trackers (name, position, is_in_roadmap, default_status_id) VALUES
            ('Bug', 1, true, 1),
            ('Feature', 2, true, 1),
            ('Support', 3, false, 1)",
        )
        .await?;

        // Insert issue statuses
        db.execute_unprepared(
            "INSERT INTO issue_statuses (name, position, is_closed, is_default, default_done_ratio) VALUES
            ('New', 1, false, true, 0),
            ('In Progress', 2, false, false, NULL),
            ('Resolved', 3, false, false, 100),
            ('Feedback', 4, false, false, NULL),
            ('Closed', 5, true, false, 100),
            ('Rejected', 6, true, false, 100)"
        ).await?;

        // Insert priorities (enumerations)
        db.execute_unprepared(
            "INSERT INTO enumerations (name, position, is_default, type, active) VALUES
            ('Low', 1, false, 'IssuePriority', true),
            ('Normal', 2, true, 'IssuePriority', true),
            ('High', 3, false, 'IssuePriority', true),
            ('Urgent', 4, false, 'IssuePriority', true),
            ('Immediate', 5, false, 'IssuePriority', true)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DELETE FROM enumerations WHERE type = 'IssuePriority'")
            .await?;
        db.execute_unprepared("DELETE FROM issue_statuses").await?;
        db.execute_unprepared("DELETE FROM trackers").await?;
        db.execute_unprepared("DELETE FROM roles").await?;

        Ok(())
    }
}
