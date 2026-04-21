/**
 * Project status enum
 */
export type ProjectStatus = 1 | 5 | 9; // 1=active, 5=closed, 9=archived

/**
 * Project type
 */
export interface Project {
  id: number;
  name: string;
  identifier: string;
  description: string | null;
  homepage: string | null;
  is_public: boolean;
  parent_id: number | null;
  status: ProjectStatus;
  lft: number;
  rgt: number;
  inherit_members: boolean;
  created_on: string;
  updated_on: string | null;
}

/**
 * Project creation request
 */
export interface ProjectCreateRequest {
  name: string;
  identifier: string;
  description?: string;
  homepage?: string;
  is_public?: boolean;
  parent_id?: number;
  inherit_members?: boolean;
}

/**
 * Project update request
 */
export interface ProjectUpdateRequest {
  name?: string;
  description?: string;
  homepage?: string;
  is_public?: boolean;
  parent_id?: number | null;
  status?: ProjectStatus;
  inherit_members?: boolean;
}

/**
 * Project with additional info for list view
 */
export interface ProjectListItem extends Project {
  parent?: Pick<Project, "id" | "name">;
  issue_count?: number;
}
