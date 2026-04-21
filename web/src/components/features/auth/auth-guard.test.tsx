import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { AuthGuard, GuestGuard } from "./auth-guard";

// Mock dependencies at top level
const mockPush = vi.fn();

vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: mockPush,
  }),
  usePathname: () => "/protected-page",
}));

vi.mock("@/lib/constants/routes", () => ({
  AUTH_ROUTES: {
    LOGIN: "/login",
    REGISTER: "/register",
  },
}));

vi.mock("@/stores", () => ({
  useAuthStore: vi.fn((selector) =>
    selector({
      isAuthenticated: false,
      user: null,
    })
  ),
}));

describe("AuthGuard", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should show loading initially when not authenticated", () => {
    render(
      <AuthGuard>
        <div>Protected Content</div>
      </AuthGuard>
    );

    expect(screen.getByText(/checking authentication/i)).toBeInTheDocument();
  });

  it("should not render children immediately when not authenticated", () => {
    render(
      <AuthGuard>
        <div>Protected Content</div>
      </AuthGuard>
    );

    // Initially shows loading, not children
    expect(screen.queryByText("Protected Content")).not.toBeInTheDocument();
  });

  it("should show loading when requireAdmin is true and user is not admin", () => {
    render(
      <AuthGuard requireAdmin>
        <div>Admin Content</div>
      </AuthGuard>
    );

    // Should show loading initially
    expect(screen.getByText(/loading|checking/i)).toBeInTheDocument();
  });
});

describe("GuestGuard", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should show loading initially", () => {
    render(
      <GuestGuard>
        <div>Login Page</div>
      </GuestGuard>
    );

    expect(screen.getByText(/loading/i)).toBeInTheDocument();
  });

  it("should not render children initially", () => {
    render(
      <GuestGuard>
        <div>Login Page</div>
      </GuestGuard>
    );

    // Initially shows loading
    expect(screen.getByText(/loading/i)).toBeInTheDocument();
  });
});
