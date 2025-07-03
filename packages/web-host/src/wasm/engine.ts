import type { HostApi, PluginApi } from "../types";

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

function loadPlugins(): Promise<PluginApi["plugin"][]> {
  return Promise.all([
    import("./generated/plugin_echo/transpiled/plugin_echo.js"),
    import("./generated/plugin_weather/transpiled/plugin_weather.js"),
    import("./generated/plugin_greet/transpiled/plugin_greet.js"),
    import("./generated/plugin_ls/transpiled/plugin_ls.js"),
  ]).then((plugins) => plugins.map((plugin) => plugin.plugin));
}

async function loadReplLogicGuest(): Promise<HostApi> {
  return import("./generated/repl_logic_guest/transpiled/repl_logic_guest.js");
}

export async function prepareEngine(): Promise<ReturnType<typeof makeEngine>> {
  const [replLogicGuest, plugins] = await Promise.all([
    loadReplLogicGuest(),
    loadPlugins(),
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
