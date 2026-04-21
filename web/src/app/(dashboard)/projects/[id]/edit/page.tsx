"use client";

import { useParams } from "next/navigation";
import { ProjectForm } from "@/components/features/project";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator } from "@/components/layout";

export default function EditProjectPage() {
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
          <BreadcrumbLink href={`/projects/${projectId}`}>Project</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink>Edit</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      <div className="space-y-1">
        <h1 className="text-2xl font-bold">Edit Project</h1>
        <p className="text-sm text-muted-foreground">Update project metadata and hierarchy.</p>
      </div>

      <ProjectForm projectId={projectId} />
    </div>
  );
}
