import { setReplVar } from "repl:api/host-state";
import { createContext, useContext, useEffect, useState } from "react";
import { prepareEngine } from "../wasm/engine";
import { useReplHistory } from "./replHistory";

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

export function WasmProvider({ children }: { children: React.ReactNode }) {
  const { addEntry: addReplHistoryEntry } = useReplHistory();
  const [context, setContext] = useState<WasmContext>({
    status: "loading",
    error: null,
    engine: null,
  });

  useEffect(() => {
    console.log("useEffect prepareEngine");
    prepareEngine({ addReplHistoryEntry })
      .then(async (engine) => {
        // await sleep(1000);
        console.log("useEffect prepareEngine success", engine);
        setReplVar({ key: "ROOT", value: "/Users" });
        setReplVar({ key: "USER", value: "Tophe" });
        setReplVar({ key: "?", value: "0" });
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
  }, [addReplHistoryEntry]);

  return (
    <WasmContext.Provider value={context}>{children}</WasmContext.Provider>
  );
}

export function useWasm() {
  const context = useContext(WasmContext);
  return context;
}
