import { createContext, useEffect, useReducer, useState } from "react";

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
  const plugins = new Map();
  const replVars = new Map();
  let replLogicGuest: (...args: any) => any | undefined;
  return {
    registerPlugin(name: string, func: (...args: any) => any) {
      plugins.set(name, func);
    },
    registerReplLogicGuest(func: (...args: any) => any) {
      replLogicGuest = func;
    },
    runPlugin(name: string, payload: string) {
      const plugin = plugins.get(name);
      if (!plugin) {
        throw new Error(`Plugin ${name} not found`);
      }
      return plugin(payload);
    },
    runReplLogicGuest(payload: string) {
      if (!replLogicGuest) {
        throw new Error("Repl logic guest not found");
      }
      return replLogicGuest().run(payload);
    },
    host: {
      setReplVars(vars: Record<string, string>) {
        for (const [name, value] of Object.entries(vars)) {
          replVars.set(name, value);
        }
      },
      setReplVar(name: string, value: string) {
        replVars.set(name, value);
      },
      getReplVars() {
        return Array.from(replVars.entries()).map(([name, value]) => ({
          name,
          value,
        }));
      },
      getPluginsNames() {
        return Array.from(plugins.keys());
      },
    },
  };
}

function loadPlugins(paths: string[]) {
  return Promise.all(
    paths.map(async (path) => {
      const module = await import(path);
      const plugin = module.default;
      return plugin;
    }),
  );
}

async function loadReplLogicGuest() {
  const module = await import(REPL_LOGIC_GUEST_PATH);
  const plugin = module.default;
  return plugin;
}

async function prepareWasmApi() {
  const [plugins, replLogicGuest] = await Promise.all([
    loadPlugins(PLUGINS_PATHS),
    loadReplLogicGuest(),
  ]);
  const api = makeApi();
  for (const plugin of plugins) {
    api.registerPlugin(plugin.func.name(), plugin.func);
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
    prepareWasmApi()
      .then((api) => {
        setContext({
          status: "ready",
          error: null,
          api,
        });
      })
      .catch((error) => {
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
