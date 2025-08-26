import { expect, test } from "@playwright/test";
import { fillAndSubmitCommand, getLastStd } from "./utils";

test("echo $0", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "echo foo", { expectStdout: "foo" });
  await fillAndSubmitCommand(page, "echo bar", { expectStdout: "bar" });
  await fillAndSubmitCommand(page, "echo $0", { expectStdout: "bar" });
});

test("echo $ROOT/$USER", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "echo $ROOT/$USER", {
    expectStdout: "/Users/Tophe",
  });
});

test("export USER=WebAssembly", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "export USER=WebAssembly");
  await fillAndSubmitCommand(page, "echo $ROOT/$USER", {
    expectStdout: "/Users/WebAssembly",
  });
});

test("echo $?", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "echo $?", { expectStdout: "0" });
  await fillAndSubmitCommand(page, "azertyuiop", {
    expectStderr:
      "Unknown command: azertyuiop. Try `help` to see available commands.",
  });
  await fillAndSubmitCommand(page, "echo $?", { expectStdout: "1" });
  await fillAndSubmitCommand(page, "echo $?", { expectStdout: "0" });
});

test("help", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "help");
  const stdout = await getLastStd(page, "stdout");
  await expect(stdout).toContainText("help - Show the manual for a command");
});

test("man help -> should show the help", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "help");
  const stdout = await getLastStd(page, "stdout");
  await expect(stdout).toContainText("help - Show the manual for a command");
});

test("man for reserved commands", async ({ page }) => {
  const reservedCommands = [
    ["help", "help - Show the manual for a command"],
    ["export", "export - Export a variable to the environment"],
    [
      "list-commands",
      "list-commands - List the plugins loaded in the host and the reserved commands (not overridable by plugins) included in the REPL logic.",
    ],
    ["man", "man - Show the manual for a command"],
  ];
  await page.goto("/#repl");
  for (const [command, partialManpage] of reservedCommands) {
    await fillAndSubmitCommand(page, `man ${command}`);
    const stdout = await getLastStd(page, "stdout");
    await expect(stdout).toContainText(partialManpage);
  }
});

test("list-commands", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "list-commands");
  const stdout = await getLastStd(page, "stdout");
  await expect(stdout).toContainText(
    `cat	plugin
echo	plugin
echoc	plugin
echogo	plugin
export	reserved
greet	plugin
help	reserved
list-commands	reserved
ls	plugin
man	reserved
tee	plugin
weather	plugin`,
  );
});

test("man", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "man");
  const stdout = await getLastStd(page, "stdout");
  await expect(stdout).toContainText("man - Show the manual for a command");
  await fillAndSubmitCommand(page, "man man");
  const stdout2 = await getLastStd(page, "stdout");
  await expect(stdout2).toContainText("man - Show the manual for a command");
});

test("man echo", async ({ page }) => {
  await page.goto("/#repl");
  await fillAndSubmitCommand(page, "man echo");
  const stdout = await getLastStd(page, "stdout");
  await expect(stdout).toContainText("echo - Echo a message (built with Rust");
});
