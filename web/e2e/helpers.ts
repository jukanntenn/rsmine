import { expect, type Page } from "@playwright/test";

export async function seedAdminSession(page: Page) {
  const response = await page.request.post("/api/v1/auth/login", {
    data: {
      username: "admin",
      password: "admin123",
    },
  });
  if (!response.ok()) {
    throw new Error("Failed to seed admin session");
  }
  const body = await response.json();
  const token = body.token as string;
  const user = body.user;
  if (!token || !user) {
    throw new Error("Invalid auth response while seeding session");
  }
  await page.addInitScript(
    ({ tokenValue, userValue }) => {
      window.localStorage.setItem("auth_token", tokenValue);
      window.localStorage.setItem(
        "auth-storage",
        JSON.stringify({
          state: {
            user: userValue,
            token: tokenValue,
            isAuthenticated: true,
          },
          version: 0,
        })
      );
    },
    { tokenValue: token, userValue: user }
  );
  await page.goto("/projects");
  await expect(page).toHaveURL(/\/projects$/);
}

export async function loginAsAdmin(page: Page) {
  await page.goto("/login");
  await page.evaluate(() => {
    window.localStorage.clear();
    window.sessionStorage.clear();
  });
  await page.reload();
  await page.getByPlaceholder("Enter your username").fill("admin");
  await page.getByPlaceholder("Enter your password").fill("admin123");
  await page.getByRole("button", { name: "Sign In" }).click();
  await expect(page).toHaveURL(/\/projects$/);
  await page.evaluate(() => {
    window.sessionStorage.removeItem("redirect_after_login");
  });
}

export async function createProject(page: Page, name: string, identifier: string) {
  await page.goto("/projects/new");
  await expect(page).toHaveURL(/\/projects\/new$/);
  await expect(page.locator("#name")).toBeVisible();
  await page.locator("#name").fill(name);
  await page.locator("#identifier").fill(identifier);
  await page.getByRole("button", { name: "Create Project" }).click();
  await expect(page).toHaveURL(/\/projects\/\d+$/);
  const url = page.url();
  const match = url.match(/\/projects\/(\d+)$/);
  if (!match) {
    throw new Error("Project id not found from URL");
  }
  return Number(match[1]);
}

export async function createIssueInProject(page: Page, projectId: number, subject: string) {
  await page.goto(`/projects/${projectId}/issues/new`);
  await page.locator("#tracker_id").selectOption({ index: 1 });
  await page.locator("#priority_id").selectOption({ index: 1 });
  await page.locator("#subject").fill(subject);
  const createResponsePromise = page.waitForResponse(
    (response) => response.request().method() === "POST" && response.url().includes("/api/v1/issues")
  );
  await page.getByRole("button", { name: "Create Issue" }).click();
  const createResponse = await createResponsePromise;
  if (!createResponse.ok()) {
    throw new Error(`Create issue failed: ${createResponse.status()} ${await createResponse.text()}`);
  }
  const body = await createResponse.json();
  const issueId = body.issue?.id as number | undefined;
  if (!issueId) {
    throw new Error("Issue id missing in create response");
  }
  await page.waitForURL(new RegExp(`/issues/${issueId}$`), { timeout: 10000 }).catch(async () => {
    await page.goto(`/issues/${issueId}`);
  });
  return issueId;
}

export async function findProjectWithTrackers(page: Page) {
  const token = await page.evaluate(() => window.localStorage.getItem("auth_token"));
  if (!token) {
    throw new Error("Missing auth token");
  }
  const projectsResponse = await page.request.get("/api/v1/projects?limit=100", {
    headers: { Authorization: `Bearer ${token}` },
  });
  if (!projectsResponse.ok()) {
    throw new Error(`Failed to load projects: ${projectsResponse.status()}`);
  }
  const projectsBody = await projectsResponse.json();
  const projects = projectsBody.projects as Array<{ id: number }>;
  for (const project of projects) {
    const trackersResponse = await page.request.get(`/api/v1/projects/${project.id}/trackers`, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!trackersResponse.ok()) {
      continue;
    }
    const trackersBody = await trackersResponse.json();
    const trackers = trackersBody.trackers as Array<{ id: number }>;
    if (trackers.length > 0) {
      return project.id;
    }
  }
  throw new Error("No project with trackers found");
}
