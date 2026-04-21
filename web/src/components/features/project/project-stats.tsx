"use client";

import * as React from "react";
import { Ticket, CheckCircle, Clock, AlertCircle } from "lucide-react";
import { cn } from "@/lib/utils";
import { Card, CardContent } from "@/components/ui/card";

interface ProjectStatsProps {
  stats: {
    total: number;
    open: number;
    closed: number;
    in_progress: number;
  };
  className?: string;
}

interface StatCardProps {
  icon: React.ReactNode;
  value: number;
  label: string;
  iconClassName?: string;
}

function StatCard({ icon, value, label, iconClassName }: StatCardProps) {
  return (
    <Card>
      <CardContent className="flex items-center gap-4 pt-6">
        <div className={cn("flex h-12 w-12 items-center justify-center rounded-lg", iconClassName)}>
          {icon}
        </div>
        <div>
          <div className="text-2xl font-bold">{value}</div>
          <div className="text-sm text-muted-foreground">{label}</div>
        </div>
      </CardContent>
    </Card>
  );
}

export function ProjectStats({ stats, className }: ProjectStatsProps) {
  return (
    <div className={cn("grid gap-4 sm:grid-cols-2 lg:grid-cols-4", className)}>
      <StatCard
        icon={<Ticket className="h-6 w-6 text-muted-foreground" />}
        value={stats.total}
        label="Total Issues"
        iconClassName="bg-muted"
      />
      <StatCard
        icon={<AlertCircle className="h-6 w-6 text-blue-500" />}
        value={stats.open}
        label="Open"
        iconClassName="bg-blue-500/10"
      />
      <StatCard
        icon={<Clock className="h-6 w-6 text-amber-500" />}
        value={stats.in_progress}
        label="In Progress"
        iconClassName="bg-amber-500/10"
      />
      <StatCard
        icon={<CheckCircle className="h-6 w-6 text-green-500" />}
        value={stats.closed}
        label="Closed"
        iconClassName="bg-green-500/10"
      />
    </div>
  );
}
