import { expect, type Page, test } from "@playwright/test";

async function getLastStd(page: Page, type: "stdin" | "stdout" | "stderr") {
  return await page.locator(`[data-stdtype='${type}']`).last();
}

test("echo foo - enter", async ({ page }) => {
  await page.goto("/#repl");
  const input = await page.getByPlaceholder("Type a command...");
  await input.fill("echo foo");
  await input.press("Enter");
  const stdin = await getLastStd(page, "stdin");
  await expect(stdin).toHaveText("echo foo");
  const stdout = await getLastStd(page, "stdout");
  await expect(stdout).toHaveText("foo");
});

test("echo foo - run", async ({ page }) => {
  await page.goto("/#repl");
  const input = await page.getByPlaceholder("Type a command...");
  await input.fill("echo foo");
  await page.getByRole("button", { name: "Run", exact: true }).click();
  const stdin = await getLastStd(page, "stdin");
  await expect(stdin).toHaveText("echo foo");
  const stdout = await getLastStd(page, "stdout");
  await expect(stdout).toHaveText("foo");
});
