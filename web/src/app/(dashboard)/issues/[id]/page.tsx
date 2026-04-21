"use client";

import { useQuery } from "@tanstack/react-query";
import { useParams } from "next/navigation";
import Link from "next/link";
import { Edit } from "lucide-react";
import { issuesApi } from "@/lib/api";
import { IssueAttachments, IssueJournals, IssueRelations, IssueSidebar } from "@/components/features/issue";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator } from "@/components/layout";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";

export default function IssueDetailPage() {
  const params = useParams();
  const issueId = Number(params.id);

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["issue", issueId, "detail"],
    queryFn: () => issuesApi.get(issueId, { include: "attachments,journals,relations,children" }),
  });

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-64" />
        <Skeleton className="h-28 w-full" />
      </div>
    );
  }

  if (isError || !data?.issue) {
    return (
      <EmptyState
        title="Issue not found"
        description={error instanceof Error ? error.message : "The issue does not exist or is not visible."}
      />
    );
  }

  const issue = data.issue;

  return (
    <div className="space-y-6">
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink href="/projects">Projects</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink href={`/projects/${issue.project.id}`}>{issue.project.name || "Project"}</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink>Issue #{issue.id}</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">#{issue.id}</h1>
          <p className="text-lg">{issue.subject}</p>
        </div>
        <Button asChild>
          <Link href={`/issues/${issue.id}/edit`}>
            <Edit className="h-4 w-4 mr-2" />
            Edit
          </Link>
        </Button>
      </div>

      <div className="grid gap-6 lg:grid-cols-[1fr_320px]">
        <div className="space-y-6">
          <section className="space-y-2">
            <h2 className="text-lg font-semibold">Description</h2>
            <div className="whitespace-pre-wrap rounded-md border p-3 text-sm">
              {issue.description || "No description provided."}
            </div>
          </section>

          <IssueAttachments attachments={issue.attachments} />
          <IssueRelations relations={issue.relations} />
          <IssueJournals journals={issue.journals} />
        </div>

        <div className="rounded-md border p-4">
          <IssueSidebar issue={issue} />
        </div>
      </div>
    </div>
  );
}
