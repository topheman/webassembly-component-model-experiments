interface ReplPageProps {
  onBackToHome: () => void;
}

export const ReplPage = ({ onBackToHome }: ReplPageProps) => (
  <div className="container mx-auto px-4 py-12 max-w-4xl">
    <div className="flex items-center justify-between mb-10">
      <h2 className="text-4xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-[var(--color-wasi-purple)] to-[var(--color-wasi-violet)] drop-shadow">
        REPL Interface
      </h2>
      <button
        type="button"
        onClick={onBackToHome}
        className="cursor-pointer inline-flex items-center gap-2 bg-gradient-to-r from-[var(--color-wasi-accent)] to-[var(--color-wasi-violet)] text-white font-semibold px-6 py-2 rounded-full shadow hover:from-[var(--color-wasi-violet)] hover:to-[var(--color-wasi-purple)] transition-all"
      >
        <span className="text-lg">←</span> Back to Home
      </button>
    </div>
    <div className="bg-white rounded-2xl p-10 border border-[var(--color-wasi-purple)]/20 shadow-lg">
      <div className="text-center text-gray-600">
        <p className="text-xl mb-4 font-medium">
          REPL interface coming soon...
        </p>
        <p className="text-base">
          This is where the WebAssembly-powered REPL will be implemented.
        </p>
      </div>
    </div>
  </div>
);
