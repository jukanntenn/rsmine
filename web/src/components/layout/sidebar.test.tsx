import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { Sidebar } from "./sidebar";
import { useSidebarStore, useAuthStore } from "@/stores";

// Mock stores
vi.mock("@/stores", () => ({
  useSidebarStore: vi.fn(),
  useAuthStore: vi.fn(),
}));

// Mock next/navigation
vi.mock("next/navigation", () => ({
  usePathname: () => "/",
}));

describe("Sidebar", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (useSidebarStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      collapsed: false,
    });
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: null,
    });
  });

  it("renders main navigation items", () => {
    render(<Sidebar />);
    expect(screen.getByText("Projects")).toBeInTheDocument();
    expect(screen.getByText("Issues")).toBeInTheDocument();
  });

  it("does not render admin section for non-admin users", () => {
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: { id: 1, admin: false },
    });
    render(<Sidebar />);
    expect(screen.queryByText("Administration")).not.toBeInTheDocument();
    expect(screen.queryByText("Users")).not.toBeInTheDocument();
  });

  it("renders admin section for admin users", () => {
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: { id: 1, admin: true, firstname: "Admin", lastname: "User", mail: "admin@test.com" },
    });
    render(<Sidebar />);
    expect(screen.getByText("Administration")).toBeInTheDocument();
    expect(screen.getByText("Users")).toBeInTheDocument();
    expect(screen.getByText("Settings")).toBeInTheDocument();
  });

  it("renders settings section for admin users", () => {
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: { id: 1, admin: true, firstname: "Admin", lastname: "User", mail: "admin@test.com" },
    });
    render(<Sidebar />);
    expect(screen.getByText("Roles")).toBeInTheDocument();
    expect(screen.getByText("Trackers")).toBeInTheDocument();
    expect(screen.getByText("Statuses")).toBeInTheDocument();
    expect(screen.getByText("Priorities")).toBeInTheDocument();
  });

  it("hides section labels when collapsed", () => {
    (useSidebarStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      collapsed: true,
    });
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: { id: 1, admin: true, firstname: "Admin", lastname: "User", mail: "admin@test.com" },
    });
    render(<Sidebar />);
    expect(screen.queryByText("Administration")).not.toBeInTheDocument();
    expect(screen.queryByText("Settings")).not.toBeInTheDocument();
  });

  it("applies collapsed width class", () => {
    (useSidebarStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      collapsed: true,
    });
    const { container } = render(<Sidebar />);
    const aside = container.querySelector("aside");
    expect(aside).toHaveClass("w-16");
  });

  it("applies expanded width class when not collapsed", () => {
    (useSidebarStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      collapsed: false,
    });
    const { container } = render(<Sidebar />);
    const aside = container.querySelector("aside");
    expect(aside).toHaveClass("w-64");
  });
});
