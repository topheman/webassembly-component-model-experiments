import { useState } from "react";
import viteLogo from "/vite.svg";
import reactLogo from "./assets/react.svg";

function App() {
  const [count, setCount] = useState(0);

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-900 text-white p-4">
      <div className="max-w-4xl mx-auto text-center">
        <div className="flex justify-center items-center gap-8 mb-8">
          <a
            href="https://vite.dev"
            target="_blank"
            rel="noopener noreferrer"
            className="group"
          >
            <img
              src={viteLogo}
              className="h-24 p-6 transition-all duration-300 group-hover:drop-shadow-[0_0_2em_#646cffaa] group-hover:scale-110"
              alt="Vite logo"
            />
          </a>
          <a
            href="https://react.dev"
            target="_blank"
            rel="noopener noreferrer"
            className="group"
          >
            <img
              src={reactLogo}
              className="h-24 p-6 transition-all duration-300 group-hover:drop-shadow-[0_0_2em_#61dafbaa] group-hover:scale-110 animate-spin-slow"
              alt="React logo"
            />
          </a>
        </div>

        <h1 className="text-5xl font-bold mb-8 bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
          Vite + React
        </h1>

        <div className="bg-gray-800 rounded-lg p-8 mb-8 border border-gray-700">
          <button
            type="button"
            onClick={() => setCount((count) => count + 1)}
            className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-6 rounded-lg border border-transparent transition-all duration-200 hover:border-blue-400 focus:outline-none focus:ring-4 focus:ring-blue-500/50 mb-4"
          >
            count is {count}
          </button>
          <p className="text-gray-300">
            Edit{" "}
            <code className="bg-gray-700 px-2 py-1 rounded text-sm">
              src/App.tsx
            </code>{" "}
            and save to test HMR
          </p>
        </div>

        <p className="text-gray-400 text-sm">
          Click on the Vite and React logos to learn more
        </p>
      </div>
    </div>
  );
}

export default App;
