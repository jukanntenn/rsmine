"use client";

import Link from "next/link";
import { Link2 } from "lucide-react";
import type { IssueRelation } from "@/types";

interface IssueRelationsProps {
  relations?: IssueRelation[];
}

export function IssueRelations({ relations }: IssueRelationsProps) {
  if (!relations?.length) {
    return null;
  }

  return (
    <section className="space-y-2">
      <h2 className="text-lg font-semibold">Relations</h2>
      <div className="space-y-1">
        {relations.map((relation) => (
          <div key={relation.id} className="flex items-center gap-2 text-sm">
            <Link2 className="h-4 w-4 text-muted-foreground" />
            <Link href={`/issues/${relation.issue_to_id}`} className="text-primary hover:underline">
              #{relation.issue_to_id}
            </Link>
            <span className="text-muted-foreground">{relation.relation_type}</span>
          </div>
        ))}
      </div>
    </section>
  );
}
