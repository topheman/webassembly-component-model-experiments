import { Play, RefreshCcw } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { useReplLogic } from "../hooks/replLogic";
import type { WasmEngine } from "../hooks/wasm";
import { cn } from "../utils/css";
import { ReplHistory } from "./ReplHistory";

function getRandomCommand() {
  const commands = [
    () => "echo foo",
    () => "echo $ROOT/$USER",
    () => `export USER=${Math.random() > 0.5 ? "Tophe" : "Topheman"}`,
    () => "ls",
    () => "weather Paris",
    () => "greet $USER",
    () => "azertyuiop",
    () => "echo $0",
    () => "echo $?",
    () => "man echo",
    () => "man ls",
    () => "man weather",
    () => "man greet",
    () => "man export",
    () => "help",
    () => "man help",
  ];
  return commands[Math.floor(Math.random() * commands.length)]();
}

export function Repl({ engine }: { engine: WasmEngine }) {
  console.log("Repl", engine);
  const historyRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const formRef = useRef<HTMLFormElement>(null);
  const [input, setInput] = useState("");
  const { handleInput, replHistory } = useReplLogic({ engine });
  const [inputFocus, setInputFocus] = useState(false);

  function handleSubmit(
    event: Pick<
      React.FormEvent<HTMLFormElement>,
      "preventDefault" | "currentTarget"
    >,
  ) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const input = formData.get("input") as string;
    if (input.trim() === "") {
      return;
    }
    console.log("input", input);
    handleInput(input);
    if (inputRef.current && inputFocus) {
      inputRef.current.select();
    }
  }

  // biome-ignore lint/correctness/useExhaustiveDependencies: dependency necessary
  useEffect(() => {
    // scroll to the bottom of the history each time an entry is added
    if (historyRef.current) {
      historyRef.current.scrollTop = historyRef.current.scrollHeight;
      historyRef.current.scrollLeft = 0;
    }
    // on mobile, scroll to the top of the window to gracefully handle the virtual keyboard appearing - the setTimeout is necessary (layout computing)
    setTimeout(() => {
      window.scrollTo(0, 0);
    }, 100);
  }, [replHistory, inputFocus]);

  return (
    <div>
      <ReplHistory
        ref={historyRef}
        className={cn(
          "static overflow-y-scroll md:max-h-[60vh] max-h-[75vh]",
          inputFocus && "max-h-[40vh]",
        )}
        history={replHistory}
      />
      <div>
        <form ref={formRef} onSubmit={handleSubmit}>
          <div className="flex items-center gap-2">
            <input
              name="input"
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              ref={inputRef}
              className="border border-gray-300 rounded-md p-2 w-full"
              onFocus={() => setInputFocus(true)}
              onBlur={() => setInputFocus(false)}
            />
            <button
              type="submit"
              className="cursor-pointer bg-blue-500 text-white px-4 py-2 rounded-md"
              title="Run"
            >
              <Play />
            </button>
            <button
              onClick={() => {
                setInput(getRandomCommand());
                // trigger the onSubmit event
                setTimeout(() => {
                  handleSubmit({
                    preventDefault: () => {},
                    // biome-ignore lint/style/noNonNullAssertion: formRef is not null
                    currentTarget: formRef.current!,
                  });
                }, 0);
              }}
              type="button"
              className="cursor-pointer bg-blue-500 text-white px-4 py-2 rounded-md"
              title="Run random command"
            >
              <RefreshCcw />
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
