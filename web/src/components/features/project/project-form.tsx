"use client";

import * as React from "react";
import { useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { projectsApi } from "@/lib/api";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Select } from "@/components/ui/select";

const projectFormSchema = z.object({
  name: z.string().min(1, "Name is required").max(255),
  identifier: z
    .string()
    .min(1, "Identifier is required")
    .max(100)
    .regex(/^[a-z0-9_-]+$/, "Identifier must be lowercase letters, numbers, hyphens, or underscores"),
  description: z.string().optional(),
  homepage: z.string().url("Must be a valid URL").optional().or(z.literal("")),
  is_public: z.boolean(),
  parent_id: z.number().nullable().optional(),
  inherit_members: z.boolean(),
});

type ProjectFormValues = z.infer<typeof projectFormSchema>;

interface ProjectFormProps {
  projectId?: number;
}

export function ProjectForm({ projectId }: ProjectFormProps) {
  const router = useRouter();
  const queryClient = useQueryClient();
  const isEdit = typeof projectId === "number";

  const { data: projectData } = useQuery({
    queryKey: ["project", projectId],
    queryFn: () => projectsApi.get(projectId!),
    enabled: isEdit,
  });

  const { data: projectsData } = useQuery({
    queryKey: ["projects", "project-form"],
    queryFn: () => projectsApi.list({ limit: 100 }),
  });

  const {
    register,
    setValue,
    watch,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<ProjectFormValues>({
    resolver: zodResolver(projectFormSchema),
    values: projectData?.project
      ? {
          name: projectData.project.name,
          identifier: projectData.project.identifier,
          description: projectData.project.description ?? "",
          homepage: projectData.project.homepage ?? "",
          is_public: projectData.project.is_public,
          parent_id: projectData.project.parent_id ?? null,
          inherit_members: projectData.project.inherit_members,
        }
      : {
          name: "",
          identifier: "",
          description: "",
          homepage: "",
          is_public: true,
          parent_id: null,
          inherit_members: false,
        },
  });

  const createMutation = useMutation({
    mutationFn: (values: ProjectFormValues) =>
      projectsApi.create({
        name: values.name,
        identifier: values.identifier,
        description: values.description || undefined,
        homepage: values.homepage || undefined,
        is_public: values.is_public,
        parent_id: values.parent_id ?? undefined,
        inherit_members: values.inherit_members,
      }),
    onSuccess: (response) => {
      queryClient.invalidateQueries({ queryKey: ["projects"] });
      router.push(`/projects/${response.project.id}`);
    },
  });

  const updateMutation = useMutation({
    mutationFn: (values: ProjectFormValues) =>
      projectsApi.update(projectId!, {
        name: values.name,
        description: values.description || undefined,
        homepage: values.homepage || undefined,
        is_public: values.is_public,
        parent_id: values.parent_id ?? null,
        inherit_members: values.inherit_members,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["project", projectId] });
      queryClient.invalidateQueries({ queryKey: ["projects"] });
      router.push(`/projects/${projectId}`);
    },
  });

  const onSubmit = (values: ProjectFormValues) => {
    if (isEdit) {
      updateMutation.mutate(values);
      return;
    }
    createMutation.mutate(values);
  };

  const parentOptions = [
    { value: "", label: "No parent project" },
    ...(projectsData?.projects
      .filter((project) => project.id !== projectId)
      .map((project) => ({ value: String(project.id), label: project.name })) ?? []),
  ];

  const saving = isSubmitting || createMutation.isPending || updateMutation.isPending;

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      <div className="grid gap-6 md:grid-cols-2">
        <div className="space-y-2">
          <Label htmlFor="name">Name</Label>
          <Input id="name" {...register("name")} />
          {errors.name && <p className="text-sm text-destructive">{errors.name.message}</p>}
        </div>
        <div className="space-y-2">
          <Label htmlFor="identifier">Identifier</Label>
          <Input id="identifier" {...register("identifier")} disabled={isEdit} />
          {errors.identifier && <p className="text-sm text-destructive">{errors.identifier.message}</p>}
        </div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="description">Description</Label>
        <Textarea id="description" rows={4} {...register("description")} />
      </div>

      <div className="space-y-2">
        <Label htmlFor="homepage">Homepage</Label>
        <Input id="homepage" type="url" {...register("homepage")} />
        {errors.homepage && <p className="text-sm text-destructive">{errors.homepage.message}</p>}
      </div>

      <div className="space-y-2">
        <Label htmlFor="parent_id">Parent Project</Label>
        <Select
          id="parent_id"
          options={parentOptions}
          value={watch("parent_id") ? String(watch("parent_id")) : ""}
          onValueChange={(value) => setValue("parent_id", value ? Number(value) : null, { shouldDirty: true })}
        />
      </div>

      <div className="space-y-4">
        <div className="flex items-center justify-between rounded-md border p-3">
          <div>
            <p className="font-medium">Public project</p>
            <p className="text-sm text-muted-foreground">Visible to all logged-in users</p>
          </div>
          <Switch
            checked={watch("is_public")}
            onCheckedChange={(checked) => setValue("is_public", checked, { shouldDirty: true })}
          />
        </div>
        <div className="flex items-center justify-between rounded-md border p-3">
          <div>
            <p className="font-medium">Inherit members</p>
            <p className="text-sm text-muted-foreground">Inherit members from parent project</p>
          </div>
          <Switch
            checked={watch("inherit_members")}
            onCheckedChange={(checked) => setValue("inherit_members", checked, { shouldDirty: true })}
          />
        </div>
      </div>

      <div className="flex items-center gap-3">
        <Button type="submit" disabled={saving}>
          {saving ? "Saving..." : isEdit ? "Update Project" : "Create Project"}
        </Button>
        <Button type="button" variant="outline" onClick={() => router.back()} disabled={saving}>
          Cancel
        </Button>
      </div>
    </form>
  );
}
