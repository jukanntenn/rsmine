pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_users;
mod m20240101_000002_create_email_addresses;
mod m20240101_000003_create_tokens;
mod m20240101_000004_create_roles;
mod m20240101_000005_create_trackers;
mod m20240101_000006_create_issue_statuses;
mod m20240101_000007_create_enumerations;
mod m20240101_000008_create_projects;
mod m20240101_000009_create_members;
mod m20240101_000010_create_member_roles;
mod m20240101_000011_create_projects_trackers;
mod m20240101_000012_create_issue_categories;
mod m20240101_000013_create_issues;
mod m20240101_000014_create_issue_relations;
mod m20240101_000015_create_attachments;
mod m20240101_000016_create_journals;
mod m20240101_000017_create_journal_details;
mod m20240101_000100_seed_initial_data;
mod m20240101_000101_seed_admin_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_users::Migration),
            Box::new(m20240101_000002_create_email_addresses::Migration),
            Box::new(m20240101_000003_create_tokens::Migration),
            Box::new(m20240101_000004_create_roles::Migration),
            Box::new(m20240101_000005_create_trackers::Migration),
            Box::new(m20240101_000006_create_issue_statuses::Migration),
            Box::new(m20240101_000007_create_enumerations::Migration),
            Box::new(m20240101_000008_create_projects::Migration),
            Box::new(m20240101_000009_create_members::Migration),
            Box::new(m20240101_000010_create_member_roles::Migration),
            Box::new(m20240101_000011_create_projects_trackers::Migration),
            Box::new(m20240101_000012_create_issue_categories::Migration),
            Box::new(m20240101_000013_create_issues::Migration),
            Box::new(m20240101_000014_create_issue_relations::Migration),
            Box::new(m20240101_000015_create_attachments::Migration),
            Box::new(m20240101_000016_create_journals::Migration),
            Box::new(m20240101_000017_create_journal_details::Migration),
            Box::new(m20240101_000100_seed_initial_data::Migration),
            Box::new(m20240101_000101_seed_admin_user::Migration),
        ]
    }
}
