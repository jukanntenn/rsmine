# Redmine MVP Feature Test Cases

> Date: 2026-03-20
> Redmine Version: 6.1.2

---

This document organizes backend test cases for MVP features in Redmine, for reference in Rsmine development.

---

## 1. User Authentication

### 1.1 Functional Tests

**File Path:** `test/functional/account_controller_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_get_login` | GET login page renders successfully |
| `test_get_login_while_logged_in_should_redirect_to_back_url_if_present` | Logged-in user redirects to back_url |
| `test_get_login_while_logged_in_should_redirect_to_referer_without_back_url` | Redirect to referer when no back_url |
| `test_get_login_while_logged_in_should_redirect_to_home_by_default` | Default redirect to home |
| `test_login_should_redirect_to_back_url_param` | Login redirects to back_url parameter |
| `test_login_with_suburi_should_redirect_to_back_url_param` | Sub-URI support |
| `test_login_should_not_redirect_to_another_host` | **Security**: No redirect to external hosts |
| `test_login_with_wrong_password` | Wrong password shows error message |
| `test_login_with_locked_account_should_fail` | Locked accounts cannot login |
| `test_login_as_registered_user_with_manual_activation_should_inform_user` | Manual activation flow |
| `test_login_as_registered_user_with_email_activation_should_propose_new_activation_email` | Email activation flow |
| `test_login_should_rescue_auth_source_exception` | External auth source exception handling |
| `test_login_should_reset_session` | Session reset on login |
| `test_login_should_strip_whitespaces_from_user_name` | Username whitespace handling |
| `test_get_logout_should_not_logout` | GET request does not logout |
| `test_get_logout_with_anonymous_should_redirect` | Anonymous logout redirect |
| `test_logout` | POST logout destroys session |
| `test_logout_should_reset_session` | Session reset on logout |

### 1.2 Integration Tests

**File Path:** `test/integration/sessions_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_change_password_kills_sessions` | Password change invalidates all sessions |
| `test_lock_user_kills_sessions` | User lock kills sessions |
| `test_update_user_does_not_kill_sessions` | Non-password updates keep sessions |
| `test_change_password_generates_a_new_token_for_current_session` | New session token after password change |
| `test_simultaneous_sessions_should_be_valid` | Multiple concurrent sessions valid |

### 1.3 API Tests

**File Path:** `test/integration/api_test/authentication_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_api_should_deny_without_credentials` | Deny access without credentials |
| `test_api_should_accept_http_basic_auth_using_username_and_password` | HTTP Basic authentication |
| `test_api_should_deny_http_basic_auth_using_username_and_wrong_password` | Wrong password denied |
| `test_api_should_deny_http_basic_auth_if_twofa_is_active` | 2FA blocks Basic auth |
| `test_api_should_accept_http_basic_auth_using_api_key` | API Key authentication |
| `test_api_should_accept_auth_using_api_key_as_parameter` | API Key as URL parameter |
| `test_api_should_accept_auth_using_api_key_as_request_header` | API Key in request header |
| `test_api_request_should_not_use_user_session` | API uses separate auth from session |

### 1.4 Unit Tests

**File Path:** `test/unit/user_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_try_to_login_with_good_credentials_should_return_the_user` | Correct credentials return user |
| `test_try_to_login_with_wrong_credentials_should_return_nil` | Wrong credentials return nil |
| `test_try_to_login_with_locked_user_should_return_nil` | Locked user returns nil |
| `test_try_to_login_should_fall_back_to_case_insensitive` | Case-insensitive login |
| `test_try_to_login_should_select_the_exact_matching_user_first` | Exact match priority |
| `test_password_change_should_destroy_tokens` | Password change destroys tokens |
| `test_atom_key` | Atom feed key generation |
| `test_api_key_should_not_be_generated_twice` | API Key uniqueness |

---

## 2. User Management

### 2.1 Functional Tests

