"use client";

import * as React from "react";
import { useParams } from "next/navigation";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { FolderKanban, Plus, Trash2 } from "lucide-react";
import { categoriesApi, projectsApi } from "@/lib/api";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator } from "@/components/layout";
import { ProjectHeader } from "@/components/features/project";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";

export default function ProjectCategoriesPage() {
  const params = useParams();
  const projectId = Number(params.id);
  const queryClient = useQueryClient();
  const [name, setName] = React.useState("");

  const { data: projectData, isLoading: projectLoading } = useQuery({
    queryKey: ["project", projectId],
    queryFn: () => projectsApi.get(projectId),
  });

  const { data: categoriesData, isLoading: categoriesLoading } = useQuery({
    queryKey: ["categories", projectId],
    queryFn: () => categoriesApi.list(projectId),
  });

  const createMutation = useMutation({
    mutationFn: () => categoriesApi.create(projectId, { name }),
    onSuccess: () => {
      setName("");
      queryClient.invalidateQueries({ queryKey: ["categories", projectId] });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (id: number) => categoriesApi.delete(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["categories", projectId] });
    },
  });

  if (projectLoading || !projectData?.project) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-40" />
        <Skeleton className="h-20 w-full" />
      </div>
    );
  }

  const categories = categoriesData?.issue_categories ?? [];

  return (
    <div className="space-y-6">
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink href="/projects">Projects</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink href={`/projects/${projectId}`}>{projectData.project.name}</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink>Categories</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      <ProjectHeader project={projectData.project} />

      <div className="flex items-center gap-2">
        <Input value={name} onChange={(event) => setName(event.target.value)} placeholder="New category name" />
        <Button onClick={() => createMutation.mutate()} disabled={!name.trim() || createMutation.isPending}>
          <Plus className="h-4 w-4 mr-1" />
          Add
        </Button>
      </div>

      {categoriesLoading ? (
        <Skeleton className="h-32 w-full" />
      ) : categories.length === 0 ? (
        <EmptyState
          icon={<FolderKanban className="h-8 w-8 text-muted-foreground" />}
          title="No categories"
          description="Create your first issue category for this project."
        />
      ) : (
        <div className="space-y-2">
          {categories.map((category) => (
            <Card key={category.id}>
              <CardContent className="flex items-center justify-between py-3">
                <div>
                  <p className="font-medium">{category.name}</p>
                  {category.assigned_to?.name && (
                    <p className="text-sm text-muted-foreground">Assigned to: {category.assigned_to.name}</p>
                  )}
                </div>
                <Button
                  variant="ghost"
                  size="icon"
                  onClick={() => deleteMutation.mutate(category.id)}
                  disabled={deleteMutation.isPending}
                >
                  <Trash2 className="h-4 w-4" />
                </Button>
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}
