"use client";

import Link from "next/link";
import dayjs from "dayjs";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import type { IssueDetail } from "@/types";

interface IssueSidebarProps {
  issue: IssueDetail;
}

export function IssueSidebar({ issue }: IssueSidebarProps) {
  return (
    <aside className="space-y-4">
      <div className="space-y-1">
        <p className="text-sm text-muted-foreground">Status</p>
        <Badge variant={issue.status?.is_closed ? "secondary" : "default"}>{issue.status?.name || "-"}</Badge>
      </div>

      <div className="space-y-1">
        <p className="text-sm text-muted-foreground">Priority</p>
        <Badge variant="outline">{issue.priority?.name || "-"}</Badge>
      </div>

      <div className="space-y-1">
        <p className="text-sm text-muted-foreground">Project</p>
        <Link href={`/projects/${issue.project.id}`} className="text-sm text-primary hover:underline">
          {issue.project.name || `#${issue.project.id}`}
        </Link>
      </div>

      <div className="space-y-2">
        <p className="text-sm text-muted-foreground">Assignee</p>
        {issue.assigned_to ? (
          <div className="flex items-center gap-2">
            <Avatar size="sm" fallback={issue.assigned_to.name} />
            <span className="text-sm">{issue.assigned_to.name}</span>
          </div>
        ) : (
          <p className="text-sm">Unassigned</p>
        )}
      </div>

      <div className="space-y-2">
        <p className="text-sm text-muted-foreground">Author</p>
        <div className="flex items-center gap-2">
          <Avatar size="sm" fallback={issue.author.name || ""} />
          <span className="text-sm">{issue.author.name || "-"}</span>
        </div>
      </div>

      <div className="space-y-2">
        <p className="text-sm text-muted-foreground">Done Ratio</p>
        <div className="space-y-1">
          <Progress value={issue.done_ratio || 0} />
          <p className="text-xs text-muted-foreground">{issue.done_ratio || 0}%</p>
        </div>
      </div>

      <div className="space-y-1 text-sm">
        <p className="text-muted-foreground">Dates</p>
        <p>Start: {issue.start_date ? dayjs(issue.start_date).format("YYYY-MM-DD") : "-"}</p>
        <p>Due: {issue.due_date ? dayjs(issue.due_date).format("YYYY-MM-DD") : "-"}</p>
      </div>
    </aside>
  );
}
