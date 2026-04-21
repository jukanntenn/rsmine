import { useMemo } from "react";
import { useAuthStore } from "@/stores";
import { PERMISSIONS, type Permission } from "@/lib/constants/permissions";

/**
 * Hook to check if current user has a specific permission
 */
export function usePermission() {
  const { user } = useAuthStore();

  // Calculate permissions based on user admin status
  // Using useMemo instead of useEffect to avoid cascading renders
  const permissions = useMemo(() => {
    if (user?.admin) {
      return new Set(Object.values(PERMISSIONS));
    }
    // TODO: Load actual permissions from API when available
    return new Set<string>();
  }, [user]);

  const hasPermission = (permission: Permission): boolean => {
    if (user?.admin) return true;
    return permissions.has(permission);
  };

  const hasAnyPermission = (perms: Permission[]): boolean => {
    if (user?.admin) return true;
    return perms.some((p) => permissions.has(p));
  };

  const hasAllPermissions = (perms: Permission[]): boolean => {
    if (user?.admin) return true;
    return perms.every((p) => permissions.has(p));
  };

  return {
    hasPermission,
    hasAnyPermission,
    hasAllPermissions,
    permissions,
  };
}