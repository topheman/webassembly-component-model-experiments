import type { plugin as pluginApi } from "./types/plugin-api";

export const plugin: typeof pluginApi = {
  name: () => "echo",
  man: () => `
NAME
    echo - echo a message (Built with TypeScript 🟦)

USAGE
    echo <message>

DESCRIPTION
    Echo a message.
  `,
  argCount: () => 1,
  run: (payload: string) => {
    return {
      status: "success",
      stdout: payload,
      stderr: undefined,
    }
  }
}
