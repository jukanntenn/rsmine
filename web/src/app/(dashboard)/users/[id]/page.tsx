"use client";

import Link from "next/link";
import { useQuery } from "@tanstack/react-query";
import { useParams } from "next/navigation";
import { UserRound } from "lucide-react";
import { usersApi } from "@/lib/api";
import { AuthGuard } from "@/components/features/auth/auth-guard";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator } from "@/components/layout";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";

export default function UserDetailPage() {
  const params = useParams();
  const userId = Number(params.id);

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["user", userId],
    queryFn: () => usersApi.get(userId),
  });

  return (
    <AuthGuard requireAdmin>
      <div className="space-y-6">
        <Breadcrumb>
          <BreadcrumbItem>
            <BreadcrumbLink href="/users">Users</BreadcrumbLink>
          </BreadcrumbItem>
          <BreadcrumbSeparator />
          <BreadcrumbItem>
            <BreadcrumbLink>User #{userId}</BreadcrumbLink>
          </BreadcrumbItem>
        </Breadcrumb>

        {isLoading ? (
          <Skeleton className="h-56 w-full" />
        ) : isError || !data?.user ? (
          <EmptyState
            icon={<UserRound className="h-10 w-10 text-muted-foreground" />}
            title="User not found"
            description={error instanceof Error ? error.message : "The user does not exist or is not visible."}
          />
        ) : (
          <Card>
            <CardHeader>
              <CardTitle>{data.user.firstname} {data.user.lastname}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="text-sm">Login: {data.user.login}</div>
              <div className="text-sm">Email: {data.user.mail}</div>
              <div className="text-sm">Status: {data.user.status === 1 ? "Active" : data.user.status === 2 ? "Registered" : "Locked"}</div>
              <div className="text-sm">Admin: {data.user.admin ? "Yes" : "No"}</div>
              <Button asChild className="mt-2">
                <Link href={`/users/${userId}/edit`}>Edit User</Link>
              </Button>
            </CardContent>
          </Card>
        )}
      </div>
    </AuthGuard>
  );
}
