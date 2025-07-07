import { Play, WandSparkles } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { useReplHistory } from "../hooks/replHistory";
import { useReplLogic } from "../hooks/replLogic";
import type { WasmEngine } from "../hooks/wasm";
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
    () => "help",
    () => "man weather",
  ];
  return commands[Math.floor(Math.random() * commands.length)]();
}

export function Repl({
  engine,
  className,
}: {
  engine: WasmEngine;
  className?: string;
}) {
  console.log("Repl", engine);
  const historyRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const formRef = useRef<HTMLFormElement>(null);
  const [input, setInput] = useState("");
  const { handleInput, commandRunning } = useReplLogic({ engine });
  const [inputFocus, setInputFocus] = useState(false);
  const { history } = useReplHistory();

  function handleSubmit(
    event: Pick<
      React.FormEvent<HTMLFormElement>,
      "preventDefault" | "currentTarget"
    >,
  ) {
    event.preventDefault();
    if (commandRunning) {
      return;
    }
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
      console.log("scrollHeight", historyRef.current.scrollHeight);
      historyRef.current.scrollLeft = 0;
    }
  }, [history]);

  useEffect(() => {
    if (!inputFocus) {
      window.scrollTo(0, 0);
    }
  }, [inputFocus]);

  return (
    <div className={className}>
      <ReplHistory
        ref={historyRef}
        className="fixed top-[80px] bottom-[100px] overflow-y-scroll max-w-4xl w-full pr-8"
        history={history}
      />
      <div className="fixed bottom-0 left-0 right-0 bg-white border-t border-gray-200 p-4 md:max-w-4xl mx-auto md:border">
        {/** biome-ignore lint/a11y/useSemanticElements: no use of <search> */}
        <form ref={formRef} onSubmit={handleSubmit} role="search" action="">
          <div className="flex items-center gap-2">
            <div
              className={`
                relative w-full
                after:content-[''] after:absolute after:right-3 after:top-1/2 after:-translate-y-1/2
                after:w-5 after:h-5 after:rounded-full after:border-2 after:border-primary after:border-t-transparent
                after:animate-spin after:hidden
                data-[running=true]:after:block
              `}
              data-running={commandRunning}
            >
              <input
                name="input"
                type="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                ref={inputRef}
                className="border border-gray-300 rounded-md p-2 w-full pr-10"
                onFocus={() => setInputFocus(true)}
                onBlur={() => setInputFocus(false)}
              />
            </div>
            <button
              type="submit"
              className="cursor-pointer bg-primary text-white px-4 py-2 rounded-md"
              title="Run"
            >
              <Play />
            </button>
            <button
              onClick={() => {
                if (commandRunning) {
                  return;
                }
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
              className="cursor-pointer bg-primary text-white px-4 py-2 rounded-md"
              title="Run random command"
            >
              <WandSparkles />
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
