import type * as HostApiNamespace from "./generated/host-api";
import type * as PluginApiNamespace from "./generated/plugin-api";

export * from "./generated/interfaces/repl-api-transport";

export type HostApi = typeof HostApiNamespace;
export type PluginApi = typeof PluginApiNamespace;
