import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { Header } from "./header";
import { useSidebarStore } from "@/stores";
import { useAuthStore } from "@/stores";

// Mock next/navigation
const mockPathname = vi.fn();
vi.mock("next/navigation", () => ({
  usePathname: () => mockPathname(),
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

// Mock stores
vi.mock("@/stores", () => ({
  useSidebarStore: vi.fn(),
  useAuthStore: vi.fn(),
}));

describe("Header", () => {
  const mockToggle = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    mockPathname.mockReturnValue("/");
    (useSidebarStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      collapsed: false,
      toggle: mockToggle,
    });
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: null,
    });
  });

  it("renders logo and brand name", () => {
    render(<Header />);
    expect(screen.getByText("Rsmine")).toBeInTheDocument();
  });

  it("renders navigation links", () => {
    render(<Header />);
    expect(screen.getByText("Projects")).toBeInTheDocument();
    expect(screen.getByText("Issues")).toBeInTheDocument();
  });

  it("does not render Users link for non-admin users", () => {
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: {
        id: 1,
        admin: false,
        firstname: "Regular",
        lastname: "User",
        mail: "user@test.com",
      },
    });
    render(<Header />);
    // Users link should not be in the main nav
    const usersLinks = screen.queryAllByRole("link", { name: "Users" });
    expect(usersLinks).toHaveLength(0);
  });

  it("renders Users link for admin users", () => {
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: { id: 1, admin: true, firstname: "Admin", lastname: "User", mail: "admin@test.com" },
    });
    render(<Header />);
    expect(screen.getByText("Users")).toBeInTheDocument();
  });

  it("highlights active navigation item", () => {
    mockPathname.mockReturnValue("/projects");
    render(<Header />);
    const projectsLink = screen.getByText("Projects");
    expect(projectsLink.closest("a")).toHaveClass("bg-accent");
  });

  it("calls toggle when sidebar toggle button is clicked", async () => {
    const user = userEvent.setup();
    render(<Header />);
    const toggleButton = screen.getByLabelText("Collapse sidebar");
    await user.click(toggleButton);
    expect(mockToggle).toHaveBeenCalled();
  });

  it("shows expand label when sidebar is collapsed", () => {
    (useSidebarStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      collapsed: true,
      toggle: mockToggle,
    });
    render(<Header />);
    expect(screen.getByLabelText("Expand sidebar")).toBeInTheDocument();
  });

  it("renders login button when user is not authenticated", () => {
    render(<Header />);
    expect(screen.getByText("Login")).toBeInTheDocument();
  });

  it("renders mobile menu button", () => {
    render(<Header />);
    expect(screen.getByLabelText("Toggle mobile menu")).toBeInTheDocument();
  });

  it("opens mobile menu when menu button is clicked", async () => {
    const user = userEvent.setup();
    render(<Header />);
    const menuButton = screen.getByLabelText("Toggle mobile menu");
    await user.click(menuButton);
    // After clicking, mobile nav should be visible - check for the mobile nav class
    const mobileNavs = document.querySelectorAll("nav.md\\:hidden");
    expect(mobileNavs.length).toBeGreaterThan(0);
  });
});
