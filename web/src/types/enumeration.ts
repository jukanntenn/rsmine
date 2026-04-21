/**
 * Priority type (from enumerations)
 */
export interface Priority {
  id: number;
  name: string;
  position?: number;
  is_default: boolean;
  active: boolean;
}

/**
 * Tracker default status (nested in tracker response)
 */
export interface TrackerDefaultStatus {
  id: number;
  name: string;
}

/**
 * Tracker type
 */
export interface Tracker {
  id: number;
  name: string;
  position?: number;
  default_status?: TrackerDefaultStatus;
  description: string | null;
  enabled_standard_fields?: string[];
  is_in_chlog?: boolean;
  is_in_roadmap?: boolean;
  fields_bits?: number;
}

/**
 * Issue status type
 */
export interface Status {
  id: number;
  name: string;
  position?: number;
  is_closed: boolean;
  description: string | null;
}

/**
 * Issue category type
 */
export interface IssueCategory {
  id: number;
  name: string;
  project_id: number;
  assigned_to_id: number | null;
  assigned_to?: {
    id: number;
    name: string;
  } | null;
}

/**
 * Role type
 */
export interface Role {
  id: number;
  name: string;
  position: number;
  assignable: boolean;
  builtin: number;
  permissions: string[];
  issues_visibility: string;
  users_visibility: string;
  time_entries_visibility: string;
  all_roles_managed: boolean;
  settings: Record<string, unknown>;
}
