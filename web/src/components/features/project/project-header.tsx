"use client";

import * as React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { Settings, Users, Ticket, Globe, Lock } from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { ProjectStatusBadge } from "./project-status-badge";
import type { Project } from "@/types";

interface ProjectHeaderProps {
  project: Project;
  className?: string;
}

const tabs = [
  { value: "issues", label: "Issues", href: "", icon: Ticket },
  { value: "members", label: "Members", href: "/members", icon: Users },
  { value: "settings", label: "Settings", href: "/settings", icon: Settings },
] as const;

export function ProjectHeader({ project, className }: ProjectHeaderProps) {
  const pathname = usePathname();
  const projectBasePath = `/projects/${project.id}`;

  // Determine active tab based on current path
  const getActiveTab = () => {
    if (pathname === projectBasePath || pathname.startsWith(`${projectBasePath}/issues`)) {
      return "issues";
    }
    if (pathname.startsWith(`${projectBasePath}/members`)) {
      return "members";
    }
    if (pathname.startsWith(`${projectBasePath}/settings`)) {
      return "settings";
    }
    return "issues";
  };

  const activeTab = getActiveTab();

  return (
    <div className={cn("space-y-6", className)}>
      {/* Project Title and Actions */}
      <div className="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
        <div className="space-y-1">
          <div className="flex items-center gap-3">
            <h1 className="text-2xl font-bold tracking-tight sm:text-3xl">
              {project.name}
            </h1>
            <div className="flex items-center gap-2">
              {project.is_public ? (
                <Badge variant="secondary" className="gap-1">
                  <Globe className="h-3 w-3" />
                  Public
                </Badge>
              ) : (
                <Badge variant="outline" className="gap-1">
                  <Lock className="h-3 w-3" />
                  Private
                </Badge>
              )}
              <ProjectStatusBadge status={project.status} />
            </div>
          </div>
          <div className="flex items-center gap-2 text-sm text-muted-foreground">
            <code className="px-1.5 py-0.5 rounded bg-muted text-xs font-mono">
              {project.identifier}
            </code>
            {project.description && (
              <>
                <span className="text-muted-foreground/50">·</span>
                <span className="line-clamp-1">{project.description}</span>
              </>
            )}
          </div>
        </div>
        <Button asChild>
          <Link href={`/projects/${project.id}/issues/new`}>
            <Ticket className="mr-2 h-4 w-4" />
            New Issue
          </Link>
        </Button>
      </div>

      {/* Tabs Navigation */}
      <div className="border-b">
        <nav className="flex gap-1" aria-label="Project sections">
          {tabs.map((tab) => {
            const isActive = activeTab === tab.value;
            const href = tab.href ? `${projectBasePath}${tab.href}` : projectBasePath;
            const Icon = tab.icon;

            return (
              <Link
                key={tab.value}
                href={href}
                className={cn(
                  "flex items-center gap-2 px-4 py-2 text-sm font-medium border-b-2 transition-colors",
                  isActive
                    ? "border-primary text-foreground"
                    : "border-transparent text-muted-foreground hover:text-foreground hover:border-muted-foreground/30"
                )}
                aria-current={isActive ? "page" : undefined}
              >
                <Icon className="h-4 w-4" />
                {tab.label}
              </Link>
            );
          })}
        </nav>
      </div>
    </div>
  );
}
