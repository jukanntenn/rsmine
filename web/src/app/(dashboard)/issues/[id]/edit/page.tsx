"use client";

import { useParams } from "next/navigation";
import { IssueForm } from "@/components/features/issue";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator } from "@/components/layout";

export default function EditIssuePage() {
  const params = useParams();
  const issueId = Number(params.id);

  return (
    <div className="space-y-6">
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink href="/issues">Issues</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink href={`/issues/${issueId}`}>Issue #{issueId}</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink>Edit</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      <div className="space-y-1">
        <h1 className="text-2xl font-bold">Edit Issue</h1>
        <p className="text-sm text-muted-foreground">Update issue attributes, workflow state, and notes.</p>
      </div>

      <IssueForm issueId={issueId} />
    </div>
  );
}
