"use client";

import { useEffect, useState } from "react";
import { useRouter, usePathname } from "next/navigation";
import { useAuthStore } from "@/stores";
import { AUTH_ROUTES } from "@/lib/constants/routes";
import { Loading } from "@/components/ui/loading";

interface AuthGuardProps {
  children: React.ReactNode;
  requireAdmin?: boolean;
  requireAuth?: boolean;
}

/**
 * Component to protect routes that require authentication
 * Redirects to login if user is not authenticated
 */
export function AuthGuard({
  children,
  requireAdmin = false,
  requireAuth = true,
}: AuthGuardProps) {
  const router = useRouter();
  const pathname = usePathname();
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const user = useAuthStore((state) => state.user);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => setIsReady(true), 300);
    return () => clearTimeout(timer);
  }, []);

  useEffect(() => {
    if (!isReady || !requireAuth) return;

    if (!isAuthenticated) {
      // Store the attempted URL for redirect after login
      if (pathname && pathname !== AUTH_ROUTES.LOGIN) {
        sessionStorage.setItem("redirect_after_login", pathname);
      }
      router.push(AUTH_ROUTES.LOGIN);
    } else if (requireAdmin && !user?.admin) {
      router.push("/");
    }
  }, [isReady, requireAuth, requireAdmin, isAuthenticated, user, pathname, router]);

  // Show loading state while checking auth
  if (!isReady || (requireAuth && !isAuthenticated)) {
    if (!isReady) {
      return <Loading fullScreen text="Checking authentication..." />;
    }
    // Will redirect, show nothing
    return null;
  }

  // Don't render if admin required but user is not admin
  if (requireAuth && requireAdmin && !user?.admin) {
    return null;
  }

  return <>{children}</>;
}

/**
 * Component to redirect authenticated users away from auth pages (like login)
 */
export function GuestGuard({ children }: { children: React.ReactNode }) {
  const router = useRouter();
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => setIsReady(true), 300);
    return () => clearTimeout(timer);
  }, []);

  useEffect(() => {
    if (!isReady) return;

    if (isAuthenticated) {
      // Redirect to stored URL or home
      const redirectUrl = sessionStorage.getItem("redirect_after_login") || "/";
      sessionStorage.removeItem("redirect_after_login");
      router.push(redirectUrl);
    }
  }, [isReady, isAuthenticated, router]);

  // Show loading while checking
  if (!isReady) {
    return <Loading fullScreen text="Loading..." />;
  }

  // Will redirect if authenticated
  if (isAuthenticated) {
    return null;
  }

  return <>{children}</>;
}
