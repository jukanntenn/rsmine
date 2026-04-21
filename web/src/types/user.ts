/**
 * User type
 */
export interface User {
  id: number;
  login: string;
  firstname: string;
  lastname: string;
  mail: string;
  admin: boolean;
  status: number;
  language: string | null;
  last_login_on: string | null;
  created_on: string;
  updated_on: string | null;
  avatar_url?: string;
}

/**
 * User creation request
 */
export interface UserCreateRequest {
  login: string;
  firstname: string;
  lastname: string;
  mail: string;
  password?: string;
  admin?: boolean;
  status?: number;
  language?: string;
}

/**
 * User update request
 */
export interface UserUpdateRequest {
  firstname?: string;
  lastname?: string;
  mail?: string;
  password?: string;
  admin?: boolean;
  status?: number;
  language?: string;
}
