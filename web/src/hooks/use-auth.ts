"use client";

import { useCallback } from "react";
import { useRouter } from "next/navigation";
import { useAuthStore } from "@/stores";
import { authApi } from "@/lib/api";
import { AUTH_ROUTES, DASHBOARD_ROUTES } from "@/lib/constants/routes";
import type { LoginRequest } from "@/types";

/**
 * Hook to manage authentication state and operations
 */
export function useAuth() {
  const router = useRouter();
  const { user, token, isAuthenticated, login, logout, setUser, setToken } =
    useAuthStore();

  /**
   * Sign in with username and password
   */
  const signIn = useCallback(
    async (credentials: LoginRequest) => {
      const response = await authApi.login(credentials);
      login(response.user, response.token);
      router.push(DASHBOARD_ROUTES.HOME);
      return response;
    },
    [login, router]
  );

  /**
   * Sign out the current user
   */
  const signOut = useCallback(async () => {
    try {
      await authApi.logout();
    } catch {
      // Ignore logout API errors, proceed with local logout
    } finally {
      logout();
      router.push(AUTH_ROUTES.LOGIN);
    }
  }, [logout, router]);

  /**
   * Refresh current user data from API
   */
  const refreshUser = useCallback(async () => {
    if (!token) return null;

    try {
      const response = await authApi.getCurrentUser();
      setUser(response.user);
      return response.user;
    } catch {
      logout();
      return null;
    }
  }, [token, setUser, logout]);

  return {
    user,
    token,
    isAuthenticated,
    isAdmin: user?.admin ?? false,
    signIn,
    signOut,
    refreshUser,
    setUser,
    setToken,
  };
}