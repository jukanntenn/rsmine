import { test, expect } from "@playwright/test";
import { loginAsAdmin } from "./helpers";

test.describe("Authentication flow", () => {
  test("login success redirects to projects", async ({ page }) => {
    await loginAsAdmin(page);
    await expect(page.getByRole("heading", { name: "Projects" })).toBeVisible();
  });

  test("login failure stays on login page", async ({ page }) => {
    await page.goto("/login");
    await page.evaluate(() => {
      window.localStorage.clear();
      window.sessionStorage.clear();
    });
    await page.reload();
    await page.getByPlaceholder("Enter your username").fill("admin");
    await page.getByPlaceholder("Enter your password").fill("wrong-password");
    await page.getByRole("button", { name: "Sign In" }).click();
    await expect(page).toHaveURL(/\/login$/);
    await expect(page.locator("form").getByText("Invalid username or password")).toBeVisible();
  });

  test("logout redirects to login", async ({ page }) => {
    await loginAsAdmin(page);
    await page.locator("button.relative.h-9.w-9.rounded-full").click();
    await page.getByText("Log out").click();
    await expect(page).toHaveURL(/\/login$/);
  });

  test("protected route redirects to login", async ({ page }) => {
    await page.goto("/login");
    await page.evaluate(() => {
      window.localStorage.clear();
      window.sessionStorage.clear();
    });
    await page.goto("/projects");
    await expect(page).toHaveURL(/\/login$/);
  });

  test("session persists after refresh", async ({ page }) => {
    await loginAsAdmin(page);
    await page.reload();
    await expect(page).toHaveURL(/\/projects$/);
  });
});
