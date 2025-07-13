import { expect, test } from "@playwright/test";

test("start REPL link", async ({ page }) => {
  await page.goto("/");
  await page
    .getByRole("button", { name: "✨ Start REPL ✨" })
    .click({ force: true });
  await expect(page).toHaveURL(
    "/webassembly-component-model-experiments/#repl",
  );
  await expect(
    page.getByRole("heading", { name: "REPL Interface" }),
  ).toBeVisible();
});

test("direct load repl page", async ({ page }) => {
  await page.goto("/#repl");
  await expect(
    page.getByRole("heading", { name: "REPL Interface" }),
  ).toBeVisible();
});

test("back to home button", async ({ page }) => {
  await page.goto("/#repl");
  await page.getByRole("button", { name: " Back to Home" }).click();
  await expect(page).toHaveURL(
    "/webassembly-component-model-experiments/#home",
  );
  await expect(
    page.getByRole("heading", {
      name: "WebAssembly Component Model Experiments",
    }),
  ).toBeVisible();
});

test("back button", async ({ page }) => {
  await page.goto("/");
  await page
    .getByRole("button", { name: "✨ Start REPL ✨" })
    .click({ force: true });
  await expect(page).toHaveURL(
    "/webassembly-component-model-experiments/#repl",
  );
  await expect(
    page.getByRole("heading", { name: "REPL Interface" }),
  ).toBeVisible();
  await page.goBack();
  await expect(
    page.getByRole("heading", {
      name: "WebAssembly Component Model Experiments",
    }),
  ).toBeVisible();
});
