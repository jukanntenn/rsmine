import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import {
  get,
  post,
  put,
  patch,
  del,
  upload,
  download,
  ApiClientError,
} from "./client";

// Mock fetch globally
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe("API Client", () => {
  beforeEach(() => {
    mockFetch.mockReset();
    localStorage.clear();
    // Set a base API URL for tests
    process.env.NEXT_PUBLIC_API_URL = "http://localhost:3000";
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  describe("get", () => {
    it("should make a GET request with correct headers", async () => {
      const mockData = { users: [{ id: 1, name: "Test" }] };
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve(mockData),
      });

      const result = await get("/api/v1/users");

      expect(mockFetch).toHaveBeenCalledTimes(1);
      const [url, options] = mockFetch.mock.calls[0];
      expect(url).toContain("/api/v1/users");
      expect(options.method).toBe("GET");
      expect(options.headers["Content-Type"]).toBe("application/json");
      expect(result).toEqual(mockData);
    });

    it("should include query parameters in URL", async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve({ data: [] }),
      });

      await get("/api/v1/users", { limit: 10, offset: 0 });

      const [url] = mockFetch.mock.calls[0];
      expect(url).toContain("limit=10");
      expect(url).toContain("offset=0");
    });

    it("should include auth token when available", async () => {
      localStorage.setItem("auth_token", "test-token-123");

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve({ data: [] }),
      });

      await get("/api/v1/users");

      const [, options] = mockFetch.mock.calls[0];
      expect(options.headers["Authorization"]).toBe("Bearer test-token-123");
    });
  });

  describe("post", () => {
    it("should make a POST request with JSON body", async () => {
      const requestBody = { username: "test", password: "pass" };
      const mockResponse = { token: "abc", user: { id: 1 } };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve(mockResponse),
      });

      const result = await post("/api/v1/auth/login", requestBody);

      const [, options] = mockFetch.mock.calls[0];
      expect(options.method).toBe("POST");
      expect(options.body).toBe(JSON.stringify(requestBody));
      expect(result).toEqual(mockResponse);
    });
  });

  describe("put", () => {
    it("should make a PUT request with JSON body", async () => {
      const requestBody = { name: "Updated Name" };
      const mockResponse = { user: { id: 1, name: "Updated Name" } };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve(mockResponse),
      });

      const result = await put("/api/v1/users/1", requestBody);

      const [, options] = mockFetch.mock.calls[0];
      expect(options.method).toBe("PUT");
      expect(options.body).toBe(JSON.stringify(requestBody));
      expect(result).toEqual(mockResponse);
    });
  });

  describe("patch", () => {
    it("should make a PATCH request with JSON body", async () => {
      const requestBody = { description: "New description" };
      const mockResponse = { attachment: { id: 1, description: "New description" } };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve(mockResponse),
      });

      const result = await patch("/api/v1/attachments/1", requestBody);

      const [, options] = mockFetch.mock.calls[0];
      expect(options.method).toBe("PATCH");
      expect(options.body).toBe(JSON.stringify(requestBody));
      expect(result).toEqual(mockResponse);
    });
  });

  describe("del", () => {
    it("should make a DELETE request", async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 204,
        headers: new Headers(),
      });

      const result = await del("/api/v1/users/1");

      const [, options] = mockFetch.mock.calls[0];
      expect(options.method).toBe("DELETE");
      expect(result).toBeUndefined();
    });

    it("should include query parameters in DELETE URL", async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 204,
        headers: new Headers(),
      });

      await del("/api/v1/issue_categories/1", { reassign_to: 2 });

      const [url] = mockFetch.mock.calls[0];
      expect(url).toContain("reassign_to=2");
    });
  });

  describe("upload", () => {
    it("should make a POST request with FormData", async () => {
      const formData = new FormData();
      formData.append("file", new Blob(["test content"]), "test.txt");

      const mockResponse = { upload: { token: "upload-token-123" } };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve(mockResponse),
      });

      const result = await upload("/api/v1/uploads", formData);

      const [, options] = mockFetch.mock.calls[0];
      expect(options.method).toBe("POST");
      expect(options.body).toBe(formData);
      // Should NOT have Content-Type header (let browser set it with boundary)
      expect(options.headers["Content-Type"]).toBeUndefined();
      expect(result).toEqual(mockResponse);
    });

    it("should include auth token for upload", async () => {
      localStorage.setItem("auth_token", "test-token-123");
      const formData = new FormData();

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve({}),
      });

      await upload("/api/v1/uploads", formData);

      const [, options] = mockFetch.mock.calls[0];
      expect(options.headers["Authorization"]).toBe("Bearer test-token-123");
    });
  });

  describe("download", () => {
    it("should make a GET request and return a Blob", async () => {
      const mockBlob = new Blob(["file content"], { type: "text/plain" });

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "text/plain" }),
        blob: () => Promise.resolve(mockBlob),
      });

      const result = await download("/api/v1/attachments/download/1");

      expect(result).toBe(mockBlob);
      const [, options] = mockFetch.mock.calls[0];
      expect(options.method).toBe("GET");
      expect(options.headers["Accept"]).toBe("*/*");
    });

    it("should include auth token for download", async () => {
      localStorage.setItem("auth_token", "test-token-123");

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "text/plain" }),
        blob: () => Promise.resolve(new Blob()),
      });

      await download("/api/v1/attachments/download/1");

      const [, options] = mockFetch.mock.calls[0];
      expect(options.headers["Authorization"]).toBe("Bearer test-token-123");
    });
  });

  describe("error handling", () => {
    it("should throw ApiClientError on HTTP error with JSON error response", async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 422,
        statusText: "Unprocessable Entity",
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve({ errors: ["Name is required", "Email is invalid"] }),
      });

      try {
        await get("/api/v1/users");
        // Should not reach here
        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error).toBeInstanceOf(ApiClientError);
        expect((error as ApiClientError).statusCode).toBe(422);
        expect((error as ApiClientError).message).toBe("Name is required, Email is invalid");
      }
    });

    it("should throw ApiClientError on HTTP error without JSON response", async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 500,
        statusText: "Internal Server Error",
        headers: new Headers({ "content-type": "text/html" }),
      });

      try {
        await get("/api/v1/users");
        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error).toBeInstanceOf(ApiClientError);
        expect((error as ApiClientError).statusCode).toBe(500);
        expect((error as ApiClientError).message).toBe("Internal Server Error");
      }
    });

    it("should throw ApiClientError on 401 Unauthorized", async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 401,
        statusText: "Unauthorized",
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve({ errors: ["Invalid token"] }),
      });

      try {
        await get("/api/v1/users");
        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error).toBeInstanceOf(ApiClientError);
        expect((error as ApiClientError).statusCode).toBe(401);
      }
    });

    it("should throw ApiClientError on download error with JSON response", async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 404,
        statusText: "Not Found",
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve({ errors: ["Attachment not found"] }),
      });

      try {
        await download("/api/v1/attachments/download/999");
        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error).toBeInstanceOf(ApiClientError);
        expect((error as ApiClientError).message).toBe("Attachment not found");
      }
    });

    it("should throw ApiClientError on download error without JSON response", async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 404,
        statusText: "Not Found",
        headers: new Headers({ "content-type": "text/html" }),
      });

      try {
        await download("/api/v1/attachments/download/999");
        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error).toBeInstanceOf(ApiClientError);
        expect((error as ApiClientError).statusCode).toBe(404);
      }
    });
  });

  describe("ApiClientError", () => {
    it("should create error with message and status code", () => {
      const error = new ApiClientError("Test error", 400);
      expect(error.message).toBe("Test error");
      expect(error.statusCode).toBe(400);
      expect(error.errorType).toBe("API_ERROR");
      expect(error.name).toBe("ApiClientError");
    });

    it("should create error with custom error type", () => {
      const error = new ApiClientError("Network error", 0, "NETWORK_ERROR");
      expect(error.errorType).toBe("NETWORK_ERROR");
    });
  });
});
