pub mod attachment_repository;
pub mod email_address_repository;
pub mod enumeration_repository;
pub mod errors;
pub mod issue_category_repository;
pub mod issue_relation_repository;
pub mod issue_repository;
pub mod issue_status_repository;
pub mod journal_repository;
pub mod member_repository;
pub mod project_repository;
pub mod role_repository;
pub mod token_repository;
pub mod tracker_repository;
pub mod user_repository;

pub use attachment_repository::{AttachmentRepository, NewAttachment, TempAttachmentStore};
pub use email_address_repository::EmailAddressRepository;
pub use enumeration_repository::EnumerationRepository;
pub use errors::RepositoryError;
pub use issue_category_repository::{
    IssueCategoryRepository, IssueCategoryUpdate, NewIssueCategory,
};
pub use issue_relation_repository::{IssueRelationRepository, NewIssueRelation};
pub use issue_repository::{IssueRepository, IssueUpdate, NewIssue};
pub use issue_status_repository::{IssueStatusRepository, NewIssueStatus};
pub use journal_repository::{JournalRepository, NewJournal, NewJournalDetail};
pub use member_repository::{MemberRepository, NewMember};
pub use project_repository::{ProjectQueryParams, ProjectRepository};
pub use role_repository::{
    NewRole, RoleRepository, ROLE_BUILTIN_ANONYMOUS, ROLE_BUILTIN_CUSTOM, ROLE_BUILTIN_NON_MEMBER,
};
pub use token_repository::{CreateTokenDto, TokenRepository};
pub use tracker_repository::{NewTracker, TrackerRepository};
pub use user_repository::{UserQueryParams, UserRepository};

// Re-export value objects needed for queries
pub use crate::domain::value_objects::IssueQueryParams;
