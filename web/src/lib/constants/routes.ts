/**
 * Route constants for the application
 */

// Auth routes
export const AUTH_ROUTES = {
  LOGIN: "/login",
  LOGOUT: "/logout",
  REGISTER: "/register",
} as const;

// Dashboard routes
export const DASHBOARD_ROUTES = {
  HOME: "/",
  PROJECTS: "/projects",
  ISSUES: "/issues",
  USERS: "/users",
  SETTINGS: "/settings",
} as const;

// Project routes
export const PROJECT_ROUTES = {
  LIST: "/projects",
  NEW: "/projects/new",
  DETAIL: (id: number | string) => `/projects/${id}`,
  EDIT: (id: number | string) => `/projects/${id}/edit`,
  ISSUES: (id: number | string) => `/projects/${id}/issues`,
  NEW_ISSUE: (id: number | string) => `/projects/${id}/issues/new`,
  MEMBERS: (id: number | string) => `/projects/${id}/members`,
  SETTINGS: (id: number | string) => `/projects/${id}/settings`,
  CATEGORIES: (id: number | string) => `/projects/${id}/categories`,
} as const;

// Issue routes
export const ISSUE_ROUTES = {
  LIST: "/issues",
  DETAIL: (id: number | string) => `/issues/${id}`,
  EDIT: (id: number | string) => `/issues/${id}/edit`,
} as const;

// User routes
export const USER_ROUTES = {
  LIST: "/users",
  NEW: "/users/new",
  DETAIL: (id: number | string) => `/users/${id}`,
  EDIT: (id: number | string) => `/users/${id}/edit`,
} as const;

// Settings routes
export const SETTINGS_ROUTES = {
  HOME: "/settings",
  ROLES: "/settings/roles",
  TRACKERS: "/settings/trackers",
  STATUSES: "/settings/statuses",
  PRIORITIES: "/settings/priorities",
} as const;

// API routes
export const API_ROUTES = {
  // Auth
  LOGIN: "/api/auth/login",
  LOGOUT: "/api/auth/logout",
  CURRENT_USER: "/api/auth/me",

  // Users
  USERS: "/api/users",
  USER: (id: number | string) => `/api/users/${id}`,

  // Projects
  PROJECTS: "/api/projects",
  PROJECT: (id: number | string) => `/api/projects/${id}`,
  PROJECT_MEMBERS: (id: number | string) => `/api/projects/${id}/members`,
  PROJECT_ISSUES: (id: number | string) => `/api/projects/${id}/issues`,
  PROJECT_TRACKERS: (id: number | string) => `/api/projects/${id}/trackers`,
  PROJECT_CATEGORIES: (id: number | string) => `/api/projects/${id}/categories`,

  // Issues
  ISSUES: "/api/issues",
  ISSUE: (id: number | string) => `/api/issues/${id}`,
  ISSUE_RELATIONS: (id: number | string) => `/api/issues/${id}/relations`,
  ISSUE_ATTACHMENTS: (id: number | string) => `/api/issues/${id}/attachments`,
  ISSUE_JOURNALS: (id: number | string) => `/api/issues/${id}/journals`,

  // Attachments
  ATTACHMENTS: "/api/attachments",
  ATTACHMENT: (id: number | string) => `/api/attachments/${id}`,
  ATTACHMENT_DOWNLOAD: (id: number | string) => `/api/attachments/${id}/download`,

  // Enumerations
  TRACKERS: "/api/enums/trackers",
  STATUSES: "/api/enums/statuses",
  PRIORITIES: "/api/enums/priorities",
  ROLES: "/api/enums/roles",

  // Health
  HEALTH: "/api/health",
} as const;
