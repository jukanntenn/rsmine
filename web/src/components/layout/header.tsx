"use client";

import * as React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { LayoutDashboard, Menu, PanelLeft, X } from "lucide-react";
import { cn } from "@/lib/utils";
import { useSidebarStore, useAuthStore } from "@/stores";
import { Button } from "@/components/ui/button";
import { ThemeToggle } from "./theme-toggle";
import { UserNav } from "./user-nav";

const mainNavItems = [
  {
    title: "Projects",
    href: "/projects",
  },
  {
    title: "Issues",
    href: "/issues",
  },
  {
    title: "Users",
    href: "/users",
    adminOnly: true,
  },
];

export function Header() {
  const pathname = usePathname();
  const { collapsed, toggle } = useSidebarStore();
  const { user } = useAuthStore();
  const [mobileMenuOpen, setMobileMenuOpen] = React.useState(false);

  const isAdmin = user?.admin ?? false;

  const filteredNavItems = mainNavItems.filter(
    (item) => !item.adminOnly || isAdmin
  );

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="flex h-14 items-center px-4">
        {/* Mobile menu button */}
        <Button
          variant="ghost"
          size="icon"
          className="mr-2 md:hidden"
          onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
          aria-label="Toggle mobile menu"
        >
          {mobileMenuOpen ? (
            <X className="h-5 w-5" />
          ) : (
            <Menu className="h-5 w-5" />
          )}
        </Button>

        {/* Sidebar toggle */}
        <Button
          variant="ghost"
          size="icon"
          className="mr-2 hidden md:inline-flex"
          onClick={toggle}
          aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
        >
          <PanelLeft
            className={cn(
              "h-5 w-5 transition-transform",
              collapsed && "rotate-180"
            )}
          />
        </Button>

        {/* Logo */}
        <Link
          href="/"
          className="mr-6 flex items-center gap-2 font-semibold text-lg"
        >
          <LayoutDashboard className="h-6 w-6 text-primary" />
          <span className="hidden sm:inline-block">Rsmine</span>
        </Link>

        {/* Desktop Navigation */}
        <nav className="hidden md:flex items-center gap-1 text-sm font-medium">
          {filteredNavItems.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              className={cn(
                "px-3 py-2 rounded-md transition-colors hover:bg-accent hover:text-accent-foreground",
                pathname.startsWith(item.href)
                  ? "bg-accent text-accent-foreground"
                  : "text-muted-foreground"
              )}
            >
              {item.title}
            </Link>
          ))}
        </nav>

        {/* Right side actions */}
        <div className="ml-auto flex items-center gap-2">
          <ThemeToggle />
          <UserNav />
        </div>
      </div>

      {/* Mobile Navigation */}
      {mobileMenuOpen && (
        <nav className="md:hidden border-t bg-background">
          <div className="container py-4 flex flex-col gap-1">
            {filteredNavItems.map((item) => (
              <Link
                key={item.href}
                href={item.href}
                onClick={() => setMobileMenuOpen(false)}
                className={cn(
                  "px-3 py-2 rounded-md transition-colors hover:bg-accent hover:text-accent-foreground",
                  pathname.startsWith(item.href)
                    ? "bg-accent text-accent-foreground"
                    : "text-muted-foreground"
                )}
              >
                {item.title}
              </Link>
            ))}
          </div>
        </nav>
      )}
    </header>
  );
}
