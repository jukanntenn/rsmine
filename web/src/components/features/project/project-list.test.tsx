import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ProjectList } from "./project-list";
import { projectsApi } from "@/lib/api";
import type { ProjectListItem } from "@/types";

// Mock the projects API
vi.mock("@/lib/api", () => ({
  projectsApi: {
    list: vi.fn(),
  },
}));

// Mock Next.js Link
vi.mock("next/link", () => ({
  default: ({
    children,
    href,
  }: {
    children: React.ReactNode;
    href: string;
  }) => (
    <a href={href} data-testid="mock-link">
      {children}
    </a>
  ),
}));

const mockProjectsApi = vi.mocked(projectsApi);

const createTestQueryClient = () =>
  new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
        gcTime: 0,
        staleTime: 0,
      },
    },
  });

const mockProjects: ProjectListItem[] = [
  {
    id: 1,
    name: "Project Alpha",
    identifier: "project-alpha",
    description: "First test project",
    homepage: null,
    is_public: true,
    parent_id: null,
    status: 1,
    lft: 1,
    rgt: 2,
    inherit_members: false,
    created_on: "2024-01-01T00:00:00Z",
    updated_on: "2024-01-02T00:00:00Z",
    issue_count: 10,
  },
  {
    id: 2,
    name: "Project Beta",
    identifier: "project-beta",
    description: "Second test project",
    homepage: null,
    is_public: false,
    parent_id: null,
    status: 1,
    lft: 3,
    rgt: 4,
    inherit_members: false,
    created_on: "2024-01-01T00:00:00Z",
    updated_on: "2024-01-02T00:00:00Z",
    issue_count: 5,
  },
  {
    id: 3,
    name: "Project Gamma",
    identifier: "project-gamma",
    description: "A project with a parent",
    homepage: null,
    is_public: true,
    parent_id: 1,
    parent: { id: 1, name: "Project Alpha" },
    status: 1,
    lft: 5,
    rgt: 6,
    inherit_members: false,
    created_on: "2024-01-01T00:00:00Z",
    updated_on: "2024-01-02T00:00:00Z",
    issue_count: 2,
  },
];

const mockProjectsResponse = {
  projects: mockProjects,
  total_count: 3,
  offset: 0,
  limit: 12,
};

const renderWithQueryClient = (ui: React.ReactElement) => {
  const queryClient = createTestQueryClient();
  return render(
    <QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>
  );
};

