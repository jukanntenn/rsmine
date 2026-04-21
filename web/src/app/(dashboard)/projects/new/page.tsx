"use client";

import { ProjectForm } from "@/components/features/project";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator } from "@/components/layout";

export default function NewProjectPage() {
  return (
    <div className="space-y-6">
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink href="/projects">Projects</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink>New Project</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      <div className="space-y-1">
        <h1 className="text-2xl font-bold">Create Project</h1>
        <p className="text-sm text-muted-foreground">Create a new project and configure its visibility and hierarchy.</p>
      </div>

      <ProjectForm />
    </div>
  );
}
