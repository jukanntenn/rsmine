import { get, post, put, del } from "./client";
import type {
  MemberDetail,
  MemberCreateRequest,
  MemberUpdateRequest,
  PaginationParams,
  PaginatedListResponse,
} from "@/types";

/**
 * Members (Memberships) API endpoints
 */
export const membersApi = {
  /**
   * Get list of members for a project
   */
  list: async (
    projectId: number,
    params?: PaginationParams
  ): Promise<PaginatedListResponse<MemberDetail, "memberships">> => {
    return get<PaginatedListResponse<MemberDetail, "memberships">>(
      `/api/v1/projects/${projectId}/memberships`,
      params
    );
  },

  /**
   * Get a single membership by ID
   */
  get: async (id: number): Promise<{ membership: MemberDetail }> => {
    return get<{ membership: MemberDetail }>(`/api/v1/memberships/${id}`);
  },

  /**
   * Add a member to a project
   */
  create: async (
    projectId: number,
    data: MemberCreateRequest
  ): Promise<{ membership: MemberDetail }> => {
    return post<{ membership: MemberDetail }>(
      `/api/v1/projects/${projectId}/memberships`,
      { membership: data }
    );
  },

  /**
   * Update a membership (roles)
   */
  update: async (id: number, data: MemberUpdateRequest): Promise<{ membership: MemberDetail }> => {
    return put<{ membership: MemberDetail }>(`/api/v1/memberships/${id}`, {
      membership: data,
    });
  },

  /**
   * Remove a member from a project
   */
  delete: async (id: number): Promise<void> => {
    return del<void>(`/api/v1/memberships/${id}`);
  },
};
