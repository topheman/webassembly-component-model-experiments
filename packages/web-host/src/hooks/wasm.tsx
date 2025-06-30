import { createContext, useContext, useEffect, useState } from "react";

import { prepareEngine } from "../wasm/engine";

type WasmContext =
  | {
      status: "loading";
      error: null;
      engine: null;
    }
  | {
      status: "ready";
      error: null;
      engine: WasmEngine;
    }
  | {
      status: "error";
      error: Error;
      engine: null;
    };

export type WasmEngine = Awaited<ReturnType<typeof prepareEngine>>;

const WasmContext = createContext<WasmContext>({
  status: "loading",
  error: null,
  engine: null,
});

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export function WasmProvider({ children }: { children: React.ReactNode }) {
  const [context, setContext] = useState<WasmContext>({
    status: "loading",
    error: null,
    engine: null,
  });

  useEffect(() => {
    console.log("useEffect prepareEngine");
    prepareEngine()
      .then(async (engine) => {
        // await sleep(1000);
        console.log("useEffect prepareEngine success", engine);
        setContext({
          status: "ready",
          error: null,
          engine,
        });
      })
      .catch((error) => {
        console.log("useEffect prepareEngine error", error);
        setContext({
          status: "error",
          error,
          engine: null,
        });
      });
  }, []);

  return (
    <WasmContext.Provider value={context}>{children}</WasmContext.Provider>
  );
}

export function useWasm() {
  const context = useContext(WasmContext);
  return context;
}
