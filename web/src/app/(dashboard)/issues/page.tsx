"use client";

import { IssueList } from "@/components/features/issue";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink } from "@/components/layout";

export default function IssuesPage() {
  return (
    <div className="space-y-6">
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink>Issues</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>
      <div className="space-y-1">
        <h1 className="text-2xl font-bold">All Issues</h1>
        <p className="text-sm text-muted-foreground">Browse and track issues across all visible projects.</p>
      </div>
      <IssueList projectId={0} />
    </div>
  );
}
