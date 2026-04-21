import { get, post, put, del } from "./client";
import type {
  User,
  UserCreateRequest,
  UserUpdateRequest,
  PaginationParams,
  PaginatedListResponse,
} from "@/types";

/**
 * Users API endpoints
 */
export const usersApi = {
  /**
   * Get list of users
   */
  list: async (params?: PaginationParams & { status?: number }): Promise<PaginatedListResponse<User, "users">> => {
    return get<PaginatedListResponse<User, "users">>("/api/v1/users", params);
  },

  /**
   * Get a single user by ID
   */
  get: async (id: number): Promise<{ user: User }> => {
    return get<{ user: User }>(`/api/v1/users/${id}`);
  },

  /**
   * Create a new user
   */
  create: async (data: UserCreateRequest): Promise<{ user: User }> => {
    return post<{ user: User }>("/api/v1/users", { user: data });
  },

  /**
   * Update an existing user
   */
  update: async (id: number, data: UserUpdateRequest): Promise<{ user: User }> => {
    return put<{ user: User }>(`/api/v1/users/${id}`, { user: data });
  },

  /**
   * Delete a user
   */
  delete: async (id: number): Promise<void> => {
    return del<void>(`/api/v1/users/${id}`);
  },
};
