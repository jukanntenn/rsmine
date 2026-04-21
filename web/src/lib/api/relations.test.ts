import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { relationsApi } from "./relations";

// Mock the client module
vi.mock("./client", () => ({
  get: vi.fn(),
  post: vi.fn(),
  del: vi.fn(),
}));

import { get, post, del } from "./client";

const mockGet = vi.mocked(get);
const mockPost = vi.mocked(post);
const mockDel = vi.mocked(del);

describe("relationsApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("list", () => {
    it("should call GET /api/v1/issues/:issueId/relations", async () => {
      const mockResponse = {
        relations: [
          {
            id: 1,
            issue_id: 1,
            issue_to_id: 2,
            relation_type: "relates",
            delay: null,
          },
          {
            id: 2,
            issue_id: 1,
            issue_to_id: 3,
            relation_type: "blocks",
            delay: null,
          },
        ],
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await relationsApi.list(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/issues/1/relations");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("get", () => {
    it("should call GET /api/v1/relations/:id", async () => {
      const mockResponse = {
        relation: {
          id: 1,
          issue_id: 1,
          issue_to_id: 2,
          relation_type: "relates",
          delay: null,
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await relationsApi.get(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/relations/1");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("create", () => {
    it("should call POST /api/v1/issues/:issueId/relations", async () => {
      const createData = {
        issue_to_id: 2,
        relation_type: "relates" as const,
      };

      const mockResponse = {
        relation: {
          id: 1,
          issue_id: 1,
          issue_to_id: 2,
          relation_type: "relates",
          delay: null,
        },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await relationsApi.create(1, createData);

      expect(mockPost).toHaveBeenCalledTimes(1);
      expect(mockPost).toHaveBeenCalledWith("/api/v1/issues/1/relations", {
        relation: createData,
      });
      expect(result).toEqual(mockResponse);
    });

    it("should include delay for precedes relation", async () => {
      const createData = {
        issue_to_id: 2,
        relation_type: "precedes" as const,
        delay: 3,
      };

      const mockResponse = {
        relation: {
          id: 2,
          issue_id: 1,
          issue_to_id: 2,
          relation_type: "precedes",
          delay: 3,
        },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await relationsApi.create(1, createData);

      expect(mockPost).toHaveBeenCalledWith("/api/v1/issues/1/relations", {
        relation: createData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("delete", () => {
    it("should call DELETE /api/v1/relations/:id", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await relationsApi.delete(1);

      expect(mockDel).toHaveBeenCalledTimes(1);
      expect(mockDel).toHaveBeenCalledWith("/api/v1/relations/1");
    });
  });
});
