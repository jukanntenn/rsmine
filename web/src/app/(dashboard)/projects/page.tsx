import type { Metadata } from "next";
import { dehydrate, HydrationBoundary } from "@tanstack/react-query";
import { FolderKanban } from "lucide-react";
import { getQueryClient, projectsApi } from "@/lib/api";
import { ProjectList } from "@/components/features/project";

export const metadata: Metadata = {
  title: "Projects - Rsmine",
  description: "View and manage all projects",
};

/**
 * Projects list page
 * Displays all projects in a grid layout with search and filtering
 */
export default async function ProjectsPage() {
  const queryClient = getQueryClient();

  // Prefetch projects for SSR
  await queryClient.prefetchQuery({
    queryKey: ["projects", { offset: 0, limit: 12 }],
    queryFn: () => projectsApi.list({ offset: 0, limit: 12 }),
  });

  return (
    <HydrationBoundary state={dehydrate(queryClient)}>
      <div className="space-y-6">
        {/* Page Header */}
        <div className="flex items-center gap-3">
          <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
            <FolderKanban className="h-5 w-5 text-primary" />
          </div>
          <div>
            <h1 className="text-2xl font-semibold tracking-tight">Projects</h1>
            <p className="text-sm text-muted-foreground">
              Manage your projects and track progress
            </p>
          </div>
        </div>

        {/* Project List */}
        <ProjectList />
      </div>
    </HydrationBoundary>
  );
}
