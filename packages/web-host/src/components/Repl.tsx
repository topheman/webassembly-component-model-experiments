import { useEffect, useRef, useState } from "react";
import { useReplLogic } from "../hooks/replLogic";
import type { WasmEngine } from "../hooks/wasm";
import { cn } from "../utils/css";
import { ReplHistory } from "./ReplHistory";

export function Repl({ engine }: { engine: WasmEngine }) {
  console.log("Repl", engine);
  const historyRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const [input, setInput] = useState("");
  const { handleInput, replHistory } = useReplLogic({ engine });
  const [inputFocus, setInputFocus] = useState(false);

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const input = formData.get("input") as string;
    console.log("input", input);
    handleInput(input);
  }

  // biome-ignore lint/correctness/useExhaustiveDependencies: dependency necessary
  useEffect(() => {
    // scroll to the bottom of the history each time an entry is added
    if (historyRef.current) {
      historyRef.current.scrollTop = historyRef.current.scrollHeight;
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
        <form onSubmit={handleSubmit}>
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
              className="bg-blue-500 text-white px-4 py-2 rounded-md"
            >
              Run
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
