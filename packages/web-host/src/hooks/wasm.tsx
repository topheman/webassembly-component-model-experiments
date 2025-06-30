import { createContext, useEffect, useState } from "react";

import { prepareEngine } from "../wasm/engine";

type WasmContext =
  | {
      status: "idle";
      error: null;
      engine: null;
    }
  | {
      status: "loading";
      error: null;
      engine: null;
    }
  | {
      status: "ready";
      error: null;
      engine: Awaited<ReturnType<typeof prepareEngine>>;
    }
  | {
      status: "error";
      error: Error;
      engine: null;
    };

const WasmContext = createContext<WasmContext>({
  status: "idle",
  error: null,
  engine: null,
});

export function WasmProvider({ children }: { children: React.ReactNode }) {
  const [context, setContext] = useState<WasmContext>({
    status: "idle",
    error: null,
    engine: null,
  });

  useEffect(() => {
    console.log("useEffect prepareEngine");
    prepareEngine()
      .then((engine) => {
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
