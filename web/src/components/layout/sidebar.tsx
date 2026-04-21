"use client";

import * as React from "react";
import Link from "next/link";
import {
  FolderKanban,
  SquareCheck,
  Users,
  ShieldCheck,
  ListChecks,
  Flag,
  Tag,
} from "lucide-react";
import { cn } from "@/lib/utils";
import { useSidebarStore, useAuthStore } from "@/stores";
import { Separator } from "@/components/ui/separator";
import { NavItem } from "./nav-item";

const mainNavItems = [
  {
    title: "Projects",
    href: "/projects",
    icon: FolderKanban,
  },
  {
    title: "Issues",
    href: "/issues",
    icon: SquareCheck,
  },
];

const adminNavItems = [
  {
    title: "Users",
    href: "/users",
    icon: Users,
  },
];

const settingsNavItems = [
  {
    title: "Roles",
    href: "/settings/roles",
    icon: ShieldCheck,
  },
  {
    title: "Trackers",
    href: "/settings/trackers",
    icon: ListChecks,
  },
  {
    title: "Statuses",
    href: "/settings/statuses",
    icon: Tag,
  },
  {
    title: "Priorities",
    href: "/settings/priorities",
    icon: Flag,
  },
];

export function Sidebar() {
  const { collapsed } = useSidebarStore();
  const { user } = useAuthStore();
  const isAdmin = user?.admin ?? false;

  return (
    <aside
      className={cn(
        "hidden md:flex flex-col border-r bg-sidebar text-sidebar-foreground transition-all duration-300",
        collapsed ? "w-16" : "w-64"
      )}
    >
      <div className="flex-1 flex flex-col overflow-y-auto py-4">
        <nav className="flex flex-col gap-1 px-2">
          {/* Main Navigation */}
          <div className="flex flex-col gap-1">
            {mainNavItems.map((item) => (
              <NavItem
                key={item.href}
                title={item.title}
                href={item.href}
                icon={item.icon}
              />
            ))}
          </div>

          {/* Admin Section */}
          {isAdmin && (
            <>
              <Separator className="my-3" />
              <div className="flex flex-col gap-1">
                {!collapsed && (
                  <span className="px-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
                    Administration
                  </span>
                )}
                {adminNavItems.map((item) => (
                  <NavItem
                    key={item.href}
                    title={item.title}
                    href={item.href}
                    icon={item.icon}
                  />
                ))}
              </div>
            </>
          )}

          {/* Settings Section */}
          {isAdmin && (
            <>
              <Separator className="my-3" />
              <div className="flex flex-col gap-1">
                {!collapsed && (
                  <Link
                    href="/settings"
                    className="px-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground hover:text-foreground transition-colors"
                  >
                    Settings
                  </Link>
                )}
                {settingsNavItems.map((item) => (
                  <NavItem
                    key={item.href}
                    title={item.title}
                    href={item.href}
                    icon={item.icon}
                  />
                ))}
              </div>
            </>
          )}
        </nav>
      </div>
    </aside>
  );
}
