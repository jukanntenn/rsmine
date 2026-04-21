/**
 * Authenticated user response
 */
export interface AuthUser {
  id: number;
  login: string;
  firstname: string;
  lastname: string;
  mail: string;
  admin: boolean;
  language: string | null;
}

/**
 * Login request
 */
export interface LoginRequest {
  username: string;
  password: string;
}

/**
 * Login response
 */
export interface LoginResponse {
  user: AuthUser;
  token: string;
}

/**
 * Token info
 */
export interface TokenInfo {
  user_id: number;
  token: string;
  expires_at: string;
}