**File Path:** `test/functional/users_controller_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_index` | User list |
| `test_index_with_status_filter` | Filter users by status |
| `test_index_with_firstname_filter` | Filter by firstname |
| `test_index_with_group_filter` | Filter by group membership |
| `test_index_csv` | Export users to CSV |
| `test_show` | User details |
| `test_show_should_display_visible_custom_fields` | Custom fields visibility |
| `test_show_inactive` | Inactive user returns 404 |
| `test_show_displays_memberships_based_on_project_visibility` | User's project memberships |
| `test_show_current` | Current user profile |
| `test_show_issues_counts` | User's issue statistics |
| `test_new` | New user form |
| `test_create` | Create user with email notification |
| `test_create_with_preferences` | Create user with preferences |
| `test_create_with_generate_password_should_email_the_password` | Auto-generate password |
| `test_create_admin_should_send_security_notification` | Admin creation security notification |
| `test_edit` | Edit user form |
| `test_update` | Update user attributes |
| `test_update_with_group_ids_should_assign_groups` | Assign user to groups |
| `test_update_with_activation_should_send_a_notification` | User activation notification |
| `test_update_with_password_change_should_send_a_notification` | Password change notification |
| `test_update_assign_admin_should_send_security_notification` | Admin assignment notification |
| `test_destroy` | Delete user |

### 2.2 API Tests

**File Path:** `test/integration/api_test/users_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_GET_users.xml_should_return_users` | XML format user list |
| `test_GET_users.json_should_return_users` | JSON format user list |
| `test_GET_users.json_with_legacy_filter_params` | Filter by status/group/name |
| `test_GET_users_id.xml_should_return_the_user` | Get single user details |
| `test_GET_users_current.xml_should_return_current_user` | Get authenticated user |
| `test_GET_users_id.xml_with_include_memberships` | Include user memberships |
| `test_POST_users.xml_with_valid_parameters_should_create_the_user` | API create user |
| `test_POST_users.xml_with_generate_password` | API create user with auto password |
| `test_PUT_users_id.xml_with_valid_parameters_should_update_the_user` | API update user |
| `test_DELETE_users_id.xml_should_delete_the_user` | API delete user |

### 2.3 Unit Tests

**File Path:** `test/unit/user_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_create` | User creation validation |
| `test_update` | User update |
| `test_destroy_should_delete_members_and_roles` | Cascade delete members and roles |
| `test_destroy_should_update_attachments` | Reassign attachments to anonymous |
| `test_destroy_should_delete_tokens` | Delete user tokens |
| `test_mail_should_be_stripped` | Email whitespace handling |
| `test_should_create_email_address` | Create email address |
| `test_login_length_validation` | Login length validation |
| `test_generate_password_on_create` | Auto-generate password |
| `test_validate_password_length` | Password length validation |
| `test_validate_password_format` | Password format validation |
| `test_validate_password_complexity` | Password complexity validation |

---

## 3. Project Management

### 3.1 Functional Tests

**File Path:** `test/functional/projects_controller_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_index_by_anonymous_should_not_show_private_projects` | Hide private projects from anonymous |
| `test_index_atom` | Project Atom feed |
| `test_index_with_project_filter_is_my_projects` | Filter "My Projects" |
| `test_index_with_subproject_filter` | Filter by parent project |
| `test_index_as_list_should_format_column_value` | List view with columns |
| `test_index_as_list_should_indent_projects` | Nested project indentation |
| `test_index_csv` | Export projects to CSV |
| `test_new_by_admin_user_should_accept_get` | Admin new project form |
| `test_new_by_non_admin_user_with_add_project_permission` | Non-admin project creation |
| `test_create_by_admin_user_should_create_a_new_project` | Create project |
| `test_create_by_admin_user_should_create_a_new_subproject` | Create subproject |
| `test_create_by_non_admin_user_with_add_project_permission` | Non-admin creates project |
| `test_create_subproject_with_inherit_members_should_inherit_members` | Inherit members from parent |
| `test_show_by_id` | Show project by ID |
| `test_show_by_identifier` | Show project by identifier |
| `test_show_archived_project_should_be_denied` | Archived project access denied |
| `test_show_should_not_show_private_subprojects_that_are_not_visible` | Hide invisible subprojects |
| `test_settings` | Project settings page |
| `test_settings_of_subproject` | Subproject settings |

### 3.2 API Tests

