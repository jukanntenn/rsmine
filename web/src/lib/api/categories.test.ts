import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { categoriesApi } from "./categories";

// Mock the client module
vi.mock("./client", () => ({
  get: vi.fn(),
  post: vi.fn(),
  put: vi.fn(),
  del: vi.fn(),
}));

import { get, post, put, del } from "./client";

const mockGet = vi.mocked(get);
const mockPost = vi.mocked(post);
const mockPut = vi.mocked(put);
const mockDel = vi.mocked(del);

describe("categoriesApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("list", () => {
    it("should call GET /api/v1/projects/:projectId/issue_categories", async () => {
      const mockResponse = {
        issue_categories: [
          { id: 1, name: "Backend", project_id: 1, assigned_to_id: null },
          { id: 2, name: "Frontend", project_id: 1, assigned_to_id: 1 },
        ],
        total_count: 2,
        offset: 0,
        limit: 25,
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await categoriesApi.list(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith(
        "/api/v1/projects/1/issue_categories",
        undefined
      );
      expect(result).toEqual(mockResponse);
    });

    it("should pass pagination params", async () => {
      const mockResponse = {
        issue_categories: [],
        total_count: 0,
        offset: 10,
        limit: 10,
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      await categoriesApi.list(1, { offset: 10, limit: 10 });

      expect(mockGet).toHaveBeenCalledWith("/api/v1/projects/1/issue_categories", {
        offset: 10,
        limit: 10,
      });
    });
  });

  describe("get", () => {
    it("should call GET /api/v1/issue_categories/:id", async () => {
      const mockResponse = {
        issue_category: {
          id: 1,
          name: "Backend",
          project_id: 1,
          assigned_to_id: null,
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await categoriesApi.get(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/issue_categories/1");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("create", () => {
    it("should call POST /api/v1/projects/:projectId/issue_categories", async () => {
      const createData = { name: "Documentation", assigned_to_id: 2 };

      const mockResponse = {
        issue_category: {
          id: 3,
          name: "Documentation",
          project_id: 1,
          assigned_to_id: 2,
        },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await categoriesApi.create(1, createData);

      expect(mockPost).toHaveBeenCalledTimes(1);
      expect(mockPost).toHaveBeenCalledWith(
        "/api/v1/projects/1/issue_categories",
        { issue_category: createData }
      );
      expect(result).toEqual(mockResponse);
    });
  });

  describe("update", () => {
    it("should call PUT /api/v1/issue_categories/:id", async () => {
      const updateData = { name: "Backend Development" };

      const mockResponse = {
        issue_category: {
          id: 1,
          name: "Backend Development",
          project_id: 1,
          assigned_to_id: null,
        },
      };

      mockPut.mockResolvedValueOnce(mockResponse);

      const result = await categoriesApi.update(1, updateData);

      expect(mockPut).toHaveBeenCalledTimes(1);
      expect(mockPut).toHaveBeenCalledWith("/api/v1/issue_categories/1", {
        issue_category: updateData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("delete", () => {
    it("should call DELETE /api/v1/issue_categories/:id", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await categoriesApi.delete(1);

      expect(mockDel).toHaveBeenCalledTimes(1);
      expect(mockDel).toHaveBeenCalledWith(
        "/api/v1/issue_categories/1",
        undefined
      );
    });

    it("should pass reassign_to param", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await categoriesApi.delete(1, { reassign_to: 2 });

      expect(mockDel).toHaveBeenCalledWith("/api/v1/issue_categories/1", {
        reassign_to: 2,
      });
    });
  });
});
