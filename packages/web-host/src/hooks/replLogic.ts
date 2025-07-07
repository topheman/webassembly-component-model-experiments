import { setReplVar } from "repl:api/host-state";
import { useMemo, useState } from "react";
import type { ReplHistoryEntry, ReplStatus } from "../types";
import type { WasmEngine } from "../wasm/engine";
import { useReplHistory } from "./replHistory";

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
  setCommandRunning,
  addReplHistoryEntry,
}: {
  engine: WasmEngine;
  setCommandRunning: (running: boolean) => void;
  addReplHistoryEntry: (entry: ReplHistoryEntry) => void;
}) {
  return function handleInput(input: string) {
    addReplHistoryEntry({ stdin: input });
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
          addReplHistoryEntry({
            stderr: `Unknown command: ${result.val.payload}. Try \`help\` to see available commands.`,
            status: "error",
          });
          setExitStatusAnd$0("error");
          return;
        }
        const man = plugin.man();
        addReplHistoryEntry({
          stdout: man,
          status: "success",
        });
        setExitStatusAnd$0("success", man);
        return;
      }

      // a plugin command, we run it from the host
      const plugin = engine.getPlugin(result.val.command);
      if (!plugin) {
        addReplHistoryEntry({
          stderr: `Unknown command: ${result.val.command}. Try \`help\` to see available commands.`,
          status: "error",
        });
        setExitStatusAnd$0("error");
        return;
      }
      // we run the plugin command in a double requestAnimationFrame to defer
      // its execution to the next frame and let the user see `stdin` appear
      // in the history (the command output may take a while to appear)
      //
      // Didn't make it work with react transitions
      //
      // Note: all actions are sync for the moment.
      setCommandRunning(true);
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          try {
            const pluginResult = plugin.run(result.val.payload);
            addReplHistoryEntry({
              stdout: pluginResult.stdout,
              stderr: pluginResult.stderr,
              status: pluginResult.status,
            });
            setExitStatusAnd$0(pluginResult.status, pluginResult.stdout);
          } catch (error) {
            console.error(error);
            addReplHistoryEntry({
              stderr: `Error: ${error}`,
              status: "error",
            });
            setExitStatusAnd$0("error");
          } finally {
            setCommandRunning(false);
          }
        });
      });
      return;
    }

    // the result of the command is ready
    if (result.tag === "ready") {
      addReplHistoryEntry({
        stdout: result.val.stdout,
        stderr: result.val.stderr,
        status: result.val.status,
      });
      setExitStatusAnd$0(result.val.status, result.val.stdout);
    }
  };
}

export function useReplLogic({ engine }: { engine: WasmEngine }) {
  const [commandRunning, setCommandRunning] = useState(false);
  const { addEntry: addReplHistoryEntry } = useReplHistory();
  const handleInput = useMemo(
    () =>
      makeReplLogicHandler({ engine, setCommandRunning, addReplHistoryEntry }),
    [engine, addReplHistoryEntry],
  );

  return { handleInput, commandRunning };
}
