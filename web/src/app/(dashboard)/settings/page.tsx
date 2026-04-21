"use client";

import { useQueries } from "@tanstack/react-query";
import { prioritiesApi, rolesApi, statusesApi, trackersApi } from "@/lib/api";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";

export default function SettingsOverviewPage() {
  const [trackersQuery, statusesQuery, prioritiesQuery, rolesQuery] = useQueries({
    queries: [
      { queryKey: ["trackers"], queryFn: () => trackersApi.list() },
      { queryKey: ["issue-statuses"], queryFn: () => statusesApi.list() },
      { queryKey: ["priorities"], queryFn: () => prioritiesApi.list() },
      { queryKey: ["roles"], queryFn: () => rolesApi.list() },
    ],
  });

  if (trackersQuery.isLoading || statusesQuery.isLoading || prioritiesQuery.isLoading || rolesQuery.isLoading) {
    return <Skeleton className="h-40 w-full" />;
  }

  const cards = [
    { title: "Trackers", value: trackersQuery.data?.trackers.length ?? 0 },
    { title: "Issue Statuses", value: statusesQuery.data?.issue_statuses.length ?? 0 },
    { title: "Priorities", value: prioritiesQuery.data?.issue_priorities.length ?? 0 },
    { title: "Roles", value: rolesQuery.data?.roles.length ?? 0 },
  ];

  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
      {cards.map((item) => (
        <Card key={item.title}>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">{item.title}</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">{item.value}</p>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
