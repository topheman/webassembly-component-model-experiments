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

function setExitStatusAnd$0(status: ReplStatus, stdout?: string) {
  if (status === "success") {
    setReplVar({ key: "?", value: "0" });
  } else {
    setReplVar({ key: "?", value: "1" });
  }
  if (stdout) {
    setReplVar({ key: "0", value: stdout });
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

    // the result of the command is only parsed, it must be run
    if (result.tag === "to-run") {
      if (result.val.command === "") {
        return;
      }

      // a man command for plugins, we run it from the host
      if (result.val.command === "man") {
        const plugin = engine.getPlugin(result.val.payload);
        if (!plugin) {
          updateReplHistory({
            stdin: input,
            stderr: `Unknown command: ${result.val.payload}. Try \`help\` to see available commands.`,
            status: "error",
          });
          setExitStatusAnd$0("error");
          return;
        }
        const man = plugin.man();
        updateReplHistory({
          stdin: input,
          stdout: man,
          status: "success",
        });
        setExitStatusAnd$0("success", man);
        return;
      }

      // a plugin command, we run it from the host
      const plugin = engine.getPlugin(result.val.command);
      if (!plugin) {
        updateReplHistory({
          stdin: input,
          stderr: `Unknown command: ${result.val.command}. Try \`help\` to see available commands.`,
          status: "error",
        });
        setExitStatusAnd$0("error");
        return;
      }
      try {
        const pluginResult = plugin.run(result.val.payload);
        updateReplHistory({
          stdin: input,
          stdout: pluginResult.stdout,
          stderr: pluginResult.stderr,
          status: pluginResult.status,
        });
        setExitStatusAnd$0(pluginResult.status, pluginResult.stdout);
      } catch (error) {
        updateReplHistory({
          stdin: input,
          stderr: `Error: ${error}`,
          status: "error",
        });
        setExitStatusAnd$0("error");
      }
      return;
    }

    // the result of the command is ready
    if (result.tag === "ready") {
      updateReplHistory({
        stdin: input,
        stdout: result.val.stdout,
        stderr: result.val.stderr,
        status: result.val.status,
      });
      setExitStatusAnd$0(result.val.status, result.val.stdout);
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
