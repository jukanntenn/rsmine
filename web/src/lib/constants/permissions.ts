/**
 * Permission constants for the application
 */
export const PERMISSIONS = {
  // Project permissions
  VIEW_PROJECT: "view_project",
  EDIT_PROJECT: "edit_project",
  DELETE_PROJECT: "delete_project",
  CLOSE_PROJECT: "close_project",
  SELECT_PROJECT_MODULES: "select_project_modules",
  MANAGE_MEMBERS: "manage_members",
  
  // Issue permissions
  VIEW_ISSUES: "view_issues",
  ADD_ISSUES: "add_issues",
  EDIT_ISSUES: "edit_issues",
  DELETE_ISSUES: "delete_issues",
  VIEW_PRIVATE_ISSUES: "view_private_issues",
  SET_ISSUES_PRIVATE: "set_issues_private",
  SET_OWN_ISSUES_PRIVATE: "set_own_issues_private",
  MOVE_ISSUES: "move_issues",
  BULK_EDIT_ISSUES: "bulk_edit_issues",
  COPY_ISSUES: "copy_issues",
  MANAGE_ISSUE_RELATIONS: "manage_issue_relations",
  MANAGE_SUBTASKS: "manage_subtasks",
  
  // Time tracking permissions
  LOG_TIME: "log_time",
  VIEW_TIME_ENTRIES: "view_time_entries",
  EDIT_TIME_ENTRIES: "edit_time_entries",
  DELETE_TIME_ENTRIES: "delete_time_entries",
  
  // Attachment permissions
  VIEW_ATTACHMENTS: "view_attachments",
  ADD_ATTACHMENTS: "add_attachments",
  EDIT_ATTACHMENTS: "edit_attachments",
  DELETE_ATTACHMENTS: "delete_attachments",
  
  // Category permissions
  MANAGE_CATEGORIES: "manage_categories",
  
  // Wiki permissions
  VIEW_WIKI: "view_wiki",
  EDIT_WIKI: "edit_wiki",
  DELETE_WIKI_PAGES: "delete_wiki_pages",
  PROTECT_WIKI_PAGES: "protect_wiki_pages",
  MANAGE_WIKI: "manage_wiki",
  
  // Repository permissions
  VIEW_CHANGESETS: "view_changesets",
  COMMIT_ACCESS: "commit_access",
  MANAGE_REPOSITORY: "manage_repository",
  
  // News permissions
  VIEW_NEWS: "view_news",
  MANAGE_NEWS: "manage_news",
  COMMENT_NEWS: "comment_news",
  
  // Document permissions
  VIEW_DOCUMENTS: "view_documents",
  ADD_DOCUMENTS: "add_documents",
  EDIT_DOCUMENTS: "edit_documents",
  DELETE_DOCUMENTS: "delete_documents",
  
  // Message boards permissions
  VIEW_MESSAGES: "view_messages",
  ADD_MESSAGES: "add_messages",
  EDIT_MESSAGES: "edit_messages",
  DELETE_MESSAGES: "delete_messages",
  
  // Calendar and Gantt permissions
  VIEW_CALENDAR: "view_calendar",
  VIEW_GANTT: "view_gantt",
} as const;

export type Permission = typeof PERMISSIONS[keyof typeof PERMISSIONS];

/**
 * Global permissions (admin only)
 */
export const GLOBAL_PERMISSIONS = {
  ADD_PROJECT: "add_project",
  EDIT_PROJECT: "edit_project",
  DELETE_PROJECT: "delete_project",
  VIEW_PRIVATE_ISSUES: "view_private_issues",
  VIEW_ISSUES: "view_issues",
  MANAGE_MEMBERS: "manage_members",
} as const;

/**
 * Built-in roles
 */
export const BUILTIN_ROLES = {
  NON_MEMBER: 1,
  ANONYMOUS: 2,
} as const;
