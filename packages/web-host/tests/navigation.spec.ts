import { expect, test } from "@playwright/test";
import { clickWandButton } from "./utils";

test("start REPL link", async ({ page }) => {
  await page.goto("/");
  await page.getByTestId("start-repl-button-top").click({ force: true });
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
  await page.getByTestId("start-repl-button-top").click({ force: true });
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

test("history should be preserved + wand button", async ({ page }) => {
  await page.goto("/");
  await page.getByTestId("start-repl-button-top").click({ force: true });
  await expect(page).toHaveURL(
    "/webassembly-component-model-experiments/#repl",
  );
  await clickWandButton(page, "echo foo", { expectStdout: "foo" });
  await clickWandButton(page, "echo bar", { expectStdout: "bar" });
  await clickWandButton(page, "echo baz", { expectStdout: "baz" });
  await page.goBack();
  await expect(
    page.getByRole("heading", {
      name: "WebAssembly Component Model Experiments",
    }),
  ).toBeVisible();
  await page.getByTestId("start-repl-button-top").click({ force: true });
  await expect(page).toHaveURL(
    "/webassembly-component-model-experiments/#repl",
  );
  await expect(
    page.getByText("echo foofooecho barbarecho bazbaz"),
  ).toBeVisible();
});
