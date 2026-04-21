"use client";

import * as React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import type { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";
import { useSidebarStore } from "@/stores";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";

interface NavItemProps {
  title: string;
  href: string;
  icon: LucideIcon;
  badge?: number;
}

export function NavItem({ title, href, icon: Icon, badge }: NavItemProps) {
  const pathname = usePathname();
  const { collapsed } = useSidebarStore();
  
  const isActive = pathname === href || pathname.startsWith(`${href}/`);
  
  const linkContent = (
    <Link
      href={href}
      className={cn(
        "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-all hover:bg-accent hover:text-accent-foreground",
        isActive
          ? "bg-accent text-accent-foreground"
          : "text-muted-foreground",
        collapsed && "justify-center px-2"
      )}
    >
      <Icon className="h-4 w-4 shrink-0" />
      {!collapsed && <span>{title}</span>}
      {!collapsed && badge !== undefined && badge > 0 && (
        <span className="ml-auto flex h-5 min-w-5 items-center justify-center rounded-full bg-primary px-1.5 text-xs font-medium text-primary-foreground">
          {badge > 99 ? "99+" : badge}
        </span>
      )}
    </Link>
  );

  if (collapsed) {
    return (
      <Tooltip>
        <TooltipTrigger>
          {linkContent}
        </TooltipTrigger>
        <TooltipContent side="right">
          {title}
          {badge !== undefined && badge > 0 && (
            <span className="ml-2 text-muted-foreground">({badge})</span>
          )}
        </TooltipContent>
      </Tooltip>
    );
  }

  return linkContent;
}
