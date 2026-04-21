import type { IssueCategory } from "./enumeration";
import type { NamedEntity } from "./api";

/**
 * Issue relation type
 */
export type IssueRelationType =
  | "relates"
  | "duplicates"
  | "duplicated"
  | "blocks"
  | "blocked"
  | "precedes"
  | "follows"
  | "copied_to"
  | "copied_from";

/**
 * Issue type
 */
export interface Issue {
  id: number;
  project_id: number;
  tracker_id: number;
  status_id: number;
  priority_id: number;
  category_id: number | null;
  author_id: number;
  assigned_to_id: number | null;
  parent_id: number | null;
  root_id: number;
  subject: string;
  description: string | null;
  start_date: string | null;
  due_date: string | null;
  done_ratio: number;
  estimated_hours: number | null;
  is_private: boolean;
  closed_on: string | null;
  lft: number;
  rgt: number;
  created_on: string;
  updated_on: string;
}

/**
 * Issue creation request
 */
export interface IssueCreateRequest {
  project_id: number;
  tracker_id: number;
  status_id?: number;
  priority_id?: number;
  category_id?: number;
  assigned_to_id?: number;
  parent_id?: number;
  subject: string;
  description?: string;
  start_date?: string;
  due_date?: string;
  done_ratio?: number;
  estimated_hours?: number;
  is_private?: boolean;
}

/**
 * Issue update request
 */
export interface IssueUpdateRequest {
  tracker_id?: number;
  status_id?: number;
  priority_id?: number;
  category_id?: number;
  assigned_to_id?: number;
  parent_id?: number;
  subject?: string;
  description?: string;
  start_date?: string;
  due_date?: string;
  done_ratio?: number;
  estimated_hours?: number;
  is_private?: boolean;
  notes?: string;
}

/**
 * Issue query parameters
 */
export interface IssueQueryParams {
  project_id?: number;
  tracker_id?: number;
  status_id?: number | "open" | "closed";
  priority_id?: number;
  category_id?: number;
  author_id?: number;
  assigned_to_id?: number | "me";
  parent_id?: number;
  subject?: string;
  start_date_from?: string;
  start_date_to?: string;
  due_date_from?: string;
  due_date_to?: string;
  is_private?: boolean;
  is_closed?: boolean;
}

/**
 * Issue relation type
 */
export interface IssueRelation {
  id: number;
  issue_id: number;
  issue_to_id: number;
  relation_type: IssueRelationType;
  delay: number | null;
}

/**
 * Status with is_closed flag (used in issue nested response)
 */
export interface IssueStatusNested {
  id: number;
  name: string;
  is_closed: boolean;
}

/**
 * Child issue summary (from include=children)
 */
export interface ChildIssue {
  id: number;
  tracker?: NamedEntity;
  subject: string;
}

/**
 * Issue with related data for display
 */
export interface IssueDetail {
  id: number;
  project: NamedEntity;
  tracker: NamedEntity;
  status: IssueStatusNested;
  priority: NamedEntity;
  author: NamedEntity;
  assigned_to?: NamedEntity | null;
  project_id?: number;
  tracker_id?: number;
  status_id?: number;
  priority_id?: number;
  assigned_to_id?: number | null;
  author_id?: number;
  subject: string;
  category?: IssueCategory | null;
  category_id?: number | null;
  parent_id?: number | null;
  root_id?: number;
  lft?: number;
  rgt?: number;
  description: string | null;
  start_date: string | null;
  due_date: string | null;
  done_ratio: number;
  is_private: boolean;
  estimated_hours: number | null;
  created_on?: string;
  updated_on?: string;
  closed_on?: string | null;
  children?: ChildIssue[];
  attachments?: import("./journal").Attachment[];
  journals?: import("./journal").Journal[];
  relations?: IssueRelation[];
  total_estimated_hours?: number | null;
  spent_hours?: number;
  total_spent_hours?: number;
}
