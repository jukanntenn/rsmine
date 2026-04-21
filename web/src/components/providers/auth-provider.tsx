"use client";

import { useEffect, useState } from "react";
import { useRouter, usePathname } from "next/navigation";
import { useAuthStore } from "@/stores";
import { authApi } from "@/lib/api";
import { AUTH_ROUTES } from "@/lib/constants/routes";

interface AuthProviderProps {
  children: React.ReactNode;
}

// Routes that don't require authentication
const PUBLIC_ROUTES = [AUTH_ROUTES.LOGIN, AUTH_ROUTES.REGISTER];

/**
 * Auth provider that handles session validation and auto-refresh
 */
export function AuthProvider({ children }: AuthProviderProps) {
  const router = useRouter();
  const pathname = usePathname();
  const token = useAuthStore((state) => state.token);
  const setUser = useAuthStore((state) => state.setUser);
  const logout = useAuthStore((state) => state.logout);

  // Track initialization state for rendering
  const [isReady, setIsReady] = useState(true);

  // Initialize auth on mount - run only once across all instances
  useEffect(() => {
    let active = true;
    const fallbackTimer = setTimeout(() => {
      if (active) {
        setIsReady(true);
      }
    }, 3000);
    const initAuth = async () => {
      try {
        if (token) {
          const response = await authApi.getCurrentUser();
          if (active) {
            setUser(response.user);
          }
        }
      } catch {
        try {
          logout();
        } finally {
          if (active) {
            setIsReady(true);
          }
        }
        return;
      }
      if (active) {
        setIsReady(true);
      }
      clearTimeout(fallbackTimer);
    };

    initAuth();
    return () => {
      active = false;
      clearTimeout(fallbackTimer);
    };
  }, [token, setUser, logout]);

  // Handle redirect for protected routes
  useEffect(() => {
    if (!isReady) return;

    const isPublicRoute = PUBLIC_ROUTES.some((route) =>
      pathname?.startsWith(route)
    );

    if (!token && !isPublicRoute) {
      if (pathname) {
        sessionStorage.setItem("redirect_after_login", pathname);
      }
      router.push(AUTH_ROUTES.LOGIN);
    }
  }, [isReady, token, pathname, router]);

  // Redirect authenticated users away from login page
  useEffect(() => {
    if (!isReady) return;

    const isLoginPage = pathname === AUTH_ROUTES.LOGIN;

    if (token && isLoginPage) {
      const redirectUrl = sessionStorage.getItem("redirect_after_login") || "/";
      sessionStorage.removeItem("redirect_after_login");
      router.push(redirectUrl);
    }
  }, [isReady, token, pathname, router]);

  return <>{children}</>;
}
