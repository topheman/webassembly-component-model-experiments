import { setReplVar } from "repl:api/host-state";
import { useMemo, useReducer } from "react";
import type { ReplStatus } from "../types";
import type { WasmEngine } from "./wasm";

const MAX_HISTORY_LENGTH = 50;

export type ReplHistoryEntry = {
  stdin: string;
  stdout?: string;
  stderr?: string;
  status: ReplStatus;
};

function setExitStatus(status: ReplStatus) {
  if (status === "success") {
    setReplVar({ key: "?", value: "0" });
  } else {
    setReplVar({ key: "?", value: "1" });
  }
}

function makeReplLogicHandler({
  engine,
  updateReplHistory,
}: {
  engine: WasmEngine;
  updateReplHistory: (payload: ReplHistoryEntry) => void;
}) {
  return function handleInput(input: string) {
    const result = engine.getReplLogicGuest().replLogic.readline(input);
    switch (result.tag) {
      case "to-run":
        updateReplHistory({
          stdin: input,
          stdout: JSON.stringify(result, null, 2),
          status: "success",
        });
        setExitStatus("success");
        break;
      case "ready":
        updateReplHistory({
          stdin: input,
          stdout: JSON.stringify(result, null, 2),
          status: "error",
        });
        setExitStatus("error");
        break;
    }
  };
}

/**
 * Handles the state of the repl - the history of commands and their results.
 * @param state
 * @param payload
 * @returns
 */
function replStateReducer(
  state: Array<ReplHistoryEntry>,
  payload: ReplHistoryEntry,
) {
  if (state.length >= MAX_HISTORY_LENGTH) {
    // remove the oldest entry
    return [...state.slice(1), payload];
  }
  return [...state, payload];
}

export function useReplLogic({ engine }: { engine: WasmEngine }) {
  const [replHistory, updateReplHistory] = useReducer(replStateReducer, []);
  const handleInput = useMemo(
    () => makeReplLogicHandler({ engine, updateReplHistory }),
    [engine],
  );

  return { handleInput, replHistory };
}
