import { create } from "zustand";

/**
 * Issue-specific filter options
 */
export interface IssueFilters {
  project_id?: number;
  status_id?: number | "open" | "closed";
  tracker_id?: number;
  priority_id?: number;
  assigned_to_id?: number | "me";
  author_id?: number;
  subject?: string;
}

interface FilterState {
  // Common filter state
  search: string;
  page: number;
  perPage: number;
  sortBy: string;
  sortOrder: "asc" | "desc";

  // Issue-specific filters
  issueFilters: IssueFilters;

  // Actions - common
  setSearch: (search: string) => void;
  setPage: (page: number) => void;
  setPerPage: (perPage: number) => void;
  setSort: (sortBy: string, sortOrder: "asc" | "desc") => void;
  reset: () => void;

  // Actions - issue filters
  setIssueFilter: <K extends keyof IssueFilters>(
    key: K,
    value: IssueFilters[K]
  ) => void;
  clearIssueFilters: () => void;
}

const initialIssueFilters: IssueFilters = {};

const initialState = {
  search: "",
  page: 1,
  perPage: 25,
  sortBy: "created_on",
  sortOrder: "desc" as const,
  issueFilters: initialIssueFilters,
};

export const useFilterStore = create<FilterState>((set) => ({
  ...initialState,
  setSearch: (search) => set({ search, page: 1 }),
  setPage: (page) => set({ page }),
  setPerPage: (perPage) => set({ perPage, page: 1 }),
  setSort: (sortBy, sortOrder) => set({ sortBy, sortOrder }),
  reset: () => set(initialState),
  setIssueFilter: (key, value) =>
    set((state) => ({
      issueFilters: { ...state.issueFilters, [key]: value },
      page: 1, // Reset to first page when filter changes
    })),
  clearIssueFilters: () =>
    set({
      issueFilters: initialIssueFilters,
      page: 1,
    }),
}));
