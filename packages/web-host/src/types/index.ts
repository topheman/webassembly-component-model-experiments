import type { replLogic as ReplLogicTypes } from "./generated/host-api";
import type { plugin as PluginTypes } from "./generated/plugin-api";

export type PluginApi = typeof PluginTypes;
export type ReplLogicApi = typeof ReplLogicTypes;
