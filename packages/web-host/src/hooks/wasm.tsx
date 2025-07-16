import { setReplVar } from "repl:api/host-state";
import { createContext, useContext, useEffect, useState } from "react";
import { prepareEngine, type WasmEngine } from "../wasm/engine";
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
    const abortController = new AbortController();
    prepareEngine({ addReplHistoryEntry, abortSignal: abortController.signal })
      .then(async (engine) => {
        if (!engine) {
          console.log("prepareEngine aborted");
          return;
        }
        console.log("useEffect prepareEngine success", engine);
        setReplVar({ key: "ROOT", value: "/Users" });
        setReplVar({ key: "USER", value: "Tophe" });
        setReplVar({ key: "?", value: "0" });
        setContext({
          status: "ready",
          error: null,
          engine,
        });
        addReplHistoryEntry({
          stdin: "[Host] REPL host ready",
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
    return () => {
      console.log("useEffect prepareEngine abort");
      abortController.abort("Avoid react useEffect re-run");
    };
  }, [addReplHistoryEntry]);

  return (
    <WasmContext.Provider value={context}>{children}</WasmContext.Provider>
  );
}

export function useWasm() {
  const context = useContext(WasmContext);
  return context;
}
