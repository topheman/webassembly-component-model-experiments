import { useWasm } from "../hooks/wasm";
import { Repl } from "./Repl";

interface ReplPageProps {
  onBackToHome: () => void;
}

export const ReplPage = ({ onBackToHome }: ReplPageProps) => {
  const wasm = useWasm();

  return (
    <div className="container mx-auto px-4 max-w-4xl">
      <div className="sticky top-0 py-2 bg-gray-50">
        <div className="flex items-center justify-between md:mb-10 my-2">
          <h2 className="text-lg font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-[var(--color-wasi-purple)] to-[var(--color-wasi-violet)] drop-shadow">
            REPL Interface
          </h2>
          <button
            type="button"
            onClick={onBackToHome}
            className="cursor-pointer inline-flex items-center gap-2 bg-gradient-to-r from-[var(--color-wasi-accent)] to-[var(--color-wasi-violet)] text-white font-semibold px-4 py-1 rounded-full shadow hover:from-[var(--color-wasi-violet)] hover:to-[var(--color-wasi-purple)] transition-all"
          >
            <span className="text-lg">←</span> Back to Home
          </button>
        </div>
      </div>
      {wasm.status === "loading" && <div>Loading...</div>}
      {wasm.status === "ready" && <Repl engine={wasm.engine} />}
      {wasm.status === "error" && <div>Error</div>}
    </div>
  );
};
