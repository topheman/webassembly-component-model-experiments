interface ReplPageProps {
  onBackToHome: () => void;
}

export const ReplPage = ({ onBackToHome }: ReplPageProps) => (
  <div className="container mx-auto px-4 py-8 max-w-6xl">
    <div className="flex items-center justify-between mb-8">
      <h2 className="text-3xl font-bold text-primary">REPL Interface</h2>
      <button
        type="button"
        onClick={onBackToHome}
        className="text-primary hover:text-primary-700 transition-colors cursor-pointer"
      >
        ← Back to Home
      </button>
    </div>

    <div className="bg-white rounded-lg p-6 border border-gray-200 shadow-sm">
      <div className="text-center text-gray-600">
        <p className="text-lg mb-4">REPL interface coming soon...</p>
        <p className="text-sm">
          This is where the WebAssembly-powered REPL will be implemented.
        </p>
      </div>
    </div>
  </div>
);
