"use client";

import { useQuery } from "@tanstack/react-query";
import { prioritiesApi } from "@/lib/api";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";

export default function SettingsPrioritiesPage() {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["priorities"],
    queryFn: () => prioritiesApi.list(),
  });

  if (isLoading) {
    return <Skeleton className="h-44 w-full" />;
  }

  if (isError) {
    return (
      <EmptyState title="Failed to load priorities" description={error instanceof Error ? error.message : "Try again later."} />
    );
  }

  return (
    <div className="rounded-md border">
      {data?.issue_priorities.map((priority) => (
        <div key={priority.id} className="flex items-center justify-between border-b p-3 last:border-b-0">
          <span>{priority.name}</span>
          <span className="text-sm text-muted-foreground">#{priority.id}</span>
        </div>
      ))}
    </div>
  );
}
