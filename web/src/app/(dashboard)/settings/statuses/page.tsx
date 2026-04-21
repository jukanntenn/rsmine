"use client";

import { useQuery } from "@tanstack/react-query";
import { statusesApi } from "@/lib/api";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";

export default function SettingsStatusesPage() {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["issue-statuses"],
    queryFn: () => statusesApi.list(),
  });

  if (isLoading) {
    return <Skeleton className="h-44 w-full" />;
  }

  if (isError) {
    return (
      <EmptyState title="Failed to load statuses" description={error instanceof Error ? error.message : "Try again later."} />
    );
  }

  return (
    <div className="rounded-md border">
      {data?.issue_statuses.map((status) => (
        <div key={status.id} className="flex items-center justify-between border-b p-3 last:border-b-0">
          <span>{status.name}</span>
          <Badge variant={status.is_closed ? "secondary" : "default"}>{status.is_closed ? "Closed" : "Open"}</Badge>
        </div>
      ))}
    </div>
  );
}
