"use client";

import * as React from "react";
import { useQuery } from "@tanstack/react-query";
import { useParams } from "next/navigation";
import { FolderKanban } from "lucide-react";
import { projectsApi } from "@/lib/api";
import { ProjectHeader } from "@/components/features/project";
import { MemberList } from "@/components/features/member";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink } from "@/components/layout";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";

export default function ProjectMembersPage() {
  const params = useParams();
  const projectId = Number(params.id);

  const { data: projectData, isLoading, isError, error } = useQuery({
    queryKey: ["project", projectId],
    queryFn: () => projectsApi.get(projectId),
  });

  if (isLoading) {
    return (
      <div className="space-y-6">
        <div className="flex items-center gap-2">
          <Skeleton className="h-4 w-20" />
          <Skeleton className="h-4 w-4" />
          <Skeleton className="h-4 w-32" />
          <Skeleton className="h-4 w-4" />
          <Skeleton className="h-4 w-20" />
        </div>
        <div className="space-y-4">
          <Skeleton className="h-10 w-64" />
          <Skeleton className="h-6 w-96" />
        </div>
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-64 w-full" />
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

  return (
    <div className="space-y-6">
      {/* Breadcrumb */}
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink href="/projects">Projects</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbItem>
          <BreadcrumbLink href={`/projects/${projectId}`}>
            {project.name}
          </BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbItem>
          <BreadcrumbLink>Members</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      {/* Project Header with Tabs */}
      <ProjectHeader project={project} />

      {/* Members List */}
      <MemberList projectId={projectId} />
    </div>
  );
}
