import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { UserNav } from "./user-nav";
import { useAuthStore } from "@/stores";

// Mock stores
vi.mock("@/stores", () => ({
  useAuthStore: vi.fn(),
}));

// Mock next/navigation
const mockPush = vi.fn();
vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

describe("UserNav", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders login button when user is not authenticated", () => {
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: null,
      logout: vi.fn(),
    });
    render(<UserNav />);
    expect(screen.getByText("Login")).toBeInTheDocument();
  });

  it("renders user avatar when authenticated", () => {
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: {
        id: 1,
        firstname: "John",
        lastname: "Doe",
        mail: "john@example.com",
        admin: false,
      },
      logout: vi.fn(),
    });
    render(<UserNav />);
    // Avatar shows initials - Avatar component uses the fallback prop
    const avatarButton = screen.getByRole("button");
    expect(avatarButton).toBeInTheDocument();
  });

  it("shows user dropdown when clicking avatar", async () => {
    const user = userEvent.setup();
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: {
        id: 1,
        firstname: "John",
        lastname: "Doe",
        mail: "john@example.com",
        admin: false,
      },
      logout: vi.fn(),
    });
    render(<UserNav />);
    
    const avatarButton = screen.getByRole("button");
    await user.click(avatarButton);
    
    expect(screen.getByText("John Doe")).toBeInTheDocument();
    expect(screen.getByText("john@example.com")).toBeInTheDocument();
    expect(screen.getByText("Profile")).toBeInTheDocument();
  });

  it("shows settings link for admin users", async () => {
    const user = userEvent.setup();
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: {
        id: 1,
        firstname: "Admin",
        lastname: "User",
        mail: "admin@example.com",
        admin: true,
      },
      logout: vi.fn(),
    });
    render(<UserNav />);
    
    const avatarButton = screen.getByRole("button");
    await user.click(avatarButton);
    
    expect(screen.getByText("Settings")).toBeInTheDocument();
  });

  it("does not show settings link for non-admin users", async () => {
    const user = userEvent.setup();
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: {
        id: 1,
        firstname: "John",
        lastname: "Doe",
        mail: "john@example.com",
        admin: false,
      },
      logout: vi.fn(),
    });
    render(<UserNav />);
    
    const avatarButton = screen.getByRole("button");
    await user.click(avatarButton);
    
    expect(screen.queryByText("Settings")).not.toBeInTheDocument();
  });

  it("calls logout and redirects when logout is clicked", async () => {
    const mockLogout = vi.fn();
    const user = userEvent.setup();
    (useAuthStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      user: {
        id: 1,
        firstname: "John",
        lastname: "Doe",
        mail: "john@example.com",
        admin: false,
      },
      logout: mockLogout,
    });
    render(<UserNav />);
    
    const avatarButton = screen.getByRole("button");
    await user.click(avatarButton);
    
    const logoutButton = screen.getByText("Log out");
    await user.click(logoutButton);
    
    expect(mockLogout).toHaveBeenCalled();
    expect(mockPush).toHaveBeenCalledWith("/login");
  });
});
