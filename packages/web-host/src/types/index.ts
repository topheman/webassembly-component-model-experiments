// wasm

import type * as HostApiNamespace from "./generated/host-api";
import type { ReplStatus } from "./generated/interfaces/repl-api-transport";
import type * as PluginApiNamespace from "./generated/plugin-api";

export * from "./generated/interfaces/repl-api-transport";

export type HostApi = typeof HostApiNamespace;
export type PluginApi = typeof PluginApiNamespace;

// ui

export type ReplHistoryEntry =
  | {
      stdout?: string;
      stderr?: string;
      status: ReplStatus;
    }
  | {
      stdin: string;
    };
