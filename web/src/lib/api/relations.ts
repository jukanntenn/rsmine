import { get, post, del } from "./client";
import type { IssueRelation, IssueRelationType } from "@/types";

/**
 * Issue Relations API endpoints
 */
export const relationsApi = {
  /**
   * Get list of relations for an issue
   */
  list: async (issueId: number): Promise<{ relations: IssueRelation[] }> => {
    return get<{ relations: IssueRelation[] }>(`/api/v1/issues/${issueId}/relations`);
  },

  /**
   * Get a single relation by ID
   */
  get: async (id: number): Promise<{ relation: IssueRelation }> => {
    return get<{ relation: IssueRelation }>(`/api/v1/relations/${id}`);
  },

  /**
   * Create a new relation
   */
  create: async (
    issueId: number,
    data: {
      issue_to_id: number;
      relation_type: IssueRelationType;
      delay?: number;
    }
  ): Promise<{ relation: IssueRelation }> => {
    return post<{ relation: IssueRelation }>(`/api/v1/issues/${issueId}/relations`, {
      relation: data,
    });
  },

  /**
   * Delete a relation
   */
  delete: async (id: number): Promise<void> => {
    return del<void>(`/api/v1/relations/${id}`);
  },
};
