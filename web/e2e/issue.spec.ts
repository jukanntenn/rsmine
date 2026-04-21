import { test, expect } from "@playwright/test";
import { createIssueInProject, findProjectWithTrackers, seedAdminSession } from "./helpers";

function uniqueSuffix() {
  return `${Date.now()}-${Math.floor(Math.random() * 10000)}`;
}

test.describe("Issue flow", () => {
  test.beforeEach(async ({ page }) => {
    await seedAdminSession(page);
  });

  test("create and view issue details", async ({ page }) => {
    const suffix = uniqueSuffix();
    const projectId = await findProjectWithTrackers(page);
    const issueId = await createIssueInProject(page, projectId, `Issue ${suffix}`);
    await expect(page.locator("h1")).toContainText(`#${issueId}`);
    await expect(page.getByText(`Issue ${suffix}`)).toBeVisible();
    await expect(page.getByRole("heading", { name: "Description" })).toBeVisible();
  });

  test("update issue with notes", async ({ page }) => {
    const suffix = uniqueSuffix();
    const projectId = await findProjectWithTrackers(page);
    const issueId = await createIssueInProject(page, projectId, `Issue Update ${suffix}`);
    const token = await page.evaluate(() => window.localStorage.getItem("auth_token"));
    const updateResponse = await page.request.put(`/api/v1/issues/${issueId}`, {
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      data: {
        issue: {
          notes: `Update note ${suffix}`,
        },
      },
    });
    expect(updateResponse.ok()).toBeTruthy();
    await page.goto(`/issues/${issueId}`);
    await expect(page.getByText(`Update note ${suffix}`)).toBeVisible();
  });

  test("assign issue to member and show in sidebar", async ({ page }) => {
    const suffix = uniqueSuffix();
    const projectId = await findProjectWithTrackers(page);
    const issueId = await createIssueInProject(page, projectId, `Assign Issue ${suffix}`);
    const token = await page.evaluate(() => window.localStorage.getItem("auth_token"));
    const me = await page.request.get("/api/v1/auth/me", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    expect(me.ok()).toBeTruthy();
    const meBody = await me.json();
    const assignResponse = await page.request.put(`/api/v1/issues/${issueId}`, {
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      data: {
        issue: {
          assigned_to_id: meBody.user.id,
        },
      },
    });
    expect(assignResponse.ok()).toBeTruthy();
    await page.goto(`/issues/${issueId}`);
    await expect(page.getByText("Assignee")).toBeVisible();
  });

  test("search issues in list", async ({ page }) => {
    const suffix = uniqueSuffix();
    const projectId = await findProjectWithTrackers(page);
    await createIssueInProject(page, projectId, `Searchable Issue ${suffix}`);
    await page.goto(`/projects/${projectId}`);
    await page.getByPlaceholder("Search issues...").fill(`Searchable Issue ${suffix}`);
    await expect(page.getByText(`Searchable Issue ${suffix}`)).toBeVisible();
  });

  test("display relations between issues", async ({ page }) => {
    const suffix = uniqueSuffix();
    const projectId = await findProjectWithTrackers(page);
    const issueA = await createIssueInProject(page, projectId, `Issue A ${suffix}`);
    const issueB = await createIssueInProject(page, projectId, `Issue B ${suffix}`);
    const token = await page.evaluate(() => window.localStorage.getItem("auth_token"));
    if (!token) {
      throw new Error("Missing auth token");
    }
    const response = await page.request.post(`/api/v1/issues/${issueA}/relations`, {
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      data: {
        relation: {
          issue_to_id: issueB,
          relation_type: "relates",
        },
      },
    });
    expect(response.ok()).toBeTruthy();
    await page.goto(`/issues/${issueA}`);
    await expect(page.getByText("Relations")).toBeVisible();
    await expect(page.getByRole("link", { name: `#${issueB}` })).toBeVisible();
  });
});
