"use client";

import dayjs from "dayjs";
import { Avatar } from "@/components/ui/avatar";
import { Card, CardContent } from "@/components/ui/card";
import type { Journal } from "@/types";

interface IssueJournalsProps {
  journals?: Journal[];
}

export function IssueJournals({ journals }: IssueJournalsProps) {
  if (!journals?.length) {
    return null;
  }

  return (
    <section className="space-y-3">
      <h2 className="text-lg font-semibold">History</h2>
      {journals.map((journal) => (
        <Card key={journal.id}>
          <CardContent className="pt-4">
            <div className="flex items-start gap-3">
              <Avatar size="sm" fallback={journal.user?.name || "U"} />
              <div className="flex-1 space-y-2">
                <p className="text-sm text-muted-foreground">
                  <span className="font-medium text-foreground">{journal.user?.name || "Unknown"}</span>
                  {" · "}
                  {dayjs(journal.created_on).format("YYYY-MM-DD HH:mm")}
                </p>
                {journal.details?.map((detail, index) => (
                  <p key={`${journal.id}-${index}`} className="text-sm text-muted-foreground">
                    {detail.name}: {detail.old_value || "empty"} → {detail.new_value || "empty"}
                  </p>
                ))}
                {journal.notes && <p className="text-sm whitespace-pre-wrap">{journal.notes}</p>}
              </div>
            </div>
          </CardContent>
        </Card>
      ))}
    </section>
  );
}
