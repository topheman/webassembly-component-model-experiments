import { Github, User } from "lucide-react";
import { QRCodeSVG } from "qrcode.react";
import { useEffect, useState } from "react";

interface HomePageProps {
  onStartRepl: () => void;
}

export const HomePage = ({ onStartRepl }: HomePageProps) => {
  const [targetUrl, setTargetUrl] = useState<string | null>(null);

  useEffect(() => {
    const [url] = window.location.href.split("#");
    setTargetUrl(url);
  }, []);

  return (
    <div className="container mx-auto px-4 py-12 max-w-4xl">
      <div className="text-center mb-14">
        <button
          type="button"
          onClick={onStartRepl}
          className="cursor-pointer bg-gradient-to-r from-[var(--color-wasi-violet)] to-[var(--color-wasi-purple)] shadow-lg hover:from-[var(--color-wasi-purple)] hover:to-[var(--color-wasi-violet)] text-white font-bold py-4 px-12 rounded-full text-xl transition-transform duration-300 ease-out transform hover:scale-[1.3] focus:outline-none focus:ring-4 focus:ring-[var(--color-wasi-violet)]/40 animate-[pulse110_2s_ease-in-out_infinite]"
        >
          ✨ Start REPL ✨
        </button>
      </div>

      <div className="bg-white rounded-2xl p-8 border border-[var(--color-wasi-purple)]/20 shadow-lg mb-12">
        <h3 className="text-2xl font-bold mb-4">What is it?</h3>
        <p className="text-gray-700 mb-4 text-lg">
          The WebAssembly Component Model enables interoperable WebAssembly
          libraries, but existing examples are either{" "}
          <strong>too simple</strong> or <strong>too complex</strong>. This
          project demonstrates its power with a{" "}
          <strong>practical implementation</strong> that goes{" "}
          <strong>beyond "hello world"</strong>.
        </p>
        <p className="text-gray-700 mb-6 text-lg">
          It's a{" "}
          <strong>REPL (Read-Eval-Print Loop) with a plugin system</strong>{" "}
          where plugins can be written in{" "}
          <strong>any language that compiles to WebAssembly</strong>. The
          plugins are <strong>sandboxed by default</strong>, and the core logic
          itself is written in Rust and also compiles to WebAssembly.
        </p>

        <h3 className="text-2xl font-bold mb-4">This Web Version</h3>
        <p className="text-gray-700 mb-4 text-lg">
          You're currently using the <strong>web version</strong> of the REPL,
          built with React and TypeScript. There's also a corresponding{" "}
          <strong>Rust CLI version</strong> that runs in the terminal. Both
          versions load and execute the{" "}
          <strong>same WebAssembly components</strong> - the REPL logic and
          plugins are compiled once and run in both environments.
        </p>
        <p className="text-gray-700 mb-6 text-lg">
          This demonstrates how the WebAssembly Component Model enables{" "}
          <strong>true cross-platform compatibility</strong> - the same code
          runs in browsers and terminals with different host implementations.
        </p>

        <h4 className="text-xl font-bold mb-3">Key Features</h4>
        <ul className="text-gray-700 space-y-2 mb-3 text-base pl-4 list-disc">
          <li>WebAssembly Component Model architecture</li>
          <li>Dual host support (CLI + Web)</li>
          <li>Plugin system with sandboxed WebAssembly components</li>
          <li>
            Language-agnostic plugins (any language that compiles to WASM)
          </li>
          <li>Modern React + TypeScript web interface</li>
        </ul>

        <div className="flex flex-row gap-4 items-center justify-center mt-8">
          <a
            href="https://github.com/topheman/webassembly-component-model-experiments"
            target="_blank"
            rel="noopener noreferrer"
            className="text-[var(--color-wasi-purple)] font-medium transition-colors bg-primary-50 p-2 rounded-md flex items-center gap-2"
          >
            <Github className="w-4 h-4" />
            <span>GitHub Repository</span>
          </a>
          <a
            href="https://topheman.github.io/me/"
            target="_blank"
            rel="noopener noreferrer"
            className="text-[var(--color-wasi-purple)] font-medium transition-colors bg-primary-50 p-2 rounded-md flex items-center gap-2"
          >
            <User className="w-4 h-4" />
            <span>My Portfolio</span>
          </a>
        </div>
      </div>

      {targetUrl && (
        <div className="flex flex-col items-center justify-center">
          <div className="bg-white rounded-2xl p-8 border border-[var(--color-wasi-violet)]/20 shadow-lg">
            <div className="mb-4">
              <QRCodeSVG
                value={targetUrl}
                size={180}
                level="M"
                fgColor="#6C63FF"
                includeMargin={false}
              />
            </div>
            <p className="text-gray-700 text-[0.8rem] text-center mt-2">
              <a
                href={targetUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="underline break-all text-color-white"
              >
                {targetUrl}
              </a>
            </p>
          </div>
        </div>
      )}
    </div>
  );
};
