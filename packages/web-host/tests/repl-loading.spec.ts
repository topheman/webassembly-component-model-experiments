import { expect, test } from "@playwright/test";

test("wasm should load", async ({ page }) => {
  await page.goto("/#repl");
  await expect(page.getByText("[Host] Starting REPL host...")).toBeVisible();
});

test("repl logic should have loaded", async ({ page }) => {
  await page.goto("/#repl");
  await expect(page.getByText("[Host] Loaded REPL logic")).toBeVisible();
});

test("plugins should have loaded under their names", async ({ page }) => {
  const pluginNames = ["echo", "weather", "greet", "ls", "cat", "echoc", "tee"];
  await page.goto("/#repl");
  for (const pluginName of pluginNames) {
    await expect(
      page.getByText(`[Host] Loaded plugin: ${pluginName}`, { exact: true }),
    ).toBeVisible();
  }
});
