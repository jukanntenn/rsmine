"use client";

import { Badge } from "@/components/ui/badge";
import type { ProjectStatus } from "@/types";

interface ProjectStatusBadgeProps {
  status: ProjectStatus;
  className?: string;
}

const statusConfig: Record<
  ProjectStatus,
  { label: string; variant: "success" | "warning" | "secondary" }
> = {
  1: { label: "Active", variant: "success" },
  5: { label: "Closed", variant: "warning" },
  9: { label: "Archived", variant: "secondary" },
};

export function ProjectStatusBadge({
  status,
  className,
}: ProjectStatusBadgeProps) {
  const config = statusConfig[status];

  return (
    <Badge variant={config.variant} className={className}>
      {config.label}
    </Badge>
  );
}
