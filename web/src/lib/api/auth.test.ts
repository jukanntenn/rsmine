import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { authApi } from "./auth";

// Mock the client module
vi.mock("./client", () => ({
  get: vi.fn(),
  post: vi.fn(),
}));

import { get, post } from "./client";

const mockGet = vi.mocked(get);
const mockPost = vi.mocked(post);

describe("authApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("login", () => {
    it("should call POST /api/v1/auth/login with credentials", async () => {
      const mockResponse = {
        token: "test-token-123",
        user: {
          id: 1,
          login: "admin",
          firstname: "Admin",
          lastname: "User",
          mail: "admin@example.com",
          admin: true,
          language: "en",
        },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await authApi.login({
        username: "admin",
        password: "password123",
      });

      expect(mockPost).toHaveBeenCalledTimes(1);
      expect(mockPost).toHaveBeenCalledWith("/api/v1/auth/login", {
        username: "admin",
        password: "password123",
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("logout", () => {
    it("should call POST /api/v1/auth/logout", async () => {
      mockPost.mockResolvedValueOnce(undefined);

      await authApi.logout();

      expect(mockPost).toHaveBeenCalledTimes(1);
      expect(mockPost).toHaveBeenCalledWith("/api/v1/auth/logout");
    });
  });

  describe("getCurrentUser", () => {
    it("should call GET /api/v1/auth/me", async () => {
      const mockResponse = {
        user: {
          id: 1,
          login: "admin",
          firstname: "Admin",
          lastname: "User",
          mail: "admin@example.com",
          admin: true,
          language: "en",
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await authApi.getCurrentUser();

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/auth/me");
      expect(result).toEqual(mockResponse);
    });
  });
});
