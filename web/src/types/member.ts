/**
 * Member type
 */
export interface Member {
  id: number;
  project_id: number;
  user_id: number;
  created_on: string;
}

/**
 * Member with user and role info
 */
export interface MemberDetail extends Member {
  user: {
    id: number;
    name: string;
  };
  roles: Array<{
    id: number;
    name: string;
  }>;
}

/**
 * Member creation request
 */
export interface MemberCreateRequest {
  user_id: number;
  role_ids: number[];
}

/**
 * Member update request
 */
export interface MemberUpdateRequest {
  role_ids: number[];
}
