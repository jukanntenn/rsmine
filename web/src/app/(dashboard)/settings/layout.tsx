"use client";

import type { ReactNode } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { AuthGuard } from "@/components/features/auth/auth-guard";
import { cn } from "@/lib/utils";

const settingNav = [
  { href: "/settings", label: "Overview" },
  { href: "/settings/trackers", label: "Trackers" },
  { href: "/settings/statuses", label: "Issue Statuses" },
  { href: "/settings/priorities", label: "Priorities" },
  { href: "/settings/roles", label: "Roles" },
];

export default function SettingsLayout({ children }: { children: ReactNode }) {
  const pathname = usePathname();

  return (
    <AuthGuard requireAdmin>
      <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">System Settings</h1>
        <p className="text-sm text-muted-foreground">Manage workflow enums and permissions.</p>
      </div>

      <div className="flex flex-wrap gap-2">
        {settingNav.map((item) => (
          <Link
            key={item.href}
            href={item.href}
            className={cn(
              "rounded-md border px-3 py-1.5 text-sm",
              pathname === item.href ? "bg-primary text-primary-foreground border-primary" : "hover:bg-muted"
            )}
          >
            {item.label}
          </Link>
        ))}
      </div>

        {children}
      </div>
    </AuthGuard>
  );
}
