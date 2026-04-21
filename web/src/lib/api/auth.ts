import { get, post } from "./client";
import type { AuthUser, LoginRequest, LoginResponse } from "@/types";

/**
 * Auth API endpoints
 */
export const authApi = {
  /**
   * Login with username and password
   */
  login: async (credentials: LoginRequest): Promise<LoginResponse> => {
    return post<LoginResponse>("/api/v1/auth/login", credentials);
  },

  /**
   * Logout current user
   */
  logout: async (): Promise<void> => {
    return post<void>("/api/v1/auth/logout");
  },

  /**
   * Get current authenticated user info
   */
  getCurrentUser: async (): Promise<{ user: AuthUser }> => {
    return get<{ user: AuthUser }>("/api/v1/auth/me");
  },
};
