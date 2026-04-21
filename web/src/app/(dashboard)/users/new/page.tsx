"use client";

import { UserForm } from "@/components/features/user/user-form";
import { AuthGuard } from "@/components/features/auth/auth-guard";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator } from "@/components/layout";

export default function NewUserPage() {
  return (
    <AuthGuard requireAdmin>
      <div className="space-y-6">
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink href="/users">Users</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink>New User</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      <div className="space-y-1">
        <h1 className="text-2xl font-bold">Create User</h1>
        <p className="text-sm text-muted-foreground">Create a new account and assign privileges.</p>
      </div>

        <UserForm />
      </div>
    </AuthGuard>
  );
}
