"use client";

import { useAuthStore } from "@/stores";

/**
 * Hook to access current user information
 */
export function useCurrentUser() {
  const user = useAuthStore((state) => state.user);
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);

  return {
    user,
    isAuthenticated,
    isAdmin: user?.admin ?? false,
    userId: user?.id,
    userName: user ? `${user.firstname} ${user.lastname}` : null,
    userLogin: user?.login ?? null,
    userEmail: user?.mail ?? null,
    userLanguage: user?.language ?? null,
  };
}