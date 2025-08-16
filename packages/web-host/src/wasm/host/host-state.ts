import type { ReplVar } from "../../types/generated/interfaces/repl-api-host-state";

const internalState = {
  replVars: new Map<string, string>(),
  pluginsNames: new Set<string>(),
};

export function _setPluginsNames(pluginsNames: string[]) {
  internalState.pluginsNames = new Set(pluginsNames);
}

export function getPluginsNames(): string[] {
  return Array.from(internalState.pluginsNames);
}

export function getReplVars(): ReplVar[] {
  return Array.from(internalState.replVars.entries()).map(([name, value]) => ({
    key: name,
    value,
  }));
}

export function setReplVar({ key, value }: { key: string; value: string }) {
  internalState.replVars.set(key, value);
}
