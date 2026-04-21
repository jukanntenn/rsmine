"use client";

import Link from "next/link";
import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { Plus, Users } from "lucide-react";
import { usersApi } from "@/lib/api";
import { AuthGuard } from "@/components/features/auth/auth-guard";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink } from "@/components/layout";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { EmptyState } from "@/components/ui/empty-state";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

export default function UsersPage() {
  const [search, setSearch] = useState("");
  const [status, setStatus] = useState<string>("all");

  const { data, isLoading, isError, error } = useQuery({
    queryKey: ["users", { search, status }],
    queryFn: () => usersApi.list({ name: search || undefined, status: status === "all" ? undefined : Number(status), limit: 100 }),
  });

  const users = data?.users ?? [];

  return (
    <AuthGuard requireAdmin>
      <div className="space-y-6">
      <Breadcrumb>
        <BreadcrumbItem>
          <BreadcrumbLink>Users</BreadcrumbLink>
        </BreadcrumbItem>
      </Breadcrumb>

      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-2xl font-bold">User Management</h1>
          <p className="text-sm text-muted-foreground">Manage system users and roles.</p>
        </div>
        <Button asChild>
          <Link href="/users/new">
            <Plus className="h-4 w-4 mr-2" />
            New User
          </Link>
        </Button>
      </div>

      <div className="grid gap-3 md:grid-cols-[1fr_200px]">
        <Input placeholder="Search by login or name" value={search} onChange={(event) => setSearch(event.target.value)} />
        <Select
          value={status}
          onValueChange={setStatus}
          options={[
            { value: "all", label: "All statuses" },
            { value: "1", label: "Active" },
            { value: "2", label: "Registered" },
            { value: "3", label: "Locked" },
          ]}
        />
      </div>

      {isLoading ? (
        <Skeleton className="h-52 w-full" />
      ) : isError ? (
        <EmptyState title="Failed to load users" description={error instanceof Error ? error.message : "Please refresh the page."} />
      ) : users.length === 0 ? (
        <EmptyState
          icon={<Users className="h-10 w-10 text-muted-foreground" />}
          title="No users found"
          description="Adjust filters or create a new user."
        />
      ) : (
        <div className="rounded-md border">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Login</TableHead>
                <TableHead>Name</TableHead>
                <TableHead>Email</TableHead>
                <TableHead>Status</TableHead>
                <TableHead className="text-right">Action</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {users.map((user) => (
                <TableRow key={user.id}>
                  <TableCell>{user.login}</TableCell>
                  <TableCell>{user.firstname} {user.lastname}</TableCell>
                  <TableCell>{user.mail}</TableCell>
                  <TableCell>{user.status === 1 ? "Active" : user.status === 2 ? "Registered" : "Locked"}</TableCell>
                  <TableCell className="text-right">
                    <Button variant="outline" size="sm" asChild>
                      <Link href={`/users/${user.id}/edit`}>Edit</Link>
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      )}
      </div>
    </AuthGuard>
  );
}
