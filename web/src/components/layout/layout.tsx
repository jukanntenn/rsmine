"use client";

import * as React from "react";
import { cn } from "@/lib/utils";
import { Header } from "./header";
import { Sidebar } from "./sidebar";

interface LayoutProps {
  children: React.ReactNode;
  /**
   * If true, renders without sidebar (for auth pages)
   */
  noSidebar?: boolean;
  /**
   * If true, renders without header (for embedded views)
   */
  noHeader?: boolean;
  /**
   * Additional class names for the main content area
   */
  className?: string;
}

export function Layout({
  children,
  noSidebar = false,
  noHeader = false,
  className,
}: LayoutProps) {
  return (
    <div className="min-h-screen flex flex-col">
      {!noHeader && <Header />}
      <div className="flex-1 flex">
        {!noSidebar && <Sidebar />}
        <main
          className={cn(
            "flex-1 overflow-auto",
            !noSidebar && "p-6",
            className
          )}
        >
          {children}
        </main>
      </div>
    </div>
  );
}
