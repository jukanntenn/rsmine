"use client";

import { useParams } from "next/navigation";
import { IssueForm } from "@/components/features/issue";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbSeparator,
} from "@/components/layout";

export default function NewProjectIssuePage() {
  const params = useParams();
  const projectId = Number(params.id);

  return (
    <div className="space-y-6">
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink href="/projects">Projects</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink href={`/projects/${projectId}`}>
            Project
          </BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink>New Issue</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      <div className="space-y-1">
        <h1 className="text-2xl font-bold">Create Issue</h1>
        <p className="text-sm text-muted-foreground">
          Add a new issue inside this project.
        </p>
      </div>

      <IssueForm projectId={projectId} />
    </div>
  );
}
