import { QRCodeSVG } from "qrcode.react";
import { useEffect, useState } from "react";

interface HomePageProps {
  onStartRepl: () => void;
}

export const HomePage = ({ onStartRepl }: HomePageProps) => {
  const [locationHref, setLocationHref] = useState<string | null>(null);

  useEffect(() => {
    setLocationHref(window.location.href);
  }, []);

  return (
    <div className="container mx-auto px-4 py-12 max-w-4xl">
      <div className="text-center mb-14">
        <h2 className="text-5xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-[var(--color-wasi-purple)] to-[var(--color-wasi-violet)] mb-3 drop-shadow">
          WebAssembly Component Model Experiments
        </h2>
        <p className="text-2xl text-gray-600 mb-8 font-medium">
          A web-based REPL interface for WebAssembly Component Model
        </p>
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
          This is a web application that provides a REPL-like interface for
          interacting with WebAssembly components. The implementation of the
          REPL logic and plugins are built in any language that can be compiled
          to WebAssembly.
        </p>
        <p className="text-gray-700 mb-6 text-lg">
          Built with React, TypeScript, and Tailwind CSS, this project
          demonstrates the power and flexibility of the WebAssembly Component
          Model.
        </p>
        <h4 className="text-xl font-bold mb-3">Features</h4>
        <ul className="text-gray-700 space-y-2 mb-3 text-base pl-4 list-disc">
          <li>Web-based REPL interface</li>
          <li>WebAssembly Component Model support</li>
          <li>Plugin system for extensibility</li>
          <li>Modern React + TypeScript stack</li>
          <li>Responsive design</li>
        </ul>
        <p className="font-semibold flex items-center gap-2">
          <strong>
            <span className="text-xl">🚧</span> This description is still in
            progress <span className="text-xl">🚧</span>
          </strong>
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-8 mb-10">
        <div className="bg-white rounded-2xl p-8 border border-[var(--color-wasi-violet)]/20 shadow-lg flex flex-col justify-between">
          <h3 className="text-2xl font-bold mb-4">Links</h3>
          <div className="space-y-3">
            <a
              href="https://github.com/topheman/webassembly-component-model-repl"
              target="_blank"
              rel="noopener noreferrer"
              className="block text-[var(--color-wasi-purple)] font-medium transition-colors"
            >
              📦 GitHub Repository
            </a>
            <a
              href="https://topheman.github.io/me/"
              target="_blank"
              rel="noopener noreferrer"
              className="block text-[var(--color-wasi-purple)] font-medium transition-colors"
            >
              👨‍💻 My Portfolio
            </a>
          </div>
        </div>
        {locationHref && (
          <div className="flex flex-col items-center justify-center bg-white rounded-2xl p-8 border border-[var(--color-wasi-violet)]/20 shadow-lg">
            <div className="mb-4">
              <QRCodeSVG
                value={locationHref}
                size={180}
                level="M"
                fgColor="#6C63FF"
                includeMargin={false}
              />
            </div>
            <p className="text-gray-700 text-sm text-center mt-2">
              <a
                href={locationHref}
                target="_blank"
                rel="noopener noreferrer"
                className="underline break-all text-color-white"
              >
                {locationHref}
              </a>
            </p>
          </div>
        )}
      </div>
    </div>
  );
};