**File Path:** `test/integration/api_test/projects_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_GET_projects.xml_should_return_projects` | Project list |
| `test_GET_projects.xml_with_include_issue_categories` | Include issue categories |
| `test_GET_projects.xml_with_include_trackers` | Include trackers |
| `test_GET_projects.xml_with_include_enabled_modules` | Include enabled modules |
| `test_GET_projects_id.xml_should_return_the_project` | Get single project |
| `test_GET_projects_id.xml_with_include_issue_categories` | Project with categories |
| `test_POST_projects.xml_with_valid_parameters_should_create_the_project` | API create project |
| `test_POST_projects.xml_should_accept_enabled_module_names_attribute` | Set modules on creation |
| `test_PUT_projects_id.xml_with_valid_parameters_should_update_the_project` | API update project |
| `test_DELETE_projects_id.xml_should_schedule_deletion_of_the_project` | API delete project (scheduled) |
| `test_PUT_projects_id_archive.xml_should_archive_project` | Archive project |
| `test_PUT_projects_id_unarchive.xml_should_unarchive_project` | Unarchive project |
| `test_PUT_projects_id_close.xml_should_close_project` | Close project |
| `test_PUT_projects_id_reopen.xml_should_reopen_project` | Reopen project |

### 3.3 Unit Tests

**File Path:** `test/unit/project_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_default_attributes` | Project defaults |
| `test_archive` | Archive project and subprojects |
| `test_unarchive` | Unarchive project |
| `test_destroy` | Delete project with all related data |
| `test_move_an_orphan_project_to_a_root_project` | Move project under parent |
| `test_set_parent_should_add_children_in_alphabetical_order` | Alphabetical subproject ordering |
| `test_parent` | Get parent project |
| `test_ancestors` | Get ancestor projects |
| `test_root` | Get root project |
| `test_children` | Get child projects |
| `test_descendants` | Get all descendant projects |
| `test_allowed_parents_with_add_subprojects_permission` | Allowed parent projects |
| `test_rolled_up_trackers` | Aggregate trackers from subprojects |
| `test_shared_versions` | Version sharing across hierarchy |

---

## 4. Member Management

### 4.1 Functional Tests

**File Path:** `test/functional/members_controller_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_index_with_csv_format_should_export_csv` | Export members to CSV |
| `test_new` | New member form |
| `test_new_should_propose_managed_roles_only` | Role visibility based on permissions |
| `test_create` | Add member to project |
| `test_create_multiple` | Add multiple members |
| `test_create_should_ignore_unmanaged_roles` | Cannot assign unmanaged roles |
| `test_edit` | Edit member form |
| `test_update` | Update member roles |
| `test_update_locked_member_should_be_allowed` | Update locked member's roles |
| `test_update_should_not_add_unmanaged_roles` | Cannot add unmanaged roles |
| `test_destroy` | Remove member from project |
| `test_destroy_locked_member_should_be_allowed` | Remove locked member |
| `test_autocomplete` | User autocomplete for adding members |

### 4.2 API Tests

**File Path:** `test/integration/api_test/memberships_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_GET_projects_project_id_memberships.xml_should_return_memberships` | Project member list |
| `test_GET_projects_project_id_memberships.xml_should_include_locked_users` | Include locked users |
| `test_POST_projects_project_id_memberships.xml_should_create_the_membership` | API add member |
| `test_POST_projects_project_id_memberships.xml_should_create_the_group_membership` | API add group as member |
| `test_GET_memberships_id.xml_should_return_the_membership` | Get single membership |
| `test_PUT_memberships_id.xml_should_update_the_membership` | API update member roles |
| `test_DELETE_memberships_id.xml_should_destroy_the_membership` | API remove member |

### 4.3 Unit Tests

**File Path:** `test/unit/member_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_create` | Create membership with roles |
| `test_update_roles` | Update member roles |
| `test_update_roles_with_inherited_roles` | Handle inherited roles from groups |
| `test_validate` | Validation: unique per project, at least one role |
| `test_destroy` | Delete membership |
| `test_managed_roles_should_return_all_roles_for_admins` | Admin role management |
| `test_create_principal_memberships_should_not_error_with_2_projects_and_inheritance` | Bulk membership creation |

