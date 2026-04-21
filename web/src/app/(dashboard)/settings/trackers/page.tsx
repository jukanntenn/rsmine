"use client";

import { useQuery } from "@tanstack/react-query";
import { trackersApi } from "@/lib/api";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";

export default function SettingsTrackersPage() {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["trackers"],
    queryFn: () => trackersApi.list(),
  });

  if (isLoading) {
    return <Skeleton className="h-44 w-full" />;
  }

  if (isError) {
    return <EmptyState title="Failed to load trackers" description={error instanceof Error ? error.message : "Try again later."} />;
  }

  return (
    <div className="rounded-md border">
      {data?.trackers.map((tracker) => (
        <div key={tracker.id} className="flex items-center justify-between border-b p-3 last:border-b-0">
          <span>{tracker.name}</span>
          <span className="text-sm text-muted-foreground">#{tracker.id}</span>
        </div>
      ))}
    </div>
  );
}
