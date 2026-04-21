import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { Layout } from "./layout";
import { useSidebarStore } from "@/stores";

// Mock stores
vi.mock("@/stores", () => ({
  useSidebarStore: vi.fn(),
  useAuthStore: vi.fn(() => ({ user: null })),
}));

// Mock next/navigation
vi.mock("next/navigation", () => ({
  usePathname: () => "/",
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe("Layout", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (useSidebarStore as unknown as ReturnType<typeof vi.fn>).mockReturnValue({
      collapsed: false,
    });
  });

  it("renders children correctly", () => {
    render(
      <Layout>
        <div data-testid="child">Test Content</div>
      </Layout>
    );
    expect(screen.getByTestId("child")).toBeInTheDocument();
    expect(screen.getByText("Test Content")).toBeInTheDocument();
  });

  it("renders header by default", () => {
    render(<Layout>Content</Layout>);
    expect(screen.getByText("Rsmine")).toBeInTheDocument();
  });

  it("renders sidebar by default", () => {
    const { container } = render(<Layout>Content</Layout>);
    // Check for aside element which is the sidebar
    const aside = container.querySelector("aside");
    expect(aside).toBeInTheDocument();
    expect(aside).toHaveClass("bg-sidebar");
  });

  it("does not render header when noHeader is true", () => {
    render(<Layout noHeader>Content</Layout>);
    expect(screen.queryByText("Rsmine")).not.toBeInTheDocument();
  });

  it("does not render sidebar when noSidebar is true", () => {
    const { container } = render(<Layout noSidebar>Content</Layout>);
    const aside = container.querySelector("aside");
    expect(aside).not.toBeInTheDocument();
  });

  it("applies custom className to main content area", () => {
    const { container } = render(
      <Layout className="custom-class">
        Content
      </Layout>
    );
    const main = container.querySelector("main");
    expect(main).toHaveClass("custom-class");
  });

  it("applies padding when sidebar is shown", () => {
    const { container } = render(<Layout>Content</Layout>);
    const main = container.querySelector("main");
    expect(main).toHaveClass("p-6");
  });

  it("does not apply sidebar padding when noSidebar is true", () => {
    const { container } = render(<Layout noSidebar>Content</Layout>);
    const main = container.querySelector("main");
    expect(main).not.toHaveClass("p-6");
  });
});
