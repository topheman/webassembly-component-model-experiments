import { expect, test } from "@playwright/test";
import { fillAndSubmitCommand, getLastStd } from "./utils";

test("echo foo", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "echo foo", { expectStdout: "foo" });
});

test("greet World", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "greet World", {
    expectStdout: "Hello, World!",
  });
});

test("weather Paris", async ({ page, browserName }) => {
  test.skip(
    browserName === "webkit",
    "Skipping weather test on WebKit - network override `route.fulfill` does not work properly",
  );
  await page.route("https://wttr.in/Paris?format=j1", (route) => {
    route.fulfill({
      status: 200,
      body: JSON.stringify({
        current_condition: [
          {
            weatherDesc: [
              {
                value: "Sunny",
              },
            ],
          },
        ],
      }),
    });
  });
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "weather Paris", {
    expectStdout: "Sunny",
  });
});

test("ls", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "ls", {
    expectStdout: `D	data
D	documents
D	logs
F	.config
F	.hidden_file
F	README.md`,
  });
});

test("ls data", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "ls data", {
    expectStdout: `D	data/processed
D	data/raw
F	data/sample.csv
F	data/users.yaml`,
  });
});

test("ls data/users.yaml - should handle files", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "ls data/users.yaml", {
    expectStdout: "F	data/users.yaml",
  });
});

test("cat data", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "cat data", {
    expectStderr: "cat: data: Is a directory",
  });
});

test("cat README.md", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "cat README.md");
  const stdout = await getLastStd(page, "stdout");
  await expect(stdout).toContainText(`# filesystem

You are interacting with a virtual filesystem, in your browser!`);
});
