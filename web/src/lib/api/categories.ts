import { get, post, put, del } from "./client";
import type { IssueCategory, PaginationParams, PaginatedListResponse } from "@/types";

/**
 * Issue Categories API endpoints
 */
export const categoriesApi = {
  /**
   * Get list of categories for a project
   */
  list: async (
    projectId: number,
    params?: PaginationParams
  ): Promise<PaginatedListResponse<IssueCategory, "issue_categories">> => {
    return get<PaginatedListResponse<IssueCategory, "issue_categories">>(
      `/api/v1/projects/${projectId}/issue_categories`,
      params
    );
  },

  /**
   * Get a single category by ID
   */
  get: async (id: number): Promise<{ issue_category: IssueCategory }> => {
    return get<{ issue_category: IssueCategory }>(`/api/v1/issue_categories/${id}`);
  },

  /**
   * Create a new category in a project
   */
  create: async (
    projectId: number,
    data: { name: string; assigned_to_id?: number }
  ): Promise<{ issue_category: IssueCategory }> => {
    return post<{ issue_category: IssueCategory }>(
      `/api/v1/projects/${projectId}/issue_categories`,
      { issue_category: data }
    );
  },

  /**
   * Update an existing category
   */
  update: async (
    id: number,
    data: { name?: string; assigned_to_id?: number | null }
  ): Promise<{ issue_category: IssueCategory }> => {
    return put<{ issue_category: IssueCategory }>(`/api/v1/issue_categories/${id}`, {
      issue_category: data,
    });
  },

  /**
   * Delete a category
   */
  delete: async (id: number, params?: { reassign_to?: number }): Promise<void> => {
    return del<void>(`/api/v1/issue_categories/${id}`, params);
  },
};
