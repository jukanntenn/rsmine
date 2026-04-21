import { get, post, put, del } from "./client";
import type {
  Tracker,
  Status,
  Priority,
  Role,
} from "@/types";

/**
 * Trackers API endpoints
 */
export const trackersApi = {
  /**
   * Get list of all trackers
   */
  list: async (): Promise<{ trackers: Tracker[] }> => {
    return get<{ trackers: Tracker[] }>("/api/v1/trackers");
  },

  /**
   * Get a single tracker by ID
   */
  get: async (id: number): Promise<{ tracker: Tracker }> => {
    return get<{ tracker: Tracker }>(`/api/v1/trackers/${id}`);
  },

  /**
   * Create a new tracker
   */
  create: async (data: Partial<Tracker>): Promise<{ tracker: Tracker }> => {
    return post<{ tracker: Tracker }>("/api/v1/trackers", { tracker: data });
  },

  /**
   * Update an existing tracker
   */
  update: async (id: number, data: Partial<Tracker>): Promise<{ tracker: Tracker }> => {
    return put<{ tracker: Tracker }>(`/api/v1/trackers/${id}`, { tracker: data });
  },

  /**
   * Delete a tracker
   */
  delete: async (id: number): Promise<void> => {
    return del<void>(`/api/v1/trackers/${id}`);
  },
};

/**
 * Issue Statuses API endpoints
 */
export const statusesApi = {
  /**
   * Get list of all issue statuses
   */
  list: async (): Promise<{ issue_statuses: Status[] }> => {
    return get<{ issue_statuses: Status[] }>("/api/v1/issue_statuses");
  },

  /**
   * Get a single status by ID
   */
  get: async (id: number): Promise<{ issue_status: Status }> => {
    return get<{ issue_status: Status }>(`/api/v1/issue_statuses/${id}`);
  },

  /**
   * Create a new status
   */
  create: async (data: Partial<Status>): Promise<{ issue_status: Status }> => {
    return post<{ issue_status: Status }>("/api/v1/issue_statuses", { issue_status: data });
  },

  /**
   * Update an existing status
   */
  update: async (id: number, data: Partial<Status>): Promise<{ issue_status: Status }> => {
    return put<{ issue_status: Status }>(`/api/v1/issue_statuses/${id}`, { issue_status: data });
  },

  /**
   * Delete a status
   */
  delete: async (id: number): Promise<void> => {
    return del<void>(`/api/v1/issue_statuses/${id}`);
  },
};

/**
 * Issue Priorities API endpoints
 */
export const prioritiesApi = {
  /**
   * Get list of all issue priorities
   */
  list: async (): Promise<{ issue_priorities: Priority[] }> => {
    return get<{ issue_priorities: Priority[] }>("/api/v1/enumerations/issue_priorities");
  },
};

/**
 * Roles API endpoints
 */
export const rolesApi = {
  /**
   * Get list of all roles
   */
  list: async (): Promise<{ roles: Role[] }> => {
    return get<{ roles: Role[] }>("/api/v1/roles");
  },

  /**
   * Get a single role by ID
   */
  get: async (id: number): Promise<{ role: Role }> => {
    return get<{ role: Role }>(`/api/v1/roles/${id}`);
  },

  /**
   * Create a new role
   */
  create: async (data: Partial<Role>): Promise<{ role: Role }> => {
    return post<{ role: Role }>("/api/v1/roles", { role: data });
  },

  /**
   * Update an existing role
   */
  update: async (id: number, data: Partial<Role>): Promise<{ role: Role }> => {
    return put<{ role: Role }>(`/api/v1/roles/${id}`, { role: data });
  },

  /**
   * Delete a role
   */
  delete: async (id: number): Promise<void> => {
    return del<void>(`/api/v1/roles/${id}`);
  },
};
