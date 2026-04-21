import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { ProjectStatusBadge } from "./project-status-badge";
import type { ProjectStatus } from "@/types";

describe("ProjectStatusBadge", () => {
  describe("rendering", () => {
    it("should render 'Active' for status 1", () => {
      render(<ProjectStatusBadge status={1 as ProjectStatus} />);
      expect(screen.getByText("Active")).toBeInTheDocument();
    });

    it("should render 'Closed' for status 5", () => {
      render(<ProjectStatusBadge status={5 as ProjectStatus} />);
      expect(screen.getByText("Closed")).toBeInTheDocument();
    });

    it("should render 'Archived' for status 9", () => {
      render(<ProjectStatusBadge status={9 as ProjectStatus} />);
      expect(screen.getByText("Archived")).toBeInTheDocument();
    });
  });

  describe("styling", () => {
    it("should have success variant for active status", () => {
      const { container } = render(<ProjectStatusBadge status={1 as ProjectStatus} />);
      const badge = container.querySelector(".bg-green-500");
      expect(badge).toBeInTheDocument();
    });

    it("should have warning variant for closed status", () => {
      const { container } = render(<ProjectStatusBadge status={5 as ProjectStatus} />);
      const badge = container.querySelector(".bg-yellow-500");
      expect(badge).toBeInTheDocument();
    });

    it("should have secondary variant for archived status", () => {
      const { container } = render(<ProjectStatusBadge status={9 as ProjectStatus} />);
      const badge = container.querySelector(".bg-secondary");
      expect(badge).toBeInTheDocument();
    });
  });

  describe("className prop", () => {
    it("should apply custom className", () => {
      const { container } = render(
        <ProjectStatusBadge status={1 as ProjectStatus} className="custom-class" />
      );
      const badge = container.querySelector(".custom-class");
      expect(badge).toBeInTheDocument();
    });
  });
});
