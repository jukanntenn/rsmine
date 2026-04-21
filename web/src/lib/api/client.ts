import type { ApiErrorResponse } from "@/types/api";

/**
 * API Configuration
 */
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "";

/**
 * Custom error class for API errors
 */
export class ApiClientError extends Error {
  statusCode: number;
  errorType: string;

  constructor(message: string, statusCode: number, errorType: string = "API_ERROR") {
    super(message);
    this.name = "ApiClientError";
    this.statusCode = statusCode;
    this.errorType = errorType;
  }
}

/**
 * Get authentication token from storage
 */
function getAuthToken(): string | null {
  if (typeof window === "undefined") return null;
  return localStorage.getItem("auth_token");
}

/**
 * Build URL with query parameters
 */
function buildUrl(endpoint: string, params?: Record<string, unknown>): string {
  const url = new URL(endpoint, API_BASE_URL || window.location.origin);
  
  if (params) {
    Object.entries(params).forEach(([key, value]) => {
      if (value !== undefined && value !== null) {
        url.searchParams.append(key, String(value));
      }
    });
  }
  
  return url.toString();
}

/**
 * Default request headers
 */
function getDefaultHeaders(): HeadersInit {
  const headers: HeadersInit = {
    "Content-Type": "application/json",
    Accept: "application/json",
  };

  const token = getAuthToken();
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  return headers;
}

/**
 * Handle API response
 */
async function handleResponse<T>(response: Response): Promise<T> {
  const contentType = response.headers.get("content-type");
  const isJson = contentType?.includes("application/json");

  if (!response.ok) {
    if (isJson) {
      const error: ApiErrorResponse = await response.json();
      throw new ApiClientError(
        error.errors?.join(", ") || "An error occurred",
        response.status,
        "API_ERROR"
      );
    }
    throw new ApiClientError(
      response.statusText || "An error occurred",
      response.status
    );
  }

  // Handle empty responses (204 No Content)
  if (response.status === 204) {
    return undefined as T;
  }

  if (isJson) {
    return response.json();
  }

  // Handle non-JSON responses (e.g., file downloads)
  return response as unknown as T;
}

/**
 * HTTP GET request
 */
export async function get<T>(
  endpoint: string,
  params?: Record<string, unknown>
): Promise<T> {
  const response = await fetch(buildUrl(endpoint, params), {
    method: "GET",
    headers: getDefaultHeaders(),
    credentials: "include",
  });

  return handleResponse<T>(response);
}

/**
 * HTTP POST request
 */
export async function post<T, B = unknown>(
  endpoint: string,
  body?: B,
  params?: Record<string, unknown>
): Promise<T> {
  const response = await fetch(buildUrl(endpoint, params), {
    method: "POST",
    headers: getDefaultHeaders(),
    body: body ? JSON.stringify(body) : undefined,
    credentials: "include",
  });

  return handleResponse<T>(response);
}

/**
 * HTTP PUT request
 */
export async function put<T, B = unknown>(
  endpoint: string,
  body?: B,
  params?: Record<string, unknown>
): Promise<T> {
  const response = await fetch(buildUrl(endpoint, params), {
    method: "PUT",
    headers: getDefaultHeaders(),
    body: body ? JSON.stringify(body) : undefined,
    credentials: "include",
  });

  return handleResponse<T>(response);
}

/**
 * HTTP PATCH request
 */
export async function patch<T, B = unknown>(
  endpoint: string,
  body?: B,
  params?: Record<string, unknown>
): Promise<T> {
  const response = await fetch(buildUrl(endpoint, params), {
    method: "PATCH",
    headers: getDefaultHeaders(),
    body: body ? JSON.stringify(body) : undefined,
    credentials: "include",
  });

  return handleResponse<T>(response);
}

/**
 * HTTP DELETE request
 */
export async function del<T>(
  endpoint: string,
  params?: Record<string, unknown>
): Promise<T> {
  const response = await fetch(buildUrl(endpoint, params), {
    method: "DELETE",
    headers: getDefaultHeaders(),
    credentials: "include",
  });

  return handleResponse<T>(response);
}

/**
 * Upload file (multipart/form-data)
 */
export async function upload<T>(
  endpoint: string,
  formData: FormData,
  params?: Record<string, unknown>
): Promise<T> {
  const headers: HeadersInit = {
    Accept: "application/json",
  };

  const token = getAuthToken();
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const response = await fetch(buildUrl(endpoint, params), {
    method: "POST",
    headers,
    body: formData,
    credentials: "include",
  });

  return handleResponse<T>(response);
}

/**
 * Download file (returns blob)
 */
export async function download(
  endpoint: string,
  params?: Record<string, unknown>
): Promise<Blob> {
  const headers: HeadersInit = {
    Accept: "*/*",
  };

  const token = getAuthToken();
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const response = await fetch(buildUrl(endpoint, params), {
    method: "GET",
    headers,
    credentials: "include",
  });

  if (!response.ok) {
    const contentType = response.headers.get("content-type");
    if (contentType?.includes("application/json")) {
      const error: ApiErrorResponse = await response.json();
      throw new ApiClientError(
        error.errors?.join(", ") || "An error occurred",
        response.status,
        "API_ERROR"
      );
    }
    throw new ApiClientError(
      response.statusText || "Download failed",
      response.status
    );
  }

  return response.blob();
}

// Export types for consumers
export type { ApiErrorResponse };

/**
 * API Client object with all methods
 */
export const apiClient = {
  get,
  post,
  put,
  patch,
  delete: del,
  upload,
  download,
};
