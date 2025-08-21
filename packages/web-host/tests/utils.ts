import { expect, type Locator, type Page } from "@playwright/test";

/**
 * Get the last std output of the given type
 * If expectContent is provided, it will retry to get the last std output until it matches the expected content
 */
export async function getLastStd(
  page: Page,
  type: "stdin" | "stdout" | "stderr",
  {
    expectContent,
  }: {
    expectContent?: string;
  } = {},
) {
  const locator = await page.locator(`[data-stdtype='${type}']`).last();
  if (expectContent) {
    const text = await locator.textContent();
    if (text?.includes(expectContent)) {
      return locator;
    }
    // if no match, do a hard expect that will fail the test with a clear error message
    // Sorry you landed here, you will most likely have to add some `sleep()` in your code 🥲
    await expect(locator).toHaveText(expectContent);
  }
  return locator;
}

/**
 * Get the last std output of the given type after the given locator
 * This is useful to get the last std output after a command has been submitted
 * This ensures you don't have false positives when checking the last std output
 */
export async function getLastStdAfter(
  page: Page,
  type: "stdin" | "stdout" | "stderr",
  stdLocator: Locator,
) {
  const stdinIndex = await stdLocator.getAttribute("data-std-index");
  return await page
    .locator(`[data-std-index='${stdinIndex}'] ~ [data-stdtype='${type}']`)
    .last();
}

export async function sleep(ms?: number): Promise<void> {
  const DEFAULT_DELAY = 200; // taking into account the default delay necessary in the CI
  return new Promise((resolve) => setTimeout(resolve, ms ?? DEFAULT_DELAY));
}

/**
 * Fill the input with the command and submit it
 * Pass the expected stdin, stdout and stderr to check the results
 */
export async function fillAndSubmitCommand(
  page: Page,
  command: string,
  {
    expectStdin,
    expectStdout,
    expectStderr,
    afterSubmit,
  }: {
    expectStdin?: string;
    expectStdout?: string;
    expectStderr?: string;
    afterSubmit?: () => Promise<void>;
  } = {},
) {
  const expectedStdin = expectStdin ?? command;
  const input = await page.getByPlaceholder("Type a command...");
  await input.fill(command);
  await input.press("Enter");
  if (afterSubmit) {
    await afterSubmit();
  }
  const stdin = await getLastStd(page, "stdin", {
    expectContent: expectedStdin,
  });
  if (expectStdout) {
    const stdout = await getLastStdAfter(page, "stdout", stdin);
    await expect(stdout).toHaveText(expectStdout);
  }
  if (expectStderr) {
    const stderr = await getLastStdAfter(page, "stderr", stdin);
    await expect(stderr).toHaveText(expectStderr);
  }
}

/**
 * Click the wand button and check the results
 * Pass the expected stdin, stdout and stderr to check the results
 */
export async function clickWandButton(
  page: Page,
  command: string,
  {
    expectStdin,
    expectStdout,
    expectStderr,
  }: {
    expectStdin?: string;
    expectStdout?: string;
    expectStderr?: string;
  } = {},
) {
  const expectedStdin = expectStdin ?? command;
  await page.getByTitle("Run example command").click({ force: true });
  const input = await page.getByPlaceholder("Type a command...");
  await expect(input).toHaveValue(expectedStdin);
  const stdin = await getLastStd(page, "stdin", {
    expectContent: expectedStdin,
  });
  if (expectStdout) {
    const stdout = await getLastStdAfter(page, "stdout", stdin);
    await expect(stdout).toHaveText(expectStdout);
  }
  if (expectStderr) {
    const stderr = await getLastStd(page, "stderr");
    await expect(stderr).toHaveText(expectStderr);
  }
}
