"use client";

import * as React from "react";
import Link from "next/link";
import { useQuery } from "@tanstack/react-query";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import {
  Ticket,
  Search,
  Filter,
  ChevronRight,
} from "lucide-react";

dayjs.extend(relativeTime);
import { issuesApi } from "@/lib/api";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import type { IssueDetail } from "@/types";

interface IssueListProps {
  projectId?: number;
  className?: string;
}

function IssueStatusIndicator({ status }: { status?: { name: string; is_closed: boolean } }) {
  if (!status) return null;

  return (
    <Badge
      variant={status.is_closed ? "secondary" : "default"}
      className="font-normal"
    >
      {status.name}
    </Badge>
  );
}

function IssueRow({ issue }: { issue: IssueDetail }) {
  const createdAt = issue.created_on
    ? dayjs(issue.created_on).fromNow()
    : "";

  return (
    <TableRow className="group">
      <TableCell className="font-mono text-sm text-muted-foreground w-20">
        #{issue.id}
      </TableCell>
      <TableCell>
        <Link
          href={`/issues/${issue.id}`}
          className="font-medium hover:text-primary transition-colors line-clamp-1"
        >
          {issue.subject}
        </Link>
      </TableCell>
      <TableCell>
        <IssueStatusIndicator status={issue.status} />
      </TableCell>
      <TableCell>
        {issue.priority && (
          <Badge variant="outline" className="font-normal">
            {issue.priority.name}
          </Badge>
        )}
      </TableCell>
      <TableCell className="text-sm text-muted-foreground">
        {issue.assigned_to?.name || "-"}
      </TableCell>
      <TableCell className="text-sm text-muted-foreground">
        {createdAt}
      </TableCell>
      <TableCell className="w-8">
        <ChevronRight className="h-4 w-4 text-muted-foreground opacity-0 group-hover:opacity-100 transition-opacity" />
      </TableCell>
    </TableRow>
  );
}

function IssueRowSkeleton() {
  return (
    <TableRow>
      <TableCell>
        <Skeleton className="h-4 w-10" />
      </TableCell>
      <TableCell>
        <Skeleton className="h-4 w-48" />
      </TableCell>
      <TableCell>
        <Skeleton className="h-5 w-16" />
      </TableCell>
      <TableCell>
        <Skeleton className="h-5 w-14" />
      </TableCell>
      <TableCell>
        <Skeleton className="h-4 w-20" />
      </TableCell>
      <TableCell>
        <Skeleton className="h-4 w-24" />
      </TableCell>
      <TableCell>
        <Skeleton className="h-4 w-4" />
      </TableCell>
    </TableRow>
  );
}

export function IssueList({ projectId, className }: IssueListProps) {
  const [searchQuery, setSearchQuery] = React.useState("");
  const [debouncedSearch, setDebouncedSearch] = React.useState("");
  const [page, setPage] = React.useState(0);
  const limit = 20;

  // Debounce search
  React.useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(searchQuery);
      setPage(0);
    }, 300);
    return () => clearTimeout(timer);
  }, [searchQuery]);

  const { data, isLoading, isError, error, refetch } = useQuery({
    queryKey: ["issues", { project_id: projectId, offset: page * limit, limit }],
    queryFn: () =>
      issuesApi.list({
        project_id: projectId,
        offset: page * limit,
        limit,
      }),
  });

  const totalCount = data?.total_count ?? 0;
  const totalPages = Math.ceil(totalCount / limit);

  // Filter issues client-side for search
  const filteredIssues = React.useMemo(() => {
    const allIssues = data?.issues ?? [];
    if (!debouncedSearch) return allIssues;
    const query = debouncedSearch.toLowerCase();
    return allIssues.filter(
      (issue) =>
        issue.subject.toLowerCase().includes(query) ||
        String(issue.id).includes(query)
    );
  }, [data?.issues, debouncedSearch]);

  if (isError) {
    return (
      <EmptyState
        icon={<Ticket className="h-10 w-10 text-muted-foreground" />}
        title="Failed to load issues"
        description={
          error instanceof Error ? error.message : "An error occurred while loading issues"
        }
        action={
          <Button onClick={() => refetch()} variant="outline">
            Try Again
          </Button>
        }
      />
    );
  }

  return (
    <div className={cn("space-y-4", className)}>
      {/* Search and Filter Bar */}
      <div className="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between">
        <div className="relative w-full sm:w-80">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Search issues..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-9"
          />
        </div>
        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm">
            <Filter className="h-4 w-4 mr-2" />
            Filters
          </Button>
        </div>
      </div>

      {/* Issues Table */}
      <div className="rounded-lg border bg-card">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-20">ID</TableHead>
              <TableHead>Subject</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Priority</TableHead>
              <TableHead>Assignee</TableHead>
              <TableHead>Created</TableHead>
              <TableHead className="w-8"></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <IssueRowSkeleton key={i} />
              ))
            ) : filteredIssues.length === 0 ? (
              <TableRow>
                <TableCell colSpan={7} className="text-center py-8">
                  <EmptyState
                    icon={<Ticket className="h-8 w-8 text-muted-foreground mx-auto" />}
                    title="No issues found"
                    description={
                      searchQuery
                        ? "Try adjusting your search terms"
                        : "Create your first issue to get started"
                    }
                    className="py-0"
                  />
                </TableCell>
              </TableRow>
            ) : (
              filteredIssues.map((issue) => (
                <IssueRow key={issue.id} issue={issue} />
              ))
            )}
          </TableBody>
        </Table>
      </div>

      {/* Pagination */}
      {!isLoading && totalPages > 1 && (
        <div className="flex items-center justify-between pt-4">
          <p className="text-sm text-muted-foreground">
            Showing {page * limit + 1} - {Math.min((page + 1) * limit, totalCount)} of{" "}
            {totalCount} issues
          </p>
          <div className="flex items-center gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setPage((p) => Math.max(0, p - 1))}
              disabled={page === 0}
            >
              Previous
            </Button>
            <div className="flex items-center gap-1">
              {Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
                let pageNum: number;
                if (totalPages <= 5) {
                  pageNum = i;
                } else if (page < 2) {
                  pageNum = i;
                } else if (page > totalPages - 3) {
                  pageNum = totalPages - 5 + i;
                } else {
                  pageNum = page - 2 + i;
                }
                return (
                  <Button
                    key={pageNum}
                    variant={pageNum === page ? "default" : "outline"}
                    size="sm"
                    className="w-9"
                    onClick={() => setPage(pageNum)}
                  >
                    {pageNum + 1}
                  </Button>
                );
              })}
            </div>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setPage((p) => Math.min(totalPages - 1, p + 1))}
              disabled={page >= totalPages - 1}
            >
              Next
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
