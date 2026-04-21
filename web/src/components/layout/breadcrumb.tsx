"use client";

import * as React from "react";
import Link from "next/link";
import { ChevronRight, Home } from "lucide-react";
import { cn } from "@/lib/utils";

interface BreadcrumbContextValue {
  items: BreadcrumbItemData[];
  addItem: (item: BreadcrumbItemData) => void;
  removeItem: (id: string) => void;
}

interface BreadcrumbItemData {
  id: string;
  label: string;
  href?: string;
}

const BreadcrumbContext = React.createContext<BreadcrumbContextValue | undefined>(undefined);

interface BreadcrumbProps {
  children?: React.ReactNode;
  className?: string;
  showHome?: boolean;
}

function Breadcrumb({ children, className, showHome = true }: BreadcrumbProps) {
  const [items, setItems] = React.useState<BreadcrumbItemData[]>([]);

  const addItem = React.useCallback((item: BreadcrumbItemData) => {
    setItems((prev) => {
      const exists = prev.some((i) => i.id === item.id);
      if (exists) return prev;
      return [...prev, item];
    });
  }, []);

  const removeItem = React.useCallback((id: string) => {
    setItems((prev) => prev.filter((item) => item.id !== id));
  }, []);

  return (
    <BreadcrumbContext.Provider value={{ items, addItem, removeItem }}>
      <nav aria-label="breadcrumb" className={cn("flex items-center text-sm", className)}>
        {showHome && (
          <>
            <Link
              href="/"
              className="flex items-center text-muted-foreground hover:text-foreground transition-colors"
            >
              <Home className="h-4 w-4" />
            </Link>
            {(items.length > 0 || children) && (
              <ChevronRight className="h-4 w-4 mx-1 text-muted-foreground" />
            )}
          </>
        )}
        <ol className="flex items-center gap-1">
          {items.map((item, index) => (
            <React.Fragment key={item.id}>
              <li className="flex items-center">
                {item.href ? (
                  <Link
                    href={item.href}
                    className="text-muted-foreground hover:text-foreground transition-colors"
                  >
                    {item.label}
                  </Link>
                ) : (
                  <span className="text-foreground font-medium">{item.label}</span>
                )}
              </li>
              {index < items.length - 1 && (
                <ChevronRight className="h-4 w-4 text-muted-foreground" />
              )}
            </React.Fragment>
          ))}
          {children}
        </ol>
      </nav>
    </BreadcrumbContext.Provider>
  );
}

interface BreadcrumbItemProps {
  children: React.ReactNode;
  className?: string;
}

function BreadcrumbItem({ children, className }: BreadcrumbItemProps) {
  return (
    <li className={cn("flex items-center", className)}>
      {children}
    </li>
  );
}

interface BreadcrumbLinkProps {
  href?: string;
  children: React.ReactNode;
  className?: string;
}

function BreadcrumbLink({ href, children, className }: BreadcrumbLinkProps) {
  if (href) {
    return (
      <Link
        href={href}
        className={cn(
          "text-muted-foreground hover:text-foreground transition-colors",
          className
        )}
      >
        {children}
      </Link>
    );
  }
  return (
    <span className={cn("text-foreground font-medium", className)}>
      {children}
    </span>
  );
}

function BreadcrumbSeparator({ className }: { className?: string }) {
  return (
    <ChevronRight className={cn("h-4 w-4 text-muted-foreground", className)} />
  );
}

export { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator };
