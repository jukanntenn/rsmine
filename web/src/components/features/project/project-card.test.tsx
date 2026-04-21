import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { ProjectCard } from "./project-card";
import type { ProjectListItem } from "@/types";

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

const mockProject: ProjectListItem = {
  id: 1,
  name: "Test Project",
  identifier: "test-project",
  description: "A test project description",
  homepage: null,
  is_public: true,
  parent_id: null,
  status: 1,
  lft: 1,
  rgt: 2,
  inherit_members: false,
  created_on: "2024-01-01T00:00:00Z",
  updated_on: "2024-01-02T00:00:00Z",
};

const mockPrivateProject: ProjectListItem = {
  ...mockProject,
  id: 2,
  name: "Private Project",
  identifier: "private-project",
  is_public: false,
};

const mockProjectWithParent: ProjectListItem = {
  ...mockProject,
  id: 3,
  name: "Child Project",
  identifier: "child-project",
  parent_id: 1,
  parent: { id: 1, name: "Parent Project" },
};

const mockProjectWithIssueCount: ProjectListItem = {
  ...mockProject,
  id: 4,
  issue_count: 42,
};

describe("ProjectCard", () => {
  describe("rendering", () => {
    it("should render project name", () => {
      render(<ProjectCard project={mockProject} />);
      expect(screen.getByText("Test Project")).toBeInTheDocument();
    });

    it("should render project identifier", () => {
      render(<ProjectCard project={mockProject} />);
      expect(screen.getByText("test-project")).toBeInTheDocument();
    });

    it("should render project description", () => {
      render(<ProjectCard project={mockProject} />);
      expect(screen.getByText("A test project description")).toBeInTheDocument();
    });

    it("should render public badge for public projects", () => {
      render(<ProjectCard project={mockProject} />);
      expect(screen.getByText("Public")).toBeInTheDocument();
    });

    it("should render private badge for private projects", () => {
      render(<ProjectCard project={mockPrivateProject} />);
      expect(screen.getByText("Private")).toBeInTheDocument();
    });

    it("should render active status badge", () => {
      render(<ProjectCard project={mockProject} />);
      expect(screen.getByText("Active")).toBeInTheDocument();
    });

    it("should link to project detail page", () => {
      render(<ProjectCard project={mockProject} />);
      const link = screen.getByTestId("mock-link");
      expect(link).toHaveAttribute("href", "/projects/1");
    });
  });

  describe("with parent project", () => {
    it("should render parent project name", () => {
      render(<ProjectCard project={mockProjectWithParent} />);
      expect(screen.getByText(/Parent: Parent Project/)).toBeInTheDocument();
    });
  });

  describe("with issue count", () => {
    it("should render issue count when greater than 0", () => {
      render(<ProjectCard project={mockProjectWithIssueCount} />);
      expect(screen.getByText("42 issues")).toBeInTheDocument();
    });
  });

  describe("without description", () => {
    it("should not render description element when description is null", () => {
      const projectNoDescription = { ...mockProject, description: null };
      render(<ProjectCard project={projectNoDescription} />);
      // The description line-clamp-2 paragraph should not exist
      const descriptionElement = screen.queryByText(/A test project description/);
      expect(descriptionElement).not.toBeInTheDocument();
    });
  });

  describe("accessibility", () => {
    it("should be a clickable card", () => {
      render(<ProjectCard project={mockProject} />);
      const link = screen.getByTestId("mock-link");
      expect(link).toBeInTheDocument();
    });
  });
});
