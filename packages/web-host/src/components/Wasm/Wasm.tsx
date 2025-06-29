import { init as initHostState } from "repl:api/host-state";
import { createContext, useEffect, useState } from "react";
import type { PluginApi, ReplLogicApi } from "../../types";

const PLUGINS_PATHS = [
  "../../wasm/plugin_echo/transpiled/plugin_echo.js",
  "../../wasm/plugin_weather/transpiled/plugin_weather.js",
  "../../wasm/plugin_greet/transpiled/plugin_greet.js",
  "../../wasm/plugin_ls/transpiled/plugin_ls.js",
];
const REPL_LOGIC_GUEST_PATH =
  "../../wasm/repl_logic_guest/transpiled/repl_logic_guest.js";

type Api = ReturnType<typeof makeApi>;

type WasmContext =
  | {
      status: "idle";
      error: null;
      api: null;
    }
  | {
      status: "loading";
      error: null;
      api: null;
    }
  | {
      status: "ready";
      error: null;
      api: Api;
    }
  | {
      status: "error";
      error: Error;
      api: null;
    };

const WasmContext = createContext<WasmContext>({
  status: "idle",
  error: null,
  api: null,
});

function makeApi() {
  console.log("makeApi");
  const plugins = new Map<string, PluginApi>();
  initHostState([], []); // todo: better init
  let replLogicGuest: (...args: any) => any | undefined;
  return {
    registerPlugin(name: string, pluginInstance: PluginApi) {
      plugins.set(name, pluginInstance);
      console.log("registerPlugin", name, pluginInstance);
    },
    registerReplLogicGuest(func: any) {
      console.log("registerReplLogicGuest", func);
      replLogicGuest = func;
    },
    getPlugin(name: string): PluginApi {
      const pluginInstance = plugins.get(name);
      if (!pluginInstance) {
        throw new Error(`Plugin ${name} not found`);
      }
      return pluginInstance;
    },
    runReplLogicGuest(payload: string) {
      if (!replLogicGuest) {
        throw new Error("Repl logic guest not found");
      }
      return replLogicGuest().run(payload);
    },
  };
}

function loadPlugins(paths: string[]): Promise<{ plugin: PluginApi }[]> {
  return Promise.all(
    paths.map(async (path) => {
      const module = await import(path);
      console.log("loadPlugins", path, module);
      return module;
    }),
  );
}

async function loadReplLogicGuest(): Promise<ReplLogicApi> {
  const module = await import(REPL_LOGIC_GUEST_PATH);
  console.log("loadReplLogicGuest", module);
  return module;
}

async function prepareWasmApi() {
  const [replLogicGuest, plugins] = await Promise.all([
    loadReplLogicGuest(),
    loadPlugins(PLUGINS_PATHS),
  ]);
  const api = makeApi();
  console.log("plugins", plugins);
  for (const plugin of plugins) {
    console.log("plugin", plugin);
    api.registerPlugin(plugin.plugin.name(), plugin.plugin);
  }
  api.registerReplLogicGuest(replLogicGuest);
  return api;
}

export function WasmProvider({ children }: { children: React.ReactNode }) {
  const [context, setContext] = useState<WasmContext>({
    status: "idle",
    error: null,
    api: null,
  });

  useEffect(() => {
    console.log("useEffect prepareWasmApi");
    prepareWasmApi()
      .then((api) => {
        console.log("useEffect prepareWasmApi success", api);
        setContext({
          status: "ready",
          error: null,
          api,
        });
      })
      .catch((error) => {
        console.log("useEffect prepareWasmApi error", error);
        setContext({
          status: "error",
          error,
          api: null,
        });
      });
  }, []);

  return (
    <WasmContext.Provider value={context}>{children}</WasmContext.Provider>
  );
}
