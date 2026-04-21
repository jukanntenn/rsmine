import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook } from "@testing-library/react";
import { useCurrentUser } from "./use-current-user";
import type { AuthUser } from "@/types";

// Mock the store
vi.mock("@/stores", () => ({
  useAuthStore: vi.fn(),
}));

import { useAuthStore } from "@/stores";

// Mock user data
const mockUser: AuthUser = {
  id: 1,
  login: "testuser",
  firstname: "Test",
  lastname: "User",
  mail: "test@example.com",
  admin: false,
  language: "en",
};

const mockAdminUser: AuthUser = {
  id: 2,
  login: "admin",
  firstname: "Admin",
  lastname: "User",
  mail: "admin@example.com",
  admin: true,
  language: "zh",
};

describe("useCurrentUser", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("when not authenticated", () => {
    beforeEach(() => {
      vi.mocked(useAuthStore).mockImplementation((selector) =>
        selector({
          user: null,
          isAuthenticated: false,
        } as never)
      );
    });

    it("should return null user", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.user).toBeNull();
      expect(result.current.isAuthenticated).toBe(false);
    });

    it("should return false for isAdmin", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.isAdmin).toBe(false);
    });

    it("should return null for user-related fields", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.userId).toBeUndefined();
      expect(result.current.userName).toBeNull();
      expect(result.current.userLogin).toBeNull();
      expect(result.current.userEmail).toBeNull();
      expect(result.current.userLanguage).toBeNull();
    });
  });

  describe("when authenticated as regular user", () => {
    beforeEach(() => {
      vi.mocked(useAuthStore).mockImplementation((selector) =>
        selector({
          user: mockUser,
          isAuthenticated: true,
        } as never)
      );
    });

    it("should return the user object", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.user).toEqual(mockUser);
      expect(result.current.isAuthenticated).toBe(true);
    });

    it("should return correct user id", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.userId).toBe(1);
    });

    it("should return formatted user name", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.userName).toBe("Test User");
    });

    it("should return correct login", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.userLogin).toBe("testuser");
    });

    it("should return correct email", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.userEmail).toBe("test@example.com");
    });

    it("should return correct language", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.userLanguage).toBe("en");
    });

    it("should return false for isAdmin", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.isAdmin).toBe(false);
    });
  });

  describe("when authenticated as admin user", () => {
    beforeEach(() => {
      vi.mocked(useAuthStore).mockImplementation((selector) =>
        selector({
          user: mockAdminUser,
          isAuthenticated: true,
        } as never)
      );
    });

    it("should return true for isAdmin", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.isAdmin).toBe(true);
    });

    it("should return correct admin user details", () => {
      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.userId).toBe(2);
      expect(result.current.userName).toBe("Admin User");
      expect(result.current.userLogin).toBe("admin");
      expect(result.current.userEmail).toBe("admin@example.com");
      expect(result.current.userLanguage).toBe("zh");
    });
  });

  describe("with null language", () => {
    it("should return null for userLanguage", () => {
      const userWithNullLang: AuthUser = {
        ...mockUser,
        language: null,
      };

      vi.mocked(useAuthStore).mockImplementation((selector) =>
        selector({
          user: userWithNullLang,
          isAuthenticated: true,
        } as never)
      );

      const { result } = renderHook(() => useCurrentUser());

      expect(result.current.userLanguage).toBeNull();
    });
  });
});
