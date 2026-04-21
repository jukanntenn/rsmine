import { test, expect } from "@playwright/test";
import { createProject, seedAdminSession } from "./helpers";

function uniqueSuffix() {
  return `${Date.now()}-${Math.floor(Math.random() * 10000)}`;
}

async function fillStable(
  page: import("@playwright/test").Page,
  selector: string,
  value: string
) {
  for (let i = 0; i < 5; i += 1) {
    try {
      await page.locator(selector).fill(value);
      return;
    } catch {
      await page.waitForTimeout(300);
    }
  }
  await page.locator(selector).fill(value);
}

test.describe("Project flow", () => {
  test.beforeEach(async ({ page }) => {
    await seedAdminSession(page);
  });

  test("create and view project", async ({ page }) => {
    const suffix = uniqueSuffix();
    const name = `E2E Project ${suffix}`;
    const identifier = `e2e-project-${suffix}`;
    const projectId = await createProject(page, name, identifier);
    await expect(page.locator("h1").first()).toContainText(name);
    await page.goto(`/projects/${projectId}`);
    await expect(page).toHaveURL(new RegExp(`/projects/${projectId}$`));
  });

  test("edit project", async ({ page }) => {
    const suffix = uniqueSuffix();
    const name = `Project Edit ${suffix}`;
    const identifier = `project-edit-${suffix}`;
    const projectId = await createProject(page, name, identifier);
    await page.goto(`/projects/${projectId}/edit`);
    await expect(page.locator("#name")).toHaveValue(name);
    await fillStable(page, "#name", `${name} Updated`);
    await page.getByRole("button", { name: "Update Project" }).click();
    await expect(page).toHaveURL(new RegExp(`/projects/${projectId}$`));
    await expect(page.locator("h1").first()).toContainText(`${name} Updated`);
  });

  test("delete project", async ({ page }) => {
    const suffix = uniqueSuffix();
    const name = `Project Delete ${suffix}`;
    const identifier = `project-delete-${suffix}`;
    const projectId = await createProject(page, name, identifier);
    await page.goto(`/projects/${projectId}/settings`);
    await page.getByRole("button", { name: "Delete Project" }).click();
    await page.getByRole("button", { name: "Delete", exact: true }).click();
    await expect(page).toHaveURL(/\/projects$/);
    await expect(page.getByText(name)).toHaveCount(0);
  });

  test("create subproject with parent", async ({ page }) => {
    const suffix = uniqueSuffix();
    const parentName = `Parent ${suffix}`;
    const parentIdentifier = `parent-${suffix}`;
    const parentId = await createProject(page, parentName, parentIdentifier);
    const token = await page.evaluate(() => window.localStorage.getItem("auth_token"));
    if (!token) {
      throw new Error("Missing auth token");
    }
    const response = await page.request.post("/api/v1/projects", {
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      data: {
        project: {
          name: `Child ${suffix}`,
          identifier: `child-${suffix}`,
          parent_id: parentId,
          is_public: true,
        },
      },
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    const childId = body.project.id as number;
    await page.goto(`/projects/${childId}`);
    await expect(page.locator("h1").first()).toContainText(`Child ${suffix}`);
  });

  test("update visibility flag", async ({ page }) => {
    const suffix = uniqueSuffix();
    const name = `Visibility ${suffix}`;
    const identifier = `visibility-${suffix}`;
    const projectId = await createProject(page, name, identifier);
    await page.goto(`/projects/${projectId}/edit`);
    await expect(page.locator("#name")).toHaveValue(name);
    await page.getByRole("switch").first().click();
    await page.getByRole("button", { name: "Update Project" }).click();
    await expect(page).toHaveURL(new RegExp(`/projects/${projectId}$`));
    await expect(page.getByText("Private")).toBeVisible();
  });
});
