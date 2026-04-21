import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { trackersApi, statusesApi, prioritiesApi, rolesApi } from "./enums";

// Mock the client module
vi.mock("./client", () => ({
  get: vi.fn(),
  post: vi.fn(),
  put: vi.fn(),
  del: vi.fn(),
}));

import { get, post, put, del } from "./client";

const mockGet = vi.mocked(get);
const mockPost = vi.mocked(post);
const mockPut = vi.mocked(put);
const mockDel = vi.mocked(del);

describe("trackersApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("list", () => {
    it("should call GET /api/v1/trackers", async () => {
      const mockResponse = {
        trackers: [
          { id: 1, name: "Bug", description: null },
          { id: 2, name: "Feature", description: null },
        ],
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await trackersApi.list();

      expect(mockGet).toHaveBeenCalledTimes(1);
      expect(mockGet).toHaveBeenCalledWith("/api/v1/trackers");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("get", () => {
    it("should call GET /api/v1/trackers/:id", async () => {
      const mockResponse = {
        tracker: { id: 1, name: "Bug", description: "Bug reports" },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await trackersApi.get(1);

      expect(mockGet).toHaveBeenCalledWith("/api/v1/trackers/1");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("create", () => {
    it("should call POST /api/v1/trackers", async () => {
      const createData = { name: "Support" };

      const mockResponse = {
        tracker: { id: 3, name: "Support", description: null },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await trackersApi.create(createData);

      expect(mockPost).toHaveBeenCalledWith("/api/v1/trackers", {
        tracker: createData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("update", () => {
    it("should call PUT /api/v1/trackers/:id", async () => {
      const updateData = { name: "Bugs" };

      const mockResponse = {
        tracker: { id: 1, name: "Bugs", description: null },
      };

      mockPut.mockResolvedValueOnce(mockResponse);

      const result = await trackersApi.update(1, updateData);

      expect(mockPut).toHaveBeenCalledWith("/api/v1/trackers/1", {
        tracker: updateData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("delete", () => {
    it("should call DELETE /api/v1/trackers/:id", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await trackersApi.delete(1);

      expect(mockDel).toHaveBeenCalledWith("/api/v1/trackers/1");
    });
  });
});

describe("statusesApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("list", () => {
    it("should call GET /api/v1/issue_statuses", async () => {
      const mockResponse = {
        issue_statuses: [
          { id: 1, name: "New", is_closed: false, description: null },
          { id: 2, name: "In Progress", is_closed: false, description: null },
          { id: 5, name: "Closed", is_closed: true, description: null },
        ],
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await statusesApi.list();

      expect(mockGet).toHaveBeenCalledWith("/api/v1/issue_statuses");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("get", () => {
    it("should call GET /api/v1/issue_statuses/:id", async () => {
      const mockResponse = {
        issue_status: { id: 1, name: "New", is_closed: false, description: null },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await statusesApi.get(1);

      expect(mockGet).toHaveBeenCalledWith("/api/v1/issue_statuses/1");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("create", () => {
    it("should call POST /api/v1/issue_statuses", async () => {
      const createData = { name: "On Hold", is_closed: false };

      const mockResponse = {
        issue_status: { id: 7, name: "On Hold", is_closed: false, description: null },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await statusesApi.create(createData);

      expect(mockPost).toHaveBeenCalledWith("/api/v1/issue_statuses", {
        issue_status: createData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("update", () => {
    it("should call PUT /api/v1/issue_statuses/:id", async () => {
      const updateData = { name: "Blocked" };

      const mockResponse = {
        issue_status: { id: 7, name: "Blocked", is_closed: false, description: null },
      };

      mockPut.mockResolvedValueOnce(mockResponse);

      const result = await statusesApi.update(7, updateData);

      expect(mockPut).toHaveBeenCalledWith("/api/v1/issue_statuses/7", {
        issue_status: updateData,
      });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("delete", () => {
    it("should call DELETE /api/v1/issue_statuses/:id", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await statusesApi.delete(7);

      expect(mockDel).toHaveBeenCalledWith("/api/v1/issue_statuses/7");
    });
  });
});

describe("prioritiesApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("list", () => {
    it("should call GET /api/v1/enumerations/issue_priorities", async () => {
      const mockResponse = {
        issue_priorities: [
          { id: 1, name: "Low", is_default: false, active: true },
          { id: 2, name: "Normal", is_default: true, active: true },
          { id: 3, name: "High", is_default: false, active: true },
        ],
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await prioritiesApi.list();

      expect(mockGet).toHaveBeenCalledWith(
        "/api/v1/enumerations/issue_priorities"
      );
      expect(result).toEqual(mockResponse);
    });
  });
});

describe("rolesApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("list", () => {
    it("should call GET /api/v1/roles", async () => {
      const mockResponse = {
        roles: [
          { id: 1, name: "Manager" },
          { id: 2, name: "Developer" },
          { id: 3, name: "Reporter" },
        ],
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await rolesApi.list();

      expect(mockGet).toHaveBeenCalledWith("/api/v1/roles");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("get", () => {
    it("should call GET /api/v1/roles/:id", async () => {
      const mockResponse = {
        role: {
          id: 1,
          name: "Manager",
          position: 1,
          assignable: true,
          builtin: 0,
          permissions: ["view_issues", "edit_issues"],
          issues_visibility: "all",
          users_visibility: "all",
          time_entries_visibility: "all",
          all_roles_managed: true,
          settings: {},
        },
      };

      mockGet.mockResolvedValueOnce(mockResponse);

      const result = await rolesApi.get(1);

      expect(mockGet).toHaveBeenCalledWith("/api/v1/roles/1");
      expect(result).toEqual(mockResponse);
    });
  });

  describe("create", () => {
    it("should call POST /api/v1/roles", async () => {
      const createData = { name: "QA", permissions: ["view_issues"] };

      const mockResponse = {
        role: { id: 4, name: "QA", position: 4, assignable: true, builtin: 0, permissions: ["view_issues"], issues_visibility: "default", users_visibility: "default", time_entries_visibility: "default", all_roles_managed: true, settings: {} },
      };

      mockPost.mockResolvedValueOnce(mockResponse);

      const result = await rolesApi.create(createData);

      expect(mockPost).toHaveBeenCalledWith("/api/v1/roles", { role: createData });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("update", () => {
    it("should call PUT /api/v1/roles/:id", async () => {
      const updateData = { name: "Quality Assurance" };

      const mockResponse = {
        role: { id: 4, name: "Quality Assurance", position: 4, assignable: true, builtin: 0, permissions: [], issues_visibility: "default", users_visibility: "default", time_entries_visibility: "default", all_roles_managed: true, settings: {} },
      };

      mockPut.mockResolvedValueOnce(mockResponse);

      const result = await rolesApi.update(4, updateData);

      expect(mockPut).toHaveBeenCalledWith("/api/v1/roles/4", { role: updateData });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("delete", () => {
    it("should call DELETE /api/v1/roles/:id", async () => {
      mockDel.mockResolvedValueOnce(undefined);

      await rolesApi.delete(4);

      expect(mockDel).toHaveBeenCalledWith("/api/v1/roles/4");
    });
  });
});
