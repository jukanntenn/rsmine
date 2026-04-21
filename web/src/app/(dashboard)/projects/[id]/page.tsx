"use client";

import * as React from "react";
import { useQuery } from "@tanstack/react-query";
import { useParams } from "next/navigation";
import { FolderKanban } from "lucide-react";
import { projectsApi, issuesApi } from "@/lib/api";
import { ProjectHeader, ProjectStats } from "@/components/features/project";
import { IssueList } from "@/components/features/issue";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink } from "@/components/layout";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";

export default function ProjectDetailPage() {
  const params = useParams();
  const projectId = Number(params.id);

  const { data: projectData, isLoading: projectLoading, isError, error } = useQuery({
    queryKey: ["project", projectId],
    queryFn: () => projectsApi.get(projectId),
  });

  const { data: issuesData } = useQuery({
    queryKey: ["issues", { project_id: projectId, limit: 100 }],
    queryFn: () => issuesApi.list({ project_id: projectId, limit: 100 }),
  });

  if (projectLoading) {
    return (
      <div className="space-y-6">
        <div className="flex items-center gap-2">
          <Skeleton className="h-4 w-20" />
          <Skeleton className="h-4 w-4" />
          <Skeleton className="h-4 w-32" />
        </div>
        <div className="space-y-4">
          <Skeleton className="h-10 w-64" />
          <Skeleton className="h-6 w-96" />
        </div>
        <Skeleton className="h-12 w-full" />
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
          {Array.from({ length: 4 }).map((_, i) => (
            <Skeleton key={i} className="h-24" />
          ))}
        </div>
      </div>
    );
  }

  if (isError || !projectData?.project) {
    return (
      <EmptyState
        icon={<FolderKanban className="h-10 w-10 text-muted-foreground" />}
        title="Project not found"
        description={
          error instanceof Error
            ? error.message
            : "The project you're looking for doesn't exist or you don't have access to it."
        }
      />
    );
  }

  const project = projectData.project;
  const issues = issuesData?.issues ?? [];

  // Calculate issue statistics
  const stats = {
    total: issues.length,
    open: issues.filter((i) => !i.status?.is_closed).length,
    closed: issues.filter((i) => i.status?.is_closed).length,
    in_progress: issues.filter((i) => 
      i.status && !i.status.is_closed && i.status.name.toLowerCase().includes("progress")
    ).length,
  };

  return (
    <div className="space-y-6">
      {/* Breadcrumb */}
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink href="/projects">Projects</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbItem>
          <BreadcrumbLink>{project.name}</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      {/* Project Header with Tabs */}
      <ProjectHeader project={project} />

      {/* Issue Statistics */}
      <ProjectStats stats={stats} />

      {/* Issues Section */}
      <div className="space-y-4">
        <h2 className="text-xl font-semibold">Issues</h2>
        <IssueList projectId={projectId} />
      </div>
    </div>
  );
}
