"use client";

import { useParams } from "next/navigation";
import { AuthGuard } from "@/components/features/auth/auth-guard";
import { UserForm } from "@/components/features/user/user-form";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator } from "@/components/layout";

export default function EditUserPage() {
  const params = useParams();
  const userId = Number(params.id);

  return (
    <AuthGuard requireAdmin>
      <div className="space-y-6">
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink href="/users">Users</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink href={`/users/${userId}/edit`}>User #{userId}</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
          <BreadcrumbLink>Edit</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      <div className="space-y-1">
        <h1 className="text-2xl font-bold">Edit User</h1>
        <p className="text-sm text-muted-foreground">Update profile data, status, and admin privileges.</p>
      </div>

        <UserForm userId={userId} />
      </div>
    </AuthGuard>
  );
}
