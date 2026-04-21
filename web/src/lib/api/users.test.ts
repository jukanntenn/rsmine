import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { usersApi } from "./users";

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

describe("usersApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("list", () => {
    it("should call GET /api/v1/users with pagination params", async () => {
      const mockResponse = {
        users: [
          { id: 1, login: "admin", firstname: "Admin", lastname: "User" },
        ],
        total_count: 1,
        offset: 0,
        limit: 25,
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await usersApi.list({ offset: 0, limit: 25 });

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/users", {
        offset: 0,
        limit: 25,
      });
      expect(result).toEqual(mockResponse);
    });

    it("should call GET /api/v1/users with status filter", async () => {
      const mockResponse = {
        users: [],
        total_count: 0,
        offset: 0,
        limit: 25,
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      await usersApi.list({ status: 1 });

      expect(mockGet).toHaveBeenCalledWith("/api/v1/users", { status: 1 });
    });
  });

  describe("get", () => {
    it("should call GET /api/v1/users/:id", async () => {
      const mockResponse = {
        user: {
          id: 1,
          login: "admin",
          firstname: "Admin",
          lastname: "User",
          mail: "admin@example.com",
          admin: true,
          status: 1,
          language: null,
          last_login_on: null,
          created_on: "2026-01-01T00:00:00Z",
          updated_on: null,
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await usersApi.get(1);

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/users/1");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("create", () => {
    it("should call POST /api/v1/users with user data", async () => {
      const createData = {
        login: "newuser",
        firstname: "New",
        lastname: "User",
        mail: "new@example.com",
        password: "password123",
      };

      const mockResponse = {
        user: {
          id: 2,
          ...createData,
          admin: false,
          status: 1,
        },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await usersApi.create(createData);

      expect(mockPost).toHaveBeenCalledTimes(1);
      expect(mockPost).toHaveBeenCalledWith("/api/v1/users", { user: createData });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("update", () => {
    it("should call PUT /api/v1/users/:id with user data", async () => {
      const updateData = {
        firstname: "Updated",
        lastname: "Name",
      };

      const mockResponse = {
        user: {
          id: 1,
          login: "admin",
          firstname: "Updated",
          lastname: "Name",
          mail: "admin@example.com",
          admin: true,
          status: 1,
          language: null,
          last_login_on: null,
          created_on: "2026-01-01T00:00:00Z",
          updated_on: "2026-01-02T00:00:00Z",
        },
      };

      mockPut.mockResolvedValueOnce(mockResponse);

      const result = await usersApi.update(1, updateData);

      expect(mockPut).toHaveBeenCalledTimes(1);
      expect(mockPut).toHaveBeenCalledWith("/api/v1/users/1", { user: updateData });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("delete", () => {
    it("should call DELETE /api/v1/users/:id", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await usersApi.delete(1);

      expect(mockDel).toHaveBeenCalledTimes(1);
      expect(mockDel).toHaveBeenCalledWith("/api/v1/users/1");
    });
  });
});
