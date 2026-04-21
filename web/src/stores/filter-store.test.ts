import { describe, it, expect, beforeEach } from "vitest";
import { useFilterStore } from "./filter-store";

describe("filter-store", () => {
  beforeEach(() => {
    // Reset store state before each test
    useFilterStore.setState({
      search: "",
      page: 1,
      perPage: 25,
      sortBy: "created_on",
      sortOrder: "desc",
      issueFilters: {},
    });
  });

  describe("initial state", () => {
    it("should have correct default values", () => {
      const state = useFilterStore.getState();

      expect(state.search).toBe("");
      expect(state.page).toBe(1);
      expect(state.perPage).toBe(25);
      expect(state.sortBy).toBe("created_on");
      expect(state.sortOrder).toBe("desc");
      expect(state.issueFilters).toEqual({});
    });
  });

  describe("setSearch", () => {
    it("should update search and reset to page 1", () => {
      useFilterStore.getState().setPage(5);
      useFilterStore.getState().setSearch("test query");
      const state = useFilterStore.getState();

      expect(state.search).toBe("test query");
      expect(state.page).toBe(1);
    });
  });

  describe("setPage", () => {
    it("should update page number", () => {
      useFilterStore.getState().setPage(3);

      expect(useFilterStore.getState().page).toBe(3);
    });

    it("should allow setting page to any number", () => {
      useFilterStore.getState().setPage(100);

      expect(useFilterStore.getState().page).toBe(100);
    });
  });

  describe("setPerPage", () => {
    it("should update perPage and reset to page 1", () => {
      useFilterStore.getState().setPage(5);
      useFilterStore.getState().setPerPage(50);
      const state = useFilterStore.getState();

      expect(state.perPage).toBe(50);
      expect(state.page).toBe(1);
    });
  });

  describe("setSort", () => {
    it("should update sortBy and sortOrder", () => {
      useFilterStore.getState().setSort("name", "asc");
      const state = useFilterStore.getState();

      expect(state.sortBy).toBe("name");
      expect(state.sortOrder).toBe("asc");
    });

    it("should allow changing only sort order", () => {
      useFilterStore.getState().setSort("created_on", "asc");
      const state = useFilterStore.getState();

      expect(state.sortBy).toBe("created_on");
      expect(state.sortOrder).toBe("asc");
    });
  });

  describe("reset", () => {
    it("should reset all state to initial values", () => {
      useFilterStore.getState().setSearch("test");
      useFilterStore.getState().setPage(5);
      useFilterStore.getState().setPerPage(100);
      useFilterStore.getState().setSort("name", "asc");
      useFilterStore.getState().setIssueFilter("project_id", 1);

      useFilterStore.getState().reset();
      const state = useFilterStore.getState();

      expect(state.search).toBe("");
      expect(state.page).toBe(1);
      expect(state.perPage).toBe(25);
      expect(state.sortBy).toBe("created_on");
      expect(state.sortOrder).toBe("desc");
      expect(state.issueFilters).toEqual({});
    });
  });

  describe("setIssueFilter", () => {
    it("should set project_id filter", () => {
      useFilterStore.getState().setIssueFilter("project_id", 42);

      expect(useFilterStore.getState().issueFilters.project_id).toBe(42);
    });

    it("should set status_id filter with special values", () => {
      useFilterStore.getState().setIssueFilter("status_id", "open");

      expect(useFilterStore.getState().issueFilters.status_id).toBe("open");
    });

    it("should set assigned_to_id filter with 'me' value", () => {
      useFilterStore.getState().setIssueFilter("assigned_to_id", "me");

      expect(useFilterStore.getState().issueFilters.assigned_to_id).toBe("me");
    });

    it("should set multiple filters", () => {
      useFilterStore.getState().setIssueFilter("project_id", 1);
      useFilterStore.getState().setIssueFilter("tracker_id", 2);
      useFilterStore.getState().setIssueFilter("priority_id", 3);
      const state = useFilterStore.getState();

      expect(state.issueFilters).toEqual({
        project_id: 1,
        tracker_id: 2,
        priority_id: 3,
      });
    });

    it("should reset page to 1 when filter changes", () => {
      useFilterStore.getState().setPage(10);
      expect(useFilterStore.getState().page).toBe(10);

      useFilterStore.getState().setIssueFilter("project_id", 1);

      expect(useFilterStore.getState().page).toBe(1);
    });
  });

  describe("clearIssueFilters", () => {
    it("should clear all issue filters", () => {
      useFilterStore.getState().setIssueFilter("project_id", 1);
      useFilterStore.getState().setIssueFilter("status_id", "open");
      useFilterStore.getState().setIssueFilter("tracker_id", 3);

      expect(useFilterStore.getState().issueFilters).toEqual({
        project_id: 1,
        status_id: "open",
        tracker_id: 3,
      });

      useFilterStore.getState().clearIssueFilters();

      expect(useFilterStore.getState().issueFilters).toEqual({});
    });

    it("should reset page to 1 when clearing filters", () => {
      useFilterStore.getState().setPage(5);
      useFilterStore.getState().setIssueFilter("project_id", 1);

      useFilterStore.getState().clearIssueFilters();

      expect(useFilterStore.getState().page).toBe(1);
    });
  });
});