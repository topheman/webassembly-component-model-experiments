import { expect, type Page } from "@playwright/test";

/**
 * Get the last std output of the given type
 */
export async function getLastStd(
  page: Page,
  type: "stdin" | "stdout" | "stderr",
) {
  return await page.locator(`[data-stdtype='${type}']`).last();
}

/**
 * Fill the input with the command and submit it
 * Pass the expected stdin, stdout and stderr to check the results
 */
export async function fillAndSubmitCommand(
  page: Page,
  command: string,
  {
    expectStdin = command,
    expectStdout,
    expectStderr,
  }: {
    expectStdin?: string;
    expectStdout?: string;
    expectStderr?: string;
  } = {},
) {
  const input = await page.getByPlaceholder("Type a command...");
  await input.fill(command);
  await input.press("Enter");
  const stdin = await getLastStd(page, "stdin");
  await expect(stdin).toHaveText(expectStdin);
  if (expectStdout) {
    const stdout = await getLastStd(page, "stdout");
    await expect(stdout).toHaveText(expectStdout);
  }
  if (expectStderr) {
    const stderr = await getLastStd(page, "stderr");
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
    expectStdin = command,
    expectStdout,
    expectStderr,
  }: {
    expectStdin?: string;
    expectStdout?: string;
    expectStderr?: string;
  } = {},
) {
  await page.getByTitle("Run example command").click({ force: true });
  const input = await page.getByPlaceholder("Type a command...");
  await expect(input).toHaveValue(expectStdin);
  const stdin = await getLastStd(page, "stdin");
  await expect(stdin).toHaveText(expectStdin);
  if (expectStdout) {
    const stdout = await getLastStd(page, "stdout");
    await expect(stdout).toHaveText(expectStdout);
  }
  if (expectStderr) {
    const stderr = await getLastStd(page, "stderr");
    await expect(stderr).toHaveText(expectStderr);
  }
}
