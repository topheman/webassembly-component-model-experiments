import { expect, test } from "@playwright/test";
import { fillAndSubmitCommand, getLastStd } from "./utils";

test("echo foo - enter", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "echo foo", { expectStdout: "foo" });
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

// todo: test wand button
