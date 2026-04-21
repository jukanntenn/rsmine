import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { attachmentsApi, uploadsApi } from "./attachments";

// Mock the client module
vi.mock("./client", () => ({
  get: vi.fn(),
  patch: vi.fn(),
  del: vi.fn(),
  download: vi.fn(),
  upload: vi.fn(),
}));

import { get, patch, del, download, upload } from "./client";

const mockGet = vi.mocked(get);
const mockPatch = vi.mocked(patch);
const mockDel = vi.mocked(del);
const mockDownload = vi.mocked(download);
const mockUpload = vi.mocked(upload);

describe("attachmentsApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("get", () => {
    it("should call GET /api/v1/attachments/:id", async () => {
      const mockResponse = {
        attachment: {
          id: 1,
          filename: "document.pdf",
          filesize: 1024,
          content_type: "application/pdf",
          description: "Test document",
          created_on: "2026-01-01T00:00:00Z",
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await attachmentsApi.get(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/attachments/1");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("download", () => {
    it("should call download and return Blob", async () => {
      const mockBlob = new Blob(["file content"], { type: "text/plain" });

      mockDownload.mockResolvedValueOnce(mockBlob);

      const result = await attachmentsApi.download(1);

      expect(mockDownload).toHaveBeenCalledTimes(1);
      expect(mockDownload).toHaveBeenCalledWith(
        "/api/v1/attachments/download/1"
      );
      expect(result).toBe(mockBlob);
    });
  });

  describe("update", () => {
    it("should call PATCH /api/v1/attachments/:id", async () => {
      const updateData = { description: "Updated description" };

      const mockResponse = {
        attachment: {
          id: 1,
          filename: "document.pdf",
          filesize: 1024,
          content_type: "application/pdf",
          description: "Updated description",
          created_on: "2026-01-01T00:00:00Z",
        },
      };

      mockPatch.mockResolvedValueOnce(mockResponse);

      const result = await attachmentsApi.update(1, updateData);

      expect(mockPatch).toHaveBeenCalledTimes(1);
      expect(mockPatch).toHaveBeenCalledWith("/api/v1/attachments/1", {
        attachment: updateData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("delete", () => {
    it("should call DELETE /api/v1/attachments/:id", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await attachmentsApi.delete(1);

      expect(mockDel).toHaveBeenCalledTimes(1);
      expect(mockDel).toHaveBeenCalledWith("/api/v1/attachments/1");
    });
  });
});

describe("uploadsApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("upload", () => {
    it("should call upload with FormData and return token", async () => {
      const formData = new FormData();
      formData.append("file", new Blob(["test content"]), "test.txt");

      const mockResponse = {
        upload: {
          token: "upload-token-abc123",
        },
      };

      mockUpload.mockResolvedValueOnce(mockResponse);

      const result = await uploadsApi.upload(formData);

      expect(mockUpload).toHaveBeenCalledTimes(1);
      expect(mockUpload).toHaveBeenCalledWith("/api/v1/uploads", formData);
      expect(result).toEqual(mockResponse);
    });
  });
});
