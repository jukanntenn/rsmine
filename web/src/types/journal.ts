import type { NamedEntity } from "./api";

/**
 * Attachment type
 */
export interface Attachment {
  id: number;
  container_type?: string | null;
  container_id?: number | null;
  filename: string;
  filesize: number;
  content_type: string;
  description: string | null;
  content_url?: string;
  thumbnail_url?: string;
  author?: NamedEntity;
  created_on: string;
  downloads?: number;
  digest?: string;
  disk_filename?: string;
  author_id?: number;
}

/**
 * Journal detail (field changes in API response)
 */
export interface JournalDetail {
  property: "attr" | "cf" | "attachment";
  name: string;
  old_value: string | null;
  new_value: string | null;
}

/**
 * Journal (issue notes/history) type
 */
export interface Journal {
  id: number;
  user?: NamedEntity;
  notes: string | null;
  created_on: string;
  updated_on?: string;
  private_notes: boolean;
  details: JournalDetail[];
}
