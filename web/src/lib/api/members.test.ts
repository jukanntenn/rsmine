import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { membersApi } from "./members";

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

describe("membersApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("list", () => {
    it("should call GET /api/v1/projects/:projectId/memberships", async () => {
      const mockResponse = {
        memberships: [
          {
            id: 1,
            project: { id: 1, name: "Test Project" },
            user: { id: 1, login: "admin", firstname: "Admin", lastname: "User", mail: "admin@example.com" },
            roles: [{ id: 1, name: "Manager" }],
          },
        ],
        total_count: 1,
        offset: 0,
        limit: 25,
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await membersApi.list(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith(
        "/api/v1/projects/1/memberships",
        undefined
      );
      expect(result).toEqual(mockResponse);
    });

    it("should pass pagination params", async () => {
      const mockResponse = {
        memberships: [],
        total_count: 0,
        offset: 10,
        limit: 10,
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      await membersApi.list(1, { offset: 10, limit: 10 });

      expect(mockGet).toHaveBeenCalledWith("/api/v1/projects/1/memberships", {
        offset: 10,
        limit: 10,
      });
    });
  });

  describe("get", () => {
    it("should call GET /api/v1/memberships/:id", async () => {
      const mockResponse = {
        membership: {
          id: 1,
          project: { id: 1, name: "Test Project" },
          user: { id: 1, login: "admin", firstname: "Admin", lastname: "User", mail: "admin@example.com" },
          roles: [{ id: 1, name: "Manager" }],
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await membersApi.get(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/memberships/1");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("create", () => {
    it("should call POST /api/v1/projects/:projectId/memberships", async () => {
      const createData = {
        user_id: 2,
        role_ids: [1, 2],
      };

      const mockResponse = {
        membership: {
          id: 1,
          project: { id: 1, name: "Test Project" },
          user: { id: 2, login: "newuser", firstname: "New", lastname: "User", mail: "new@example.com" },
          roles: [
            { id: 1, name: "Manager" },
            { id: 2, name: "Developer" },
          ],
        },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await membersApi.create(1, createData);

      expect(mockPost).toHaveBeenCalledTimes(1);
      expect(mockPost).toHaveBeenCalledWith(
        "/api/v1/projects/1/memberships",
        { membership: createData }
      );
      expect(result).toEqual(mockResponse);
    });
  });

  describe("update", () => {
    it("should call PUT /api/v1/memberships/:id", async () => {
      const updateData = {
        role_ids: [2],
      };

      const mockResponse = {
        membership: {
          id: 1,
          project: { id: 1, name: "Test Project" },
          user: { id: 2, login: "user", firstname: "Test", lastname: "User", mail: "test@example.com" },
          roles: [{ id: 2, name: "Developer" }],
        },
      };

      mockPut.mockResolvedValueOnce(mockResponse);

      const result = await membersApi.update(1, updateData);

      expect(mockPut).toHaveBeenCalledTimes(1);
      expect(mockPut).toHaveBeenCalledWith("/api/v1/memberships/1", {
        membership: updateData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("delete", () => {
    it("should call DELETE /api/v1/memberships/:id", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await membersApi.delete(1);

      expect(mockDel).toHaveBeenCalledTimes(1);
      expect(mockDel).toHaveBeenCalledWith("/api/v1/memberships/1");
    });
  });
});
