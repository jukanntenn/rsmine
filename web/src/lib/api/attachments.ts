import { get, patch, del, download } from "./client";
import type { Attachment } from "@/types";

/**
 * Attachments API endpoints
 */
export const attachmentsApi = {
  /**
   * Get attachment metadata by ID
   */
  get: async (id: number): Promise<{ attachment: Attachment }> => {
    return get<{ attachment: Attachment }>(`/api/v1/attachments/${id}`);
  },

  /**
   * Download an attachment by ID
   */
  download: async (id: number): Promise<Blob> => {
    return download(`/api/v1/attachments/download/${id}`);
  },

  /**
   * Update attachment description
   */
  update: async (
    id: number,
    data: { description?: string }
  ): Promise<{ attachment: Attachment }> => {
    return patch<{ attachment: Attachment }>(`/api/v1/attachments/${id}`, {
      attachment: data,
    });
  },

  /**
   * Delete an attachment
   */
  delete: async (id: number): Promise<void> => {
    return del<void>(`/api/v1/attachments/${id}`);
  },
};

/**
 * Upload API endpoint for file uploads
 */
export const uploadsApi = {
  /**
   * Upload a file and get a token for attaching to resources
   */
  upload: async (formData: FormData): Promise<{ upload: { token: string } }> => {
    const { upload } = await import("./client");
    return upload<{ upload: { token: string } }>("/api/v1/uploads", formData);
  },
};
