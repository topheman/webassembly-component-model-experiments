import { _setPluginsNames as hostStateSetPluginsNames } from "repl:api/host-state";
import type { HostApi, PluginApi, ReplHistoryEntry } from "../types";

type AddReplHistoryEntryProp = {
  addReplHistoryEntry: (entry: ReplHistoryEntry) => void;
};

export type WasmEngine = ReturnType<typeof makeEngine>;

function makeEngine() {
  const plugins = new Map<string, PluginApi["plugin"]>();
  let replLogicGuest: HostApi | undefined;
  return {
    registerPlugin(name: string, pluginInstance: PluginApi["plugin"]) {
      console.log("registerPlugin", name, pluginInstance);
      plugins.set(name, pluginInstance);
    },
    registerReplLogicGuest(func: HostApi) {
      console.log("registerReplLogicGuest", func);
      replLogicGuest = func;
    },
    getPlugin(name: string): PluginApi["plugin"] | undefined {
      return plugins.get(name);
    },
    getReplLogicGuest(): HostApi {
      if (!replLogicGuest) {
        throw new Error("Repl logic guest not found");
      }
      return replLogicGuest;
    },
  };
}

async function loadPlugins({
  addReplHistoryEntry,
}: AddReplHistoryEntryProp): Promise<PluginApi["plugin"][]> {
  const plugins = await Promise.all([
    import("./generated/plugin_echo/transpiled/plugin_echo.js"),
    import("./generated/plugin_weather/transpiled/plugin_weather.js"),
    import("./generated/plugin_greet/transpiled/plugin_greet.js"),
    import("./generated/plugin_ls/transpiled/plugin_ls.js"),
    import("./generated/plugin_cat/transpiled/plugin_cat.js"),
  ]).then((plugins) => plugins.map((plugin) => plugin.plugin));

  // log the plugins names
  const pluginsNames = plugins.map((plugin) => plugin.name());
  for (const pluginName of pluginsNames) {
    addReplHistoryEntry({
      stdin: `[Host] Loading plugin: ${pluginName}`,
    });
  }

  // set the plugins names in the host state
  hostStateSetPluginsNames(pluginsNames);

  // return the plugins instances
  return plugins;
}

async function loadReplLogicGuest(): Promise<HostApi> {
  return import("./generated/repl_logic_guest/transpiled/repl_logic_guest.js");
}

export async function prepareEngine({
  addReplHistoryEntry,
  abortSignal,
}: AddReplHistoryEntryProp & {
  abortSignal?: AbortSignal;
}): Promise<WasmEngine | undefined> {
  // The abort signal will only be marked as aborted after the next tick
  await Promise.resolve(() => setTimeout(() => {}, 0));
  // We receive an abort signal from the useEffect hook
  // it is only useful in development to avoid the useEffect to re-run
  if (abortSignal?.aborted) {
    return;
  }
  addReplHistoryEntry({ stdin: `[Host] Starting REPL host...` });
  addReplHistoryEntry({ stdin: `[Host] Loading REPL logic` });
  const [replLogicGuest, plugins] = await Promise.all([
    loadReplLogicGuest(),
    loadPlugins({ addReplHistoryEntry }),
  ]);
  const engine = makeEngine();
  console.log("plugins", plugins);
  for (const plugin of plugins) {
    console.log("plugin", plugin);
    engine.registerPlugin(plugin.name(), plugin);
  }
  engine.registerReplLogicGuest(replLogicGuest);
  return engine;
}
