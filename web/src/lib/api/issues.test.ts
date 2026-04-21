import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { issuesApi } from "./issues";

// Mock the client module
vi.mock("./client", () => ({
  get: vi.fn(),
  post: vi.fn(),
  put: vi.fn(),
  del: vi.fn(),
  upload: vi.fn(),
}));

import { get, post, put, del, upload } from "./client";

const mockGet = vi.mocked(get);
const mockPost = vi.mocked(post);
const mockPut = vi.mocked(put);
const mockDel = vi.mocked(del);
const mockUpload = vi.mocked(upload);

describe("issuesApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("list", () => {
    it("should call GET /api/v1/issues with pagination params", async () => {
      const mockResponse = {
        issues: [
          {
            id: 1,
            project: { id: 1, name: "Test" },
            subject: "Test Issue",
          },
        ],
        total_count: 1,
        offset: 0,
        limit: 25,
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await issuesApi.list({ offset: 0, limit: 25 });

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/issues", {
        offset: 0,
        limit: 25,
      });
      expect(result).toEqual(mockResponse);
    });

    it("should call GET /api/v1/issues with query params", async () => {
      const mockResponse = {
        issues: [],
        total_count: 0,
        offset: 0,
        limit: 25,
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      await issuesApi.list({ project_id: 1, assigned_to_id: 2 });

      expect(mockGet).toHaveBeenCalledWith("/api/v1/issues", {
        project_id: 1,
        assigned_to_id: 2,
      });
    });
  });

  describe("get", () => {
    it("should call GET /api/v1/issues/:id", async () => {
      const mockResponse = {
        issue: {
          id: 1,
          project: { id: 1, name: "Test" },
          subject: "Test Issue",
          description: "Test description",
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await issuesApi.get(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/issues/1", undefined);
      expect(result).toEqual(mockResponse);
    });

    it("should pass include param", async () => {
      const mockResponse = {
        issue: {
          id: 1,
          subject: "Test Issue",
          attachments: [],
          journals: [],
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      await issuesApi.get(1, { include: "attachments,journals" });

      expect(mockGet).toHaveBeenCalledWith("/api/v1/issues/1", {
        include: "attachments,journals",
      });
    });
  });

  describe("create", () => {
    it("should call POST /api/v1/issues with issue data", async () => {
      const createData = {
        project_id: 1,
        tracker_id: 1,
        subject: "New Issue",
        description: "Issue description",
      };

      const mockResponse = {
        issue: {
          id: 1,
          ...createData,
          status_id: 1,
          priority_id: 2,
          created_on: "2026-01-01T00:00:00Z",
        },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await issuesApi.create(createData);

      expect(mockPost).toHaveBeenCalledTimes(1);
      expect(mockPost).toHaveBeenCalledWith("/api/v1/issues", {
        issue: createData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("update", () => {
    it("should call PUT /api/v1/issues/:id with issue data", async () => {
      const updateData = {
        subject: "Updated Issue",
        status_id: 2,
        notes: "Status changed",
      };

      const mockResponse = {
        issue: {
          id: 1,
          subject: "Updated Issue",
          status_id: 2,
          updated_on: "2026-01-02T00:00:00Z",
        },
      };

      mockPut.mockResolvedValueOnce(mockResponse);

      const result = await issuesApi.update(1, updateData);

      expect(mockPut).toHaveBeenCalledTimes(1);
      expect(mockPut).toHaveBeenCalledWith("/api/v1/issues/1", {
        issue: updateData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("delete", () => {
    it("should call DELETE /api/v1/issues/:id", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await issuesApi.delete(1);

      expect(mockDel).toHaveBeenCalledTimes(1);
      expect(mockDel).toHaveBeenCalledWith("/api/v1/issues/1");
    });
  });

  describe("getJournals", () => {
    it("should call GET /api/v1/issues/:id/journals", async () => {
      const mockResponse = {
        journals: [
          {
            id: 1,
            user: { id: 1, name: "Admin" },
            notes: "Comment",
            created_on: "2026-01-01T00:00:00Z",
            private_notes: false,
            details: [],
          },
        ],
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await issuesApi.getJournals(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/issues/1/journals");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("getAttachments", () => {
    it("should call GET /api/v1/issues/:id/attachments", async () => {
      const mockResponse = {
        attachments: [
          {
            id: 1,
            filename: "test.txt",
            filesize: 100,
            content_type: "text/plain",
            created_on: "2026-01-01T00:00:00Z",
          },
        ],
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await issuesApi.getAttachments(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/issues/1/attachments");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("getRelations", () => {
    it("should call GET /api/v1/issues/:id/relations", async () => {
      const mockResponse = {
        relations: [
          {
            id: 1,
            issue_id: 1,
            issue_to_id: 2,
            relation_type: "relates",
            delay: null,
          },
        ],
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await issuesApi.getRelations(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/issues/1/relations");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("uploadAttachment", () => {
    it("should call upload with FormData", async () => {
      const formData = new FormData();
      formData.append("file", new Blob(["test"]), "test.txt");

      const mockResponse = {
        attachment: {
          id: 1,
          filename: "test.txt",
          filesize: 4,
          content_type: "text/plain",
          created_on: "2026-01-01T00:00:00Z",
        },
      };

      mockUpload.mockResolvedValueOnce(mockResponse);

      const result = await issuesApi.uploadAttachment(1, formData);

      expect(mockUpload).toHaveBeenCalledTimes(1);
      expect(mockUpload).toHaveBeenCalledWith(
        "/api/v1/issues/1/attachments",
        formData
      );
      expect(result).toEqual(mockResponse);
    });
  });
});