---

## 5. Issue Management

### 5.1 Functional Tests

**File Path:** `test/functional/issues_controller_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_index` | List all visible issues |
| `test_index_with_project` | List issues in a project |
| `test_index_with_project_and_subprojects` | Include subproject issues |
| `test_index_with_project_and_default_filter` | Default filter (open issues) |
| `test_index_with_short_filters` | Query shortcuts (status, tracker, etc.) |
| `test_index_with_query` | Use saved query |
| `test_index_with_query_grouped_by_tracker` | Group by tracker |
| `test_show` | Display issue details |
| `test_show_should_display_visible_custom_fields` | Custom fields visibility |
| `test_new` | New issue form |
| `test_create` | Create issue |
| `test_create_with_required_custom_field` | Required custom field validation |
| `test_create_with_parent_issue_id` | Create subtask |
| `test_edit` | Edit issue form |
| `test_update` | Update issue |
| `test_update_with_failure` | Validation failure on update |
| `test_destroy` | Delete issue |

### 5.2 API Tests

**File Path:** `test/integration/api_test/issues_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_GET_issues.xml_should_contain_metadata` | Issues list with pagination metadata |
| `test_GET_issues.xml_with_filter` | Filter issues by status |
| `test_GET_issues.xml_with_custom_field_filter` | Filter by custom field |
| `test_GET_issues_id.xml_with_journals` | Issue with change history |
| `test_GET_issues_id.xml_with_custom_fields` | Issue with custom fields |
| `test_GET_issues_id.xml_with_attachments` | Issue with attachments |
| `test_GET_issues_id.xml_with_subtasks` | Issue with child issues |
| `test_GET_issues_id.xml_with_include_watchers` | Issue with watchers |
| `test_GET_issues_id.xml_with_include_allowed_statuses` | Allowed status transitions |
| `test_POST_issues.xml_with_valid_parameters` | API create issue |
| `test_POST_issues.xml_with_custom_fields` | Create with custom fields |
| `test_PUT_issues_id.xml_with_valid_parameters` | API update issue |
| `test_DELETE_issues_id.xml` | API delete issue |

### 5.3 Unit Tests

**File Path:** `test/unit/issue_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_create` | Issue creation |
| `test_create_minimal` | Minimum required fields |
| `test_create_with_required_custom_field` | Required custom field |
| `test_create_with_parent_issue_id` | Create subtask |
| `test_visible_scope_for_anonymous` | Visibility for anonymous |
| `test_visible_scope_for_member` | Visibility for project member |
| `test_visible_scope_for_admin` | Admin sees all issues |
| `test_create_with_group_assignment` | Assign to group |
| `test_copy` | Copy issue |
| `test_move_to_another_project` | Move issue between projects |
| `test_destroy` | Delete issue |

### 5.4 Subtask Tests

**File Path:** `test/unit/issue_subtasking_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_leaf_planning_fields_should_be_editable` | Leaf issue fields editable |
| `test_parent_dates_should_be_read_only_with_parent_issue_dates_set_to_derived` | Parent dates from children |
| `test_parent_priority_should_be_the_highest_open_child_priority` | Parent priority from children |
| `test_parent_done_ratio_should_be_average_done_ratio_of_leaves` | Parent done ratio |
| `test_parent_total_estimated_hours_should_be_sum_of_visible_descendants` | Rollup estimated hours |
| `test_open_issue_with_closed_parent_should_not_validate` | Cannot add open child to closed parent |

---

## 6. Issue Relations

### 6.1 Functional Tests

**File Path:** `test/functional/issue_relations_controller_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_create` | Create relation (relates) |
| `test_create_xhr` | Create relation via AJAX |
| `test_create_should_accept_id_with_hash` | Accept #ID format |
| `test_create_follows_relation_should_update_relations_list` | Create "follows" relation |
| `test_should_create_relations_with_visible_issues_only` | Only visible issues can be related |
| `test_create_duplicated_follows_relations_should_not_raise_exception` | Duplicate relation handling |
| `test_bulk_create_with_multiple_issue_to_id_issues` | Bulk create relations |
| `test_destroy` | Delete relation |
| `test_destroy_xhr` | Delete via AJAX |

