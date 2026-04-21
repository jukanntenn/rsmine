"use client";

import * as React from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { useParams, useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { FolderKanban, Loader2, Save, Trash2 } from "lucide-react";
import { projectsApi } from "@/lib/api";
import { ProjectHeader } from "@/components/features/project";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink } from "@/components/layout";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Select } from "@/components/ui/select";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";
import { AlertDialog } from "@/components/ui/alert-dialog";
import { toast } from "sonner";

const projectSettingsSchema = z.object({
  name: z.string().min(1, "Name is required").max(255),
  identifier: z.string().min(1, "Identifier is required").max(100),
  description: z.string().optional(),
  homepage: z.string().url("Must be a valid URL").optional().or(z.literal("")),
  is_public: z.boolean(),
  status: z.union([z.literal(1), z.literal(5), z.literal(9)]),
  inherit_members: z.boolean(),
});

type ProjectSettingsForm = z.infer<typeof projectSettingsSchema>;

const statusOptions = [
  { value: "1", label: "Active" },
  { value: "5", label: "Closed" },
  { value: "9", label: "Archived" },
];

export default function ProjectSettingsPage() {
  const params = useParams();
  const router = useRouter();
  const queryClient = useQueryClient();
  const projectId = Number(params.id);
  const [showDeleteDialog, setShowDeleteDialog] = React.useState(false);

  const { data: projectData, isLoading, isError, error } = useQuery({
    queryKey: ["project", projectId],
    queryFn: () => projectsApi.get(projectId),
  });

  const updateMutation = useMutation({
    mutationFn: (data: ProjectSettingsForm) =>
      projectsApi.update(projectId, {
        name: data.name,
        description: data.description || undefined,
        homepage: data.homepage || undefined,
        is_public: data.is_public,
        status: data.status,
        inherit_members: data.inherit_members,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["project", projectId] });
      queryClient.invalidateQueries({ queryKey: ["projects"] });
      toast.success("Project settings saved");
    },
    onError: (error) => {
      toast.error(error instanceof Error ? error.message : "Failed to save settings");
    },
  });

  const deleteMutation = useMutation({
    mutationFn: () => projectsApi.delete(projectId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["projects"] });
      toast.success("Project deleted");
      router.push("/projects");
    },
    onError: (error) => {
      toast.error(error instanceof Error ? error.message : "Failed to delete project");
    },
  });

  const {
    register,
    handleSubmit,
    setValue,
    watch,
    formState: { errors, isDirty },
  } = useForm<ProjectSettingsForm>({
    resolver: zodResolver(projectSettingsSchema),
    values: projectData?.project
      ? {
          name: projectData.project.name,
          identifier: projectData.project.identifier,
          description: projectData.project.description || "",
          homepage: projectData.project.homepage || "",
          is_public: projectData.project.is_public,
          status: projectData.project.status,
          inherit_members: projectData.project.inherit_members,
        }
      : undefined,
  });

  const onSubmit = (data: ProjectSettingsForm) => {
    updateMutation.mutate(data);
  };

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
        <Skeleton className="h-96 w-full max-w-2xl" />
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
          <BreadcrumbLink>Settings</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      {/* Project Header with Tabs */}
      <ProjectHeader project={project} />

      {/* Settings Form */}
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-6 max-w-2xl">
        {/* General Settings */}
        <Card>
          <CardHeader>
            <CardTitle>General Settings</CardTitle>
            <CardDescription>
              Update your project&apos;s basic information and visibility.
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="name">Project Name</Label>
              <Input
                id="name"
                {...register("name")}
                placeholder="Enter project name"
              />
              {errors.name && (
                <p className="text-sm text-destructive">{errors.name.message}</p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="identifier">Identifier</Label>
              <Input
                id="identifier"
                {...register("identifier")}
                placeholder="project-identifier"
                disabled
              />
              <p className="text-xs text-muted-foreground">
                The identifier cannot be changed after creation.
              </p>
            </div>

            <div className="space-y-2">
              <Label htmlFor="description">Description</Label>
              <Textarea
                id="description"
                {...register("description")}
                placeholder="Project description..."
                rows={3}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="homepage">Homepage</Label>
              <Input
                id="homepage"
                {...register("homepage")}
                placeholder="https://example.com"
                type="url"
              />
              {errors.homepage && (
                <p className="text-sm text-destructive">{errors.homepage.message}</p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="status">Status</Label>
              <Select
                options={statusOptions}
                value={String(watch("status"))}
                onValueChange={(value) => setValue("status", Number(value) as 1 | 5 | 9, { shouldDirty: true })}
              />
            </div>
          </CardContent>
        </Card>

        {/* Visibility Settings */}
        <Card>
          <CardHeader>
            <CardTitle>Visibility</CardTitle>
            <CardDescription>
              Control who can see and access this project.
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <Label htmlFor="is_public">Public Project</Label>
                <p className="text-sm text-muted-foreground">
                  Public projects are visible to all users.
                </p>
              </div>
              <Switch
                id="is_public"
                checked={watch("is_public")}
                onCheckedChange={(checked) => setValue("is_public", checked, { shouldDirty: true })}
              />
            </div>

            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <Label htmlFor="inherit_members">Inherit Members</Label>
                <p className="text-sm text-muted-foreground">
                  Members of parent projects will also have access.
                </p>
              </div>
              <Switch
                id="inherit_members"
                checked={watch("inherit_members")}
                onCheckedChange={(checked) => setValue("inherit_members", checked, { shouldDirty: true })}
              />
            </div>
          </CardContent>
        </Card>

        {/* Save Button */}
        <div className="flex items-center gap-4">
          <Button type="submit" disabled={!isDirty || updateMutation.isPending}>
            {updateMutation.isPending ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Saving...
              </>
            ) : (
              <>
                <Save className="mr-2 h-4 w-4" />
                Save Changes
              </>
            )}
          </Button>
        </div>
      </form>

      {/* Danger Zone */}
      <Card className="border-destructive/50">
        <CardHeader>
          <CardTitle className="text-destructive">Danger Zone</CardTitle>
          <CardDescription>
            Irreversible actions for this project. Please proceed with caution.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-between">
            <div className="space-y-0.5">
              <p className="font-medium">Delete this project</p>
              <p className="text-sm text-muted-foreground">
                Once deleted, the project and all its data cannot be recovered.
              </p>
            </div>
            <Button variant="destructive" onClick={() => setShowDeleteDialog(true)}>
              <Trash2 className="mr-2 h-4 w-4" />
              Delete Project
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Delete Confirmation Dialog */}
      <AlertDialog
        open={showDeleteDialog}
        onOpenChange={setShowDeleteDialog}
        title="Delete Project"
        description={`Are you sure you want to delete "${project.name}"? This will permanently delete all issues, members, and other data associated with this project. This action cannot be undone.`}
        confirmText={deleteMutation.isPending ? "Deleting..." : "Delete"}
        cancelText="Cancel"
        onConfirm={() => deleteMutation.mutate()}
        variant="destructive"
        loading={deleteMutation.isPending}
      />
    </div>
  );
}
