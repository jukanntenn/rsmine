"use client";

import { Download, Paperclip } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { Attachment } from "@/types";

interface IssueAttachmentsProps {
  attachments?: Attachment[];
}

function formatFileSize(bytes: number) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export function IssueAttachments({ attachments }: IssueAttachmentsProps) {
  if (!attachments?.length) {
    return null;
  }

  return (
    <section className="space-y-2">
      <h2 className="text-lg font-semibold">Attachments</h2>
      <div className="space-y-2">
        {attachments.map((attachment) => (
          <div key={attachment.id} className="flex items-center justify-between rounded-md border p-2">
            <div className="flex items-center gap-2 text-sm">
              <Paperclip className="h-4 w-4 text-muted-foreground" />
              <span>{attachment.filename}</span>
              <span className="text-muted-foreground">({formatFileSize(attachment.filesize)})</span>
            </div>
            <Button size="icon" variant="ghost" asChild>
              <a href={attachment.content_url || `/api/v1/attachments/download/${attachment.id}`} download>
                <Download className="h-4 w-4" />
              </a>
            </Button>
          </div>
        ))}
      </div>
    </section>
  );
}