### 6.2 API Tests

**File Path:** `test/integration/api_test/issue_relations_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_GET_issues_issue_id_relations.xml` | Issue relations list |
| `test_POST_issues_issue_id_relations.xml_should_create_the_relation` | API create relation |
| `test_GET_relations_id.xml_should_return_the_relation` | Get single relation |
| `test_DELETE_relations_id.xml_should_delete_the_relation` | API delete relation |

### 6.3 Unit Tests

**File Path:** `test/unit/issue_relation_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_create` | Create precedes relation |
| `test_create_minimum` | Create default "relates" relation |
| `test_follows_relation_should_be_reversed` | Follows becomes precedes |
| `test_cannot_create_inverse_relates_relations` | No duplicate relates |
| `test_validates_circular_dependency` | Prevent circular relations |
| `test_validates_circular_dependency_of_subtask` | Circular check includes subtasks |
| `test_subtasks_should_allow_precedes_relation` | Subtasks can have precedes |
| `test_to_s_should_return_the_relation_string` | Relation string format |

---

## 7. Attachment Management

### 7.1 Functional Tests

**File Path:** `test/functional/attachments_controller_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_show_text_file` | Display text file content |
| `test_show_image` | Display image preview |
| `test_show_pdf` | Display PDF preview |
| `test_show_file_from_private_issue_without_permission` | Access denied for private issue attachment |
| `test_show_file_from_private_issue_with_permission` | Access with permission |
| `test_download_text_file` | Download file |
| `test_download_should_be_denied_without_permission` | Permission check |
| `test_thumbnail` | Generate image thumbnail |
| `test_edit_all` | Bulk edit attachments |
| `test_update_all` | Bulk update attachment metadata |
| `test_download_all_with_valid_container` | Download all as ZIP |
| `test_destroy_issue_attachment` | Delete attachment from issue |
| `test_destroy_wiki_page_attachment` | Delete from wiki |
| `test_destroy_project_attachment` | Delete from project |

### 7.2 API Tests

**File Path:** `test/integration/api_test/attachments_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_GET_attachments_id.xml_should_return_the_attachment` | Get attachment metadata |
| `test_GET_attachments_download_id_filename` | Download attachment content |
| `test_GET_attachments_thumbnail_id` | Get thumbnail |
| `test_DELETE_attachments_id.xml` | API delete attachment |
| `test_PATCH_attachments_id.json` | API update attachment metadata |
| `test_POST_uploads.xml_should_return_the_token` | Upload file and get token |
| `test_POST_uploads.xml_should_accept_filename_param` | Upload with filename |
| `test_POST_uploads.xml_should_return_errors_if_file_is_too_big` | File size validation |

### 7.3 Unit Tests

**File Path:** `test/unit/attachment_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_create` | Create attachment |
| `test_destroy` | Delete attachment and file |
| `test_create_should_auto_assign_content_type` | Auto-detect content type |
| `test_attachments_with_same_content_should_reuse_same_file` | Deduplication (same content reuses file) |
| `test_filename_should_be_sanitized` | Filename sanitization |
| `test_thumbnailable_should_be_true_for_images` | Thumbnail support check |
| `test_update_attachments` | Bulk update |
| `test_archive_attachments` | Create ZIP archive |
| `test_prune_should_destroy_old_unattached_attachments` | Cleanup orphaned attachments |

---

## 8. Notes System

### 8.1 Functional Tests

**File Path:** `test/functional/journals_controller_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_index` | Journal changes Atom feed |
| `test_index_should_return_privates_notes_with_permission_only` | Private notes visibility |
| `test_index_should_show_visible_custom_fields_only` | Custom field visibility in feed |
| `test_diff_for_description_change` | Diff view for description changes |
| `test_diff_for_custom_field` | Diff for custom field changes |
| `test_reply_to_issue` | Reply to issue with note |
| `test_reply_to_note` | Reply to specific note |
| `test_reply_to_private_note_should_fail_without_permission` | Private note reply permission |
| `test_edit_xhr` | Edit note via AJAX |
| `test_update_xhr` | Update note via AJAX |
| `test_update_xhr_with_private_notes_checked` | Mark note as private |
| `test_update_xhr_with_empty_notes_should_delete_the_journal` | Delete empty note |

