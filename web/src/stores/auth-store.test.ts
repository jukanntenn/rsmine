import { describe, it, expect, beforeEach, vi } from "vitest";
import { useAuthStore } from "./auth-store";
import type { AuthUser } from "@/types";

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
  language: "en",
};

describe("auth-store", () => {
  beforeEach(() => {
    // Reset store state before each test
    useAuthStore.setState({
      user: null,
      token: null,
      isAuthenticated: false,
    });
    vi.clearAllMocks();
  });

  describe("initial state", () => {
    it("should have correct initial state", () => {
      const state = useAuthStore.getState();

      expect(state.user).toBeNull();
      expect(state.token).toBeNull();
      expect(state.isAuthenticated).toBe(false);
    });
  });

  describe("setUser", () => {
    it("should set user and update isAuthenticated", () => {
      useAuthStore.getState().setUser(mockUser);
      const state = useAuthStore.getState();

      expect(state.user).toEqual(mockUser);
      expect(state.isAuthenticated).toBe(true);
    });

    it("should clear user when set to null", () => {
      useAuthStore.getState().setUser(mockUser);
      expect(useAuthStore.getState().isAuthenticated).toBe(true);

      useAuthStore.getState().setUser(null);
      const state = useAuthStore.getState();

      expect(state.user).toBeNull();
      expect(state.isAuthenticated).toBe(false);
    });
  });

  describe("setToken", () => {
    it("should set token", () => {
      useAuthStore.getState().setToken("test-token-123");
      const state = useAuthStore.getState();

      expect(state.token).toBe("test-token-123");
    });

    it("should store token in localStorage", () => {
      useAuthStore.getState().setToken("test-token-456");

      expect(localStorage.setItem).toHaveBeenCalledWith(
        "auth_token",
        "test-token-456"
      );
    });

    it("should remove token from localStorage when set to null", () => {
      useAuthStore.getState().setToken("test-token");
      useAuthStore.getState().setToken(null);
      const state = useAuthStore.getState();

      expect(localStorage.removeItem).toHaveBeenCalledWith("auth_token");
      expect(state.token).toBeNull();
    });
  });

  describe("login", () => {
    it("should set user and token together", () => {
      useAuthStore.getState().login(mockUser, "login-token");
      const state = useAuthStore.getState();

      expect(state.user).toEqual(mockUser);
      expect(state.token).toBe("login-token");
      expect(state.isAuthenticated).toBe(true);
    });

    it("should store token in localStorage on login", () => {
      useAuthStore.getState().login(mockUser, "login-token-789");

      expect(localStorage.setItem).toHaveBeenCalledWith(
        "auth_token",
        "login-token-789"
      );
    });
  });

  describe("logout", () => {
    it("should clear all auth state", () => {
      // First login
      useAuthStore.getState().login(mockAdminUser, "admin-token");
      expect(useAuthStore.getState().isAuthenticated).toBe(true);

      // Then logout
      useAuthStore.getState().logout();
      const state = useAuthStore.getState();

      expect(state.user).toBeNull();
      expect(state.token).toBeNull();
      expect(state.isAuthenticated).toBe(false);
    });

    it("should remove token from localStorage on logout", () => {
      useAuthStore.getState().login(mockUser, "token-to-clear");
      useAuthStore.getState().logout();

      expect(localStorage.removeItem).toHaveBeenCalledWith("auth_token");
    });
  });

  describe("user updates", () => {
    it("should allow setting different users", () => {
      useAuthStore.getState().setUser(mockUser);
      expect(useAuthStore.getState().user?.login).toBe("testuser");

      useAuthStore.getState().setUser(mockAdminUser);
      const state = useAuthStore.getState();

      expect(state.user?.login).toBe("admin");
      expect(state.user?.admin).toBe(true);
    });
  });
});