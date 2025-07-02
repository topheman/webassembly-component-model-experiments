import type { HostApi, PluginApi } from "../types";

const PLUGINS_PATHS = [
  import.meta.resolve("./generated/plugin_echo/transpiled/plugin_echo.js"),
  import.meta.resolve(
    "./generated/plugin_weather/transpiled/plugin_weather.js",
  ),
  import.meta.resolve("./generated/plugin_greet/transpiled/plugin_greet.js"),
  import.meta.resolve("./generated/plugin_ls/transpiled/plugin_ls.js"),
];
const REPL_LOGIC_GUEST_PATH = import.meta.resolve(
  "./generated/repl_logic_guest/transpiled/repl_logic_guest.js",
);

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
    getPlugin(name: string): PluginApi["plugin"] {
      const pluginInstance = plugins.get(name);
      if (!pluginInstance) {
        throw new Error(`Plugin ${name} not found`);
      }
      return pluginInstance;
    },
    getReplLogicGuest(): HostApi {
      if (!replLogicGuest) {
        throw new Error("Repl logic guest not found");
      }
      return replLogicGuest;
    },
  };
}

function loadPlugins(paths: string[]): Promise<PluginApi["plugin"][]> {
  return Promise.all(
    paths.map(async (path) => {
      const module = await import(path);
      console.log("loadPlugins", path, module);
      return module.plugin;
    }),
  );
}

async function loadReplLogicGuest(): Promise<HostApi> {
  const module = await import(REPL_LOGIC_GUEST_PATH);
  console.log("loadReplLogicGuest", module);
  return module;
}

export async function prepareEngine(): Promise<ReturnType<typeof makeEngine>> {
  const [replLogicGuest, plugins] = await Promise.all([
    loadReplLogicGuest(),
    loadPlugins(PLUGINS_PATHS),
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
