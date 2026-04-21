"use client";

import * as React from "react";
import { useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { issuesApi, membersApi, prioritiesApi, statusesApi, trackersApi } from "@/lib/api";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Select } from "@/components/ui/select";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";

const issueFormSchema = z.object({
  project_id: z.number().min(1, "Project is required"),
  tracker_id: z.number().min(1, "Tracker is required"),
  subject: z.string().min(1, "Subject is required").max(255),
  description: z.string().optional(),
  status_id: z.number().optional(),
  priority_id: z.number().min(1, "Priority is required"),
  assigned_to_id: z.number().nullable().optional(),
  start_date: z.string().optional(),
  due_date: z.string().optional(),
  estimated_hours: z.number().nullable().optional(),
  done_ratio: z.number().min(0).max(100).optional(),
  notes: z.string().optional(),
  is_private: z.boolean().optional(),
});

type IssueFormValues = z.infer<typeof issueFormSchema>;

interface IssueFormProps {
  projectId?: number;
  issueId?: number;
}

export function IssueForm({ projectId, issueId }: IssueFormProps) {
  const router = useRouter();
  const queryClient = useQueryClient();
  const isEdit = typeof issueId === "number";

  const { data: issueData } = useQuery({
    queryKey: ["issue", issueId, "form"],
    queryFn: () => issuesApi.get(issueId!),
    enabled: isEdit,
  });

  const resolvedProjectId = projectId || issueData?.issue.project?.id;

  const { data: trackersData } = useQuery({
    queryKey: ["trackers"],
    queryFn: () => trackersApi.list(),
  });

  const { data: prioritiesData } = useQuery({
    queryKey: ["priorities"],
    queryFn: () => prioritiesApi.list(),
  });

  const { data: statusesData } = useQuery({
    queryKey: ["issue-statuses"],
    queryFn: () => statusesApi.list(),
    enabled: isEdit,
  });

  const { data: membersData } = useQuery({
    queryKey: ["members", resolvedProjectId, "issue-form"],
    queryFn: () => membersApi.list(resolvedProjectId!, { limit: 100 }),
    enabled: Boolean(resolvedProjectId),
  });

  const {
    register,
    setValue,
    watch,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<IssueFormValues>({
    resolver: zodResolver(issueFormSchema),
    values: issueData?.issue
      ? {
          project_id: issueData.issue.project_id ?? issueData.issue.project.id,
          tracker_id: issueData.issue.tracker_id ?? issueData.issue.tracker.id,
          subject: issueData.issue.subject,
          description: issueData.issue.description ?? "",
          status_id: issueData.issue.status_id ?? issueData.issue.status.id,
          priority_id: issueData.issue.priority_id ?? issueData.issue.priority.id,
          assigned_to_id: issueData.issue.assigned_to_id ?? issueData.issue.assigned_to?.id ?? null,
          start_date: issueData.issue.start_date ?? "",
          due_date: issueData.issue.due_date ?? "",
          estimated_hours: issueData.issue.estimated_hours ?? null,
          done_ratio: issueData.issue.done_ratio,
          notes: "",
          is_private: issueData.issue.is_private,
        }
      : {
          project_id: projectId || 0,
          tracker_id: 0,
          subject: "",
          description: "",
          priority_id: 0,
          assigned_to_id: null,
          start_date: "",
          due_date: "",
          estimated_hours: null,
          done_ratio: 0,
          notes: "",
          is_private: false,
        },
  });

  const createMutation = useMutation({
    mutationFn: (values: IssueFormValues) =>
      issuesApi.create({
        project_id: values.project_id,
        tracker_id: values.tracker_id,
        subject: values.subject,
        description: values.description || undefined,
        status_id: values.status_id,
        priority_id: values.priority_id,
        assigned_to_id: values.assigned_to_id ?? undefined,
        start_date: values.start_date || undefined,
        due_date: values.due_date || undefined,
        estimated_hours: values.estimated_hours ?? undefined,
        done_ratio: values.done_ratio ?? undefined,
        is_private: values.is_private ?? false,
      }),
    onSuccess: (response) => {
      queryClient.invalidateQueries({ queryKey: ["issues"] });
      router.push(`/issues/${response.issue.id}`);
    },
  });

  const updateMutation = useMutation({
    mutationFn: (values: IssueFormValues) =>
      issuesApi.update(issueId!, {
        tracker_id: values.tracker_id,
        subject: values.subject,
        description: values.description || undefined,
        status_id: values.status_id,
        priority_id: values.priority_id,
        assigned_to_id: values.assigned_to_id ?? undefined,
        start_date: values.start_date || undefined,
        due_date: values.due_date || undefined,
        estimated_hours: values.estimated_hours ?? undefined,
        done_ratio: values.done_ratio ?? undefined,
        notes: values.notes || undefined,
        is_private: values.is_private ?? false,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["issue", issueId] });
      queryClient.invalidateQueries({ queryKey: ["issues"] });
      router.push(`/issues/${issueId}`);
    },
  });

  const onSubmit = (values: IssueFormValues) => {
    if (isEdit) {
      updateMutation.mutate(values);
      return;
    }
    createMutation.mutate(values);
  };

  const trackerOptions = [{ value: "", label: "Select tracker" }, ...(trackersData?.trackers.map((item) => ({
    value: String(item.id),
    label: item.name,
  })) ?? [])];
  const statusOptions = statusesData?.issue_statuses.map((item) => ({
    value: String(item.id),
    label: item.name,
  })) ?? [];
  const priorityOptions = [{ value: "", label: "Select priority" }, ...(prioritiesData?.issue_priorities.map((item) => ({
    value: String(item.id),
    label: item.name,
  })) ?? [])];
  const assigneeOptions = [
    { value: "", label: "Unassigned" },
    ...(membersData?.memberships.map((item) => ({
      value: String(item.user.id),
      label: item.user.name,
    })) ?? []),
  ];

  const saving = isSubmitting || createMutation.isPending || updateMutation.isPending;

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      <div className="grid gap-6 md:grid-cols-2">
        <div className="space-y-2">
          <Label htmlFor="tracker_id">Tracker</Label>
          <Select
            id="tracker_id"
            options={trackerOptions}
            value={watch("tracker_id") ? String(watch("tracker_id")) : ""}
            onValueChange={(value) => setValue("tracker_id", Number(value), { shouldDirty: true })}
          />
          {errors.tracker_id && <p className="text-sm text-destructive">{errors.tracker_id.message}</p>}
        </div>

        <div className="space-y-2">
          <Label htmlFor="priority_id">Priority</Label>
          <Select
            id="priority_id"
            options={priorityOptions}
            value={watch("priority_id") ? String(watch("priority_id")) : ""}
            onValueChange={(value) => setValue("priority_id", Number(value), { shouldDirty: true })}
          />
          {errors.priority_id && <p className="text-sm text-destructive">{errors.priority_id.message}</p>}
        </div>
      </div>

      {isEdit && (
        <div className="space-y-2">
          <Label htmlFor="status_id">Status</Label>
          <Select
            id="status_id"
            options={statusOptions}
            value={watch("status_id") ? String(watch("status_id")) : ""}
            onValueChange={(value) => setValue("status_id", value ? Number(value) : undefined, { shouldDirty: true })}
          />
        </div>
      )}

      <div className="space-y-2">
        <Label htmlFor="subject">Subject</Label>
        <Input id="subject" {...register("subject")} />
        {errors.subject && <p className="text-sm text-destructive">{errors.subject.message}</p>}
      </div>

      <div className="space-y-2">
        <Label htmlFor="description">Description</Label>
        <Textarea id="description" rows={6} {...register("description")} />
      </div>

      <div className="space-y-2">
        <Label htmlFor="assigned_to_id">Assignee</Label>
        <Select
          id="assigned_to_id"
          options={assigneeOptions}
          value={watch("assigned_to_id") ? String(watch("assigned_to_id")) : ""}
          onValueChange={(value) => setValue("assigned_to_id", value ? Number(value) : null, { shouldDirty: true })}
        />
      </div>

      <div className="grid gap-6 md:grid-cols-3">
        <div className="space-y-2">
          <Label htmlFor="start_date">Start Date</Label>
          <Input id="start_date" type="date" {...register("start_date")} />
        </div>
        <div className="space-y-2">
          <Label htmlFor="due_date">Due Date</Label>
          <Input id="due_date" type="date" {...register("due_date")} />
        </div>
        <div className="space-y-2">
          <Label htmlFor="estimated_hours">Estimated Hours</Label>
          <Input
            id="estimated_hours"
            type="number"
            step="0.5"
            value={watch("estimated_hours") ?? ""}
            onChange={(event) =>
              setValue("estimated_hours", event.target.value ? Number(event.target.value) : null, { shouldDirty: true })
            }
          />
        </div>
      </div>

      {isEdit && (
        <>
          <div className="space-y-2">
            <Label htmlFor="done_ratio">Done Ratio (%)</Label>
            <Input
              id="done_ratio"
              type="number"
              min={0}
              max={100}
              value={watch("done_ratio") ?? 0}
              onChange={(event) => setValue("done_ratio", Number(event.target.value), { shouldDirty: true })}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="notes">Notes</Label>
            <Textarea id="notes" rows={3} {...register("notes")} />
          </div>
        </>
      )}

      <div className="flex items-center justify-between rounded-md border p-3">
        <div>
          <p className="font-medium">Private issue</p>
          <p className="text-sm text-muted-foreground">Visible only to members with access</p>
        </div>
        <Switch
          checked={Boolean(watch("is_private"))}
          onCheckedChange={(checked) => setValue("is_private", checked, { shouldDirty: true })}
        />
      </div>

      <div className="flex items-center gap-3">
        <Button type="submit" disabled={saving}>
          {saving ? "Saving..." : isEdit ? "Update Issue" : "Create Issue"}
        </Button>
        <Button type="button" variant="outline" onClick={() => router.back()} disabled={saving}>
          Cancel
        </Button>
      </div>
    </form>
  );
}
