"use client";

import { useQuery } from "@tanstack/react-query";
import { rolesApi } from "@/lib/api";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";

export default function SettingsRolesPage() {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["roles"],
    queryFn: () => rolesApi.list(),
  });

  if (isLoading) {
    return <Skeleton className="h-44 w-full" />;
  }

  if (isError) {
    return <EmptyState title="Failed to load roles" description={error instanceof Error ? error.message : "Try again later."} />;
  }

  return (
    <div className="rounded-md border">
      {data?.roles.map((role) => (
        <div key={role.id} className="flex items-center justify-between border-b p-3 last:border-b-0">
          <span>{role.name}</span>
          <span className="text-sm text-muted-foreground">#{role.id}</span>
        </div>
      ))}
    </div>
  );
}
