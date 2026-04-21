import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { projectsApi } from "./projects";

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

describe("projectsApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("list", () => {
    it("should call GET /api/v1/projects with pagination params", async () => {
      const mockResponse = {
        projects: [
          {
            id: 1,
            name: "Test Project",
            identifier: "test-project",
            description: null,
            homepage: null,
            is_public: true,
            status: 1,
            created_on: "2026-01-01T00:00:00Z",
            updated_on: "2026-01-01T00:00:00Z",
          },
        ],
        total_count: 1,
        offset: 0,
        limit: 25,
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await projectsApi.list({ offset: 0, limit: 25 });

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/projects", {
        offset: 0,
        limit: 25,
      });
      expect(result).toEqual(mockResponse);
    });

    it("should call GET /api/v1/projects with status filter", async () => {
      const mockResponse = {
        projects: [],
        total_count: 0,
        offset: 0,
        limit: 25,
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      await projectsApi.list({ status: 1 });

      expect(mockGet).toHaveBeenCalledWith("/api/v1/projects", { status: 1 });
    });
  });

  describe("get", () => {
    it("should call GET /api/v1/projects/:id", async () => {
      const mockResponse = {
        project: {
          id: 1,
          name: "Test Project",
          identifier: "test-project",
          description: "A test project",
          homepage: null,
          is_public: true,
          status: 1,
          created_on: "2026-01-01T00:00:00Z",
          updated_on: "2026-01-01T00:00:00Z",
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await projectsApi.get(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/projects/1", undefined);
      expect(result).toEqual(mockResponse);
    });

    it("should include optional include param", async () => {
      const mockResponse = {
        project: {
          id: 1,
          name: "Test Project",
          identifier: "test-project",
          description: null,
          homepage: null,
          is_public: true,
          status: 1,
          created_on: "2026-01-01T00:00:00Z",
          updated_on: "2026-01-01T00:00:00Z",
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      await projectsApi.get(1, { include: "trackers" });

      expect(mockGet).toHaveBeenCalledWith("/api/v1/projects/1", {
        include: "trackers",
      });
    });
  });

  describe("create", () => {
    it("should call POST /api/v1/projects with project data", async () => {
      const createData = {
        name: "New Project",
        identifier: "new-project",
        description: "A new project",
      };

      const mockResponse = {
        project: {
          id: 1,
          ...createData,
          homepage: null,
          is_public: true,
          status: 1,
          created_on: "2026-01-01T00:00:00Z",
          updated_on: "2026-01-01T00:00:00Z",
        },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await projectsApi.create(createData);

      expect(mockPost).toHaveBeenCalledTimes(1);
      expect(mockPost).toHaveBeenCalledWith("/api/v1/projects", {
        project: createData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("update", () => {
    it("should call PUT /api/v1/projects/:id with project data", async () => {
      const updateData = {
        name: "Updated Project",
        description: "Updated description",
      };

      const mockResponse = {
        project: {
          id: 1,
          name: "Updated Project",
          identifier: "test-project",
          description: "Updated description",
          homepage: null,
          is_public: true,
          status: 1,
          created_on: "2026-01-01T00:00:00Z",
          updated_on: "2026-01-02T00:00:00Z",
        },
      };

      mockPut.mockResolvedValueOnce(mockResponse);

      const result = await projectsApi.update(1, updateData);

      expect(mockPut).toHaveBeenCalledTimes(1);
      expect(mockPut).toHaveBeenCalledWith("/api/v1/projects/1", {
        project: updateData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("delete", () => {
    it("should call DELETE /api/v1/projects/:id", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await projectsApi.delete(1);

      expect(mockDel).toHaveBeenCalledTimes(1);
      expect(mockDel).toHaveBeenCalledWith("/api/v1/projects/1");
    });
  });

  describe("getTrackers", () => {
    it("should call GET /api/v1/projects/:id/trackers", async () => {
      const mockResponse = {
        trackers: [
          { id: 1, name: "Bug" },
          { id: 2, name: "Feature" },
        ],
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await projectsApi.getTrackers(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/projects/1/trackers");
      expect(result).toEqual(mockResponse);
    });
  });
});
