import { useState } from "react";
import { useReplLogic } from "../hooks/replLogic";
import type { WasmEngine } from "../hooks/wasm";

export function Repl({ engine }: { engine: WasmEngine }) {
  console.log("Repl", engine);
  const [input, setInput] = useState("");
  const { handleInput, replHistory } = useReplLogic({ engine });

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const input = formData.get("input") as string;
    console.log("input", input);
    handleInput(input);
  }

  return (
    <div>
      <div>
        <form onSubmit={handleSubmit}>
          <div className="flex items-center gap-2">
            <input
              name="input"
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              className="border border-gray-300 rounded-md p-2 w-full"
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
      <div>
        <pre>{JSON.stringify(replHistory, null, 2)}</pre>
      </div>
    </div>
  );
}
