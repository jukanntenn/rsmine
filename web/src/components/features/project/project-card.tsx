"use client";

import * as React from "react";
import Link from "next/link";
import { FolderKanban, Globe, Lock, ExternalLink } from "lucide-react";
import { cn } from "@/lib/utils";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { ProjectStatusBadge } from "./project-status-badge";
import type { ProjectListItem } from "@/types";

interface ProjectCardProps {
  project: ProjectListItem;
  className?: string;
}

export function ProjectCard({ project, className }: ProjectCardProps) {
  return (
    <Link href={`/projects/${project.id}`}>
      <Card
        className={cn(
          "group cursor-pointer transition-all duration-200 hover:shadow-md hover:border-primary/50",
          className
        )}
      >
        <CardContent className="p-5">
          <div className="flex items-start justify-between gap-4">
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2 mb-2">
                <FolderKanban className="h-5 w-5 text-primary shrink-0" />
                <h3 className="font-semibold text-base truncate group-hover:text-primary transition-colors">
                  {project.name}
                </h3>
              </div>

              <div className="flex items-center gap-2 text-sm text-muted-foreground mb-3">
                <code className="px-1.5 py-0.5 rounded bg-muted text-xs font-mono">
                  {project.identifier}
                </code>
                {project.is_public ? (
                  <Badge variant="outline" className="gap-1 text-xs">
                    <Globe className="h-3 w-3" />
                    Public
                  </Badge>
                ) : (
                  <Badge variant="outline" className="gap-1 text-xs">
                    <Lock className="h-3 w-3" />
                    Private
                  </Badge>
                )}
                <ProjectStatusBadge status={project.status} />
              </div>

              {project.description && (
                <p className="text-sm text-muted-foreground line-clamp-2 mb-3">
                  {project.description}
                </p>
              )}

              <div className="flex items-center gap-4 text-xs text-muted-foreground">
                {project.parent && (
                  <span className="flex items-center gap-1">
                    Parent: {project.parent.name}
                  </span>
                )}
                {project.issue_count !== undefined && project.issue_count > 0 && (
                  <span>{project.issue_count} issues</span>
                )}
              </div>
            </div>

            <ExternalLink className="h-4 w-4 text-muted-foreground opacity-0 group-hover:opacity-100 transition-opacity shrink-0" />
          </div>
        </CardContent>
      </Card>
    </Link>
  );
}
