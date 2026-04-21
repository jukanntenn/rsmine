import { get, post, put, del } from "./client";
import type {
  Project,
  ProjectCreateRequest,
  ProjectUpdateRequest,
  ProjectListItem,
  PaginationParams,
  PaginatedListResponse,
  Tracker,
} from "@/types";

/**
 * Projects API endpoints
 */
export const projectsApi = {
  /**
   * Get list of projects
   */
  list: async (params?: PaginationParams & { status?: number }): Promise<PaginatedListResponse<ProjectListItem, "projects">> => {
    return get<PaginatedListResponse<ProjectListItem, "projects">>("/api/v1/projects", params);
  },

  /**
   * Get a single project by ID
   */
  get: async (id: number, params?: { include?: string }): Promise<{ project: Project }> => {
    return get<{ project: Project }>(`/api/v1/projects/${id}`, params);
  },

  /**
   * Create a new project
   */
  create: async (data: ProjectCreateRequest): Promise<{ project: Project }> => {
    return post<{ project: Project }>("/api/v1/projects", { project: data });
  },

  /**
   * Update an existing project
   */
  update: async (id: number, data: ProjectUpdateRequest): Promise<{ project: Project }> => {
    return put<{ project: Project }>(`/api/v1/projects/${id}`, { project: data });
  },

  /**
   * Delete a project
   */
  delete: async (id: number): Promise<void> => {
    return del<void>(`/api/v1/projects/${id}`);
  },

  /**
   * Get trackers for a project
   */
  getTrackers: async (projectId: number): Promise<{ trackers: Tracker[] }> => {
    return get<{ trackers: Tracker[] }>(`/api/v1/projects/${projectId}/trackers`);
  },
};