### 8.2 API Tests

**File Path:** `test/integration/api_test/journals_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_PUT_journals_id.xml_with_valid_parameters` | API update journal notes |
| `test_PUT_journals_id.xml_without_journal_details_should_destroy_journal` | Delete empty journal |

### 8.3 Unit Tests

**File Path:** `test/unit/journal_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_journalized_is_an_issue` | Journal belongs to issue |
| `test_new_status` | Track status changes |
| `test_create_should_send_email_notification` | Notification on journal creation |
| `test_should_not_save_journal_with_blank_notes_and_no_details` | Validation: notes or details required |
| `test_create_should_split_private_notes` | Private notes separation |
| `test_visible_scope_for_anonymous` | Visibility for anonymous users |
| `test_visible_scope_for_user` | Visibility for regular users |
| `test_visible_details_should_include_relations_to_visible_issues_only` | Filter hidden issues from details |
| `test_attachments` | Get attachments from journal details |

---

## 9. Permission Control

### 9.1 Functional Tests

**File Path:** `test/functional/roles_controller_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_index` | Role list |
| `test_index_should_show_warning_when_no_workflow_is_defined` | Workflow warning |
| `test_new` | New role form |
| `test_new_with_copy` | Copy role from existing |
| `test_create_without_workflow_copy` | Create role without workflow |
| `test_create_with_workflow_copy` | Copy workflow from another role |
| `test_create_with_managed_roles` | Create with managed role constraints |
| `test_edit` | Edit role form |
| `test_update` | Update role permissions |
| `test_update_trackers_permissions` | Per-tracker permissions |
| `test_destroy` | Delete role |
| `test_destroy_role_with_members` | Cannot delete role with members |
| `test_permissions` | Bulk edit permissions |
| `test_update_permissions` | Save bulk permission changes |

### 9.2 API Tests

**File Path:** `test/integration/api_test/roles_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_GET_roles.xml_should_return_the_roles` | Role list XML |
| `test_GET_roles.json_should_return_the_roles` | Role list JSON |
| `test_GET_roles_id.xml_should_return_the_role` | Role details with permissions |

### 9.3 Unit Tests

**File Path:** `test/unit/role_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_copy_from` | Copy role with permissions |
| `test_copy_workflows` | Copy workflow rules |
| `test_add_permission` | Add permissions to role |
| `test_remove_permission` | Remove permissions from role |
| `test_has_permission` | Check if role has permission |
| `test_permissions_all_trackers?` | Check tracker-wide permission |
| `test_permissions_tracker_ids?` | Check per-tracker permission |
| `test_allowed_to_with_symbol` | Permission check by name |
| `test_allowed_to_with_hash` | Permission check by controller/action |
| `test_anonymous_should_return_the_anonymous_role` | Get anonymous role |
| `test_non_member_should_return_the_non_member_role` | Get non-member role |

### 9.4 Workflow Tests

**File Path:** `test/unit/workflow_test.rb`

| Test Method | Description |
|-------------|-------------|
| `test_copy` | Copy workflow transitions |
| `test_workflow_permission_should_validate_rule` | Validate workflow permission rule |
| `test_workflow_permission_should_validate_field_name` | Validate field-based permissions |

---

## Summary

| Feature Module | Functional Tests | API Tests | Unit Tests | Total |
|----------------|------------------|-----------|------------|-------|
| User Authentication | 17 | 8 | 8 | 33 |
| User Management | 22 | 10 | 12 | 44 |
| Project Management | 18 | 13 | 14 | 45 |
| Member Management | 13 | 7 | 7 | 27 |
| Issue Management | 17 | 13 | 17 | 47 |
| Issue Relations | 9 | 4 | 8 | 21 |
| Attachment Management | 14 | 8 | 9 | 31 |
| Notes System | 12 | 2 | 9 | 23 |
| Permission Control | 14 | 3 | 15 | 32 |
| **Total** | **136** | **68** | **99** | **303** |

---

_End of Document_