describe("ProjectList", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockProjectsApi.list.mockResolvedValue(mockProjectsResponse);
  });

  describe("loading state", () => {
    it("should show loading skeletons while loading", async () => {
      mockProjectsApi.list.mockImplementation(
        () => new Promise((resolve) => setTimeout(() => resolve(mockProjectsResponse), 100))
      );

      renderWithQueryClient(<ProjectList />);

      // Check for loading skeletons
      const skeletons = screen.getAllByRole("generic").filter((el) =>
        el.className.includes("animate-pulse") || el.className.includes("bg-muted")
      );
      expect(skeletons.length).toBeGreaterThan(0);
    });
  });

  describe("successful rendering", () => {
    it("should render all projects", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Project Alpha")).toBeInTheDocument();
      });

      expect(screen.getByText("Project Beta")).toBeInTheDocument();
      expect(screen.getByText("Project Gamma")).toBeInTheDocument();
    });

    it("should render project identifiers", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("project-alpha")).toBeInTheDocument();
      });

      expect(screen.getByText("project-beta")).toBeInTheDocument();
      expect(screen.getByText("project-gamma")).toBeInTheDocument();
    });

    it("should render public/private badges correctly", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Project Alpha")).toBeInTheDocument();
      });

      const publicBadges = screen.getAllByText("Public");
      const privateBadges = screen.getAllByText("Private");

      expect(publicBadges.length).toBe(2); // Alpha and Gamma
      expect(privateBadges.length).toBe(1); // Beta
    });

    it("should render issue counts", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("10 issues")).toBeInTheDocument();
      });

      expect(screen.getByText("5 issues")).toBeInTheDocument();
      expect(screen.getByText("2 issues")).toBeInTheDocument();
    });

    it("should render parent project reference", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText(/Parent: Project Alpha/)).toBeInTheDocument();
      });
    });
  });

  describe("search functionality", () => {
    it("should render search input", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByPlaceholderText("Search projects...")).toBeInTheDocument();
      });
    });

    it("should filter projects by search query", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Project Alpha")).toBeInTheDocument();
      });

      const searchInput = screen.getByPlaceholderText("Search projects...");

      // Search for "Alpha"
      fireEvent.change(searchInput, { target: { value: "Alpha" } });

      // Wait for debounce
      await waitFor(
        () => {
          expect(screen.getByText("Project Alpha")).toBeInTheDocument();
          expect(screen.queryByText("Project Beta")).not.toBeInTheDocument();
        },
        { timeout: 500 }
      );
    });

    it("should filter projects by identifier", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Project Alpha")).toBeInTheDocument();
      });

      const searchInput = screen.getByPlaceholderText("Search projects...");

      fireEvent.change(searchInput, { target: { value: "beta" } });

      await waitFor(
        () => {
          expect(screen.getByText("Project Beta")).toBeInTheDocument();
          expect(screen.queryByText("Project Alpha")).not.toBeInTheDocument();
        },
        { timeout: 500 }
      );
    });

    it("should show empty state when search has no results", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Project Alpha")).toBeInTheDocument();
      });

      const searchInput = screen.getByPlaceholderText("Search projects...");

      fireEvent.change(searchInput, { target: { value: "nonexistent" } });

      await waitFor(
        () => {
          expect(screen.getByText("No projects found")).toBeInTheDocument();
        },
        { timeout: 500 }
      );
    });
  });

  describe("empty state", () => {
    it("should show empty state when no projects", async () => {
      mockProjectsApi.list.mockResolvedValue({
        projects: [],
        total_count: 0,
        offset: 0,
        limit: 12,
      });

      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("No projects found")).toBeInTheDocument();
      });

      expect(
        screen.getByText("Get started by creating your first project")
      ).toBeInTheDocument();
    });

    it("should show create button in empty state", async () => {
      mockProjectsApi.list.mockResolvedValue({
        projects: [],
        total_count: 0,
        offset: 0,
        limit: 12,
      });

      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Create Project")).toBeInTheDocument();
      });
    });
  });

  describe("new project button", () => {
    it("should render New Project button", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("New Project")).toBeInTheDocument();
      });
    });

    it("should link to /projects/new", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        const link = screen.getByRole("link", { name: /new project/i });
        expect(link).toHaveAttribute("href", "/projects/new");
      });
    });
  });

  describe("error state", () => {
    it("should show error state when API fails", async () => {
      mockProjectsApi.list.mockRejectedValue(new Error("Network error"));

      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Failed to load projects")).toBeInTheDocument();
      });
    });

    it("should show try again button on error", async () => {
      mockProjectsApi.list.mockRejectedValue(new Error("Network error"));

      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Try Again")).toBeInTheDocument();
      });
    });

    it("should refetch when try again is clicked", async () => {
      mockProjectsApi.list.mockRejectedValueOnce(new Error("Network error"));
      mockProjectsApi.list.mockResolvedValueOnce(mockProjectsResponse);

      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Try Again")).toBeInTheDocument();
      });

      fireEvent.click(screen.getByText("Try Again"));

      await waitFor(() => {
        expect(mockProjectsApi.list).toHaveBeenCalledTimes(2);
      });
    });
  });

  describe("pagination", () => {
    it("should show pagination when there are multiple pages", async () => {
      const manyProjects: ProjectListItem[] = Array.from({ length: 15 }, (_, i) => ({
        id: i + 1,
        name: `Project ${i + 1}`,
        identifier: `project-${i + 1}`,
        description: `Description ${i + 1}`,
        homepage: null,
        is_public: true,
        parent_id: null,
        status: 1 as const,
        lft: i * 2 + 1,
        rgt: i * 2 + 2,
        inherit_members: false,
        created_on: "2024-01-01T00:00:00Z",
        updated_on: "2024-01-02T00:00:00Z",
        issue_count: 0,
      }));

      mockProjectsApi.list.mockResolvedValue({
        projects: manyProjects.slice(0, 12),
        total_count: 15,
        offset: 0,
        limit: 12,
      });

      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Project 1")).toBeInTheDocument();
      });

      // Check pagination info
      await waitFor(() => {
        expect(screen.getByText(/Showing 1 - 12 of 15 projects/)).toBeInTheDocument();
      });
    });

    it("should not show pagination for single page", async () => {
      mockProjectsApi.list.mockResolvedValue({
        projects: mockProjects.slice(0, 1),
        total_count: 1,
        offset: 0,
        limit: 12,
      });

      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(screen.getByText("Project Alpha")).toBeInTheDocument();
      });

      expect(screen.queryByText("Previous")).not.toBeInTheDocument();
    });
  });

  describe("API calls", () => {
    it("should call projectsApi.list with correct params", async () => {
      renderWithQueryClient(<ProjectList />);

      await waitFor(() => {
        expect(mockProjectsApi.list).toHaveBeenCalledWith({
          offset: 0,
          limit: 12,
          status: undefined,
        });
      });
    });

    it("should call API with status filter when provided", async () => {
      renderWithQueryClient(<ProjectList statusFilter={1} />);

      await waitFor(() => {
        expect(mockProjectsApi.list).toHaveBeenCalledWith({
          offset: 0,
          limit: 12,
          status: 1,
        });
      });
    });
  });
});
