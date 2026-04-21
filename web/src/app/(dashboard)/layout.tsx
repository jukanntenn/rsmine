import type { Metadata } from "next";
import { AuthGuard } from "@/components/features/auth";
import { Layout } from "@/components/layout";

export const metadata: Metadata = {
  title: "Dashboard - Rsmine",
  description: "Project management and issue tracking dashboard",
};

/**
 * Layout for dashboard pages (requires authentication)
 * Includes sidebar and header for navigation
 */
export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <AuthGuard>
      <Layout>{children}</Layout>
    </AuthGuard>
  );
}
