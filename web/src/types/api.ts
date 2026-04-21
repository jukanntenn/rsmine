/**
 * Named entity (used in nested objects like author, assigned_to, project)
 */
export interface NamedEntity {
  id: number;
  name: string;
}

/**
 * API Response wrapper type
 */
export interface ApiResponse<T> {
  [key: string]: T;
}

/**
 * Paginated API response type (Redmine-style)
 */
export interface PaginatedResponse {
  total_count: number;
  offset: number;
  limit: number;
}

/**
 * API Error response type
 */
export interface ApiErrorResponse {
  errors: string[];
}

/**
 * Pagination query parameters (Redmine-style)
 */
export interface PaginationParams {
  offset?: number;
  limit?: number;
  [key: string]: unknown;
}

/**
 * Sort query parameters
 */
export interface SortParams {
  sort?: string;
  [key: string]: unknown;
}

/**
 * Common query parameters combining pagination and sorting
 */
export type QueryParams = PaginationParams & SortParams;

/**
 * Helper type for creating a paginated response with a specific key
 */
export type PaginatedListResponse<T, K extends string> = PaginatedResponse & {
  [key in K]: T[];
};

/**
 * Helper type for single resource response with a specific key
 */
export type SingleResponse<T, K extends string> = {
  [key in K]: T;
};