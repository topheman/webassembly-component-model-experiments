import { _setPluginsNames as hostStateSetPluginsNames } from "repl:api/host-state";
import type { HostApi, PluginApi, ReplHistoryEntry } from "../types";
import { getPluginSourceUrl } from "../utils/github";

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
    import("./generated/plugin_tee/transpiled/plugin_tee.js"),
    import("./generated/plugin-echo-c/transpiled/plugin-echo-c.js"),
    import("./generated/plugin-echo-go/transpiled/plugin-echo-go.js"),
  ]).then((plugins) =>
    plugins.map((plugin) => {
      // Since we `allowHtml`, which will `dangerouslySetInnerHTML`, we need to sanitize the plugin name
      // (a plugin author could attack by injecting a script tag)
      const sanitizedPluginName = plugin.plugin
        .name()
        .replace(/[^a-zA-Z0-9_-]/g, "");
      const sourceUrl = getPluginSourceUrl(sanitizedPluginName);
      const stdin = sourceUrl
        ? `[Host] Loaded plugin: <a href="${sourceUrl}" title="View source of the plugin on GitHub">${sanitizedPluginName}</a>`
        : `[Host] Loaded plugin: ${sanitizedPluginName}`;
      addReplHistoryEntry({
        stdin,
        allowHtml: !!sourceUrl,
      });
      return plugin.plugin;
    }),
  );

  // set the plugins names in the host state
  const pluginsNames = plugins.map((plugin) => plugin.name());
  hostStateSetPluginsNames(pluginsNames);

  // return the plugins instances
  return plugins;
}

async function loadReplLogicGuest({
  addReplHistoryEntry,
}: AddReplHistoryEntryProp): Promise<HostApi> {
  const replLogicGuest = await import(
    "./generated/repl_logic_guest/transpiled/repl_logic_guest.js"
  );
  addReplHistoryEntry({
    stdin: `[Host] Loaded REPL logic`,
  });
  return replLogicGuest;
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
  const [replLogicGuest, plugins] = await Promise.all([
    loadReplLogicGuest({ addReplHistoryEntry }),
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
