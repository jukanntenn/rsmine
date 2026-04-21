import { get, post, put, del } from "./client";
import type {
  Issue,
  IssueDetail,
  IssueCreateRequest,
  IssueUpdateRequest,
  IssueQueryParams,
  PaginationParams,
  PaginatedListResponse,
  Journal,
  Attachment,
  IssueRelation,
} from "@/types";

/**
 * Issues API endpoints
 */
export const issuesApi = {
  /**
   * Get list of issues with optional filtering
   */
  list: async (params?: PaginationParams & IssueQueryParams): Promise<PaginatedListResponse<IssueDetail, "issues">> => {
    return get<PaginatedListResponse<IssueDetail, "issues">>("/api/v1/issues", params);
  },

  /**
   * Get a single issue by ID with optional includes
   */
  get: async (
    id: number,
    params?: { include?: string }
  ): Promise<{ issue: IssueDetail }> => {
    return get<{ issue: IssueDetail }>(`/api/v1/issues/${id}`, params);
  },

  /**
   * Create a new issue
   */
  create: async (data: IssueCreateRequest): Promise<{ issue: Issue }> => {
    return post<{ issue: Issue }>("/api/v1/issues", { issue: data });
  },

  /**
   * Update an existing issue
   */
  update: async (id: number, data: IssueUpdateRequest): Promise<{ issue: Issue }> => {
    return put<{ issue: Issue }>(`/api/v1/issues/${id}`, { issue: data });
  },

  /**
   * Delete an issue
   */
  delete: async (id: number): Promise<void> => {
    return del<void>(`/api/v1/issues/${id}`);
  },

  /**
   * Get journals (change history) for an issue
   */
  getJournals: async (issueId: number): Promise<{ journals: Journal[] }> => {
    return get<{ journals: Journal[] }>(`/api/v1/issues/${issueId}/journals`);
  },

  /**
   * Get attachments for an issue
   */
  getAttachments: async (issueId: number): Promise<{ attachments: Attachment[] }> => {
    return get<{ attachments: Attachment[] }>(`/api/v1/issues/${issueId}/attachments`);
  },

  /**
   * Get relations for an issue
   */
  getRelations: async (issueId: number): Promise<{ relations: IssueRelation[] }> => {
    return get<{ relations: IssueRelation[] }>(`/api/v1/issues/${issueId}/relations`);
  },

  /**
   * Upload attachment to an issue
   */
  uploadAttachment: async (issueId: number, formData: FormData): Promise<{ attachment: Attachment }> => {
    const { upload } = await import("./client");
    return upload<{ attachment: Attachment }>(`/api/v1/issues/${issueId}/attachments`, formData);
  },
};
