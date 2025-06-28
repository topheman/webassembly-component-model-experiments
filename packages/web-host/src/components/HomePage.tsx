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
    <div className="container mx-auto px-4 py-8 max-w-4xl">
      <div className="text-center mb-12">
        <h2 className="text-4xl font-bold text-primary mb-4">
          WebAssembly Component Model REPL
        </h2>
        <p className="text-xl text-gray-600 mb-8">
          A web-based REPL interface for WebAssembly Component Model
        </p>
      </div>

      <div className="text-center mb-12">
        <button
          type="button"
          onClick={onStartRepl}
          className="bg-primary cursor-pointer hover:bg-primary-700 text-white font-bold py-4 px-8 rounded-lg transition-colors duration-200 focus:outline-none focus:ring-4 focus:ring-primary/50"
        >
          ✨ Start REPL ✨
        </button>
      </div>

      <div className="bg-white rounded-lg p-6 border border-gray-200 shadow-sm mb-12">
        <h3 className="text-2xl font-bold text-primary mb-4">What is it?</h3>
        <p className="text-gray-700 mb-6">
          This is a web application that provides a REPL-like interface for
          interacting with WebAssembly components. The implementation of the
          REPL logic and plugins are built in any language that can be compiled
          to WebAssembly.
        </p>
        <p className="text-gray-700 mb-6">
          Built with React, TypeScript, and Tailwind CSS, this project
          demonstrates the power and flexibility of the WebAssembly Component
          Model.
        </p>

        <h4 className="text-xl font-bold text-primary mb-3">Features</h4>
        <ul className="text-gray-700 space-y-2 mb-3">
          <li>• Web-based REPL interface</li>
          <li>• WebAssembly Component Model support</li>
          <li>• Plugin system for extensibility</li>
          <li>• Modern React + TypeScript stack</li>
          <li>• Responsive design</li>
        </ul>
        <p className="text-gray-700 mb-3">
          🚧 <strong>This description is still in progress</strong> 🚧
        </p>
      </div>

      <div className="bg-white rounded-lg p-6 border border-gray-200 shadow-sm mb-8">
        <h3 className="text-2xl font-bold text-primary mb-4">Links</h3>
        <div className="space-y-3">
          <a
            href="https://github.com/topheman/webassembly-component-model-repl"
            target="_blank"
            rel="noopener noreferrer"
            className="block text-primary hover:text-primary-700 transition-colors"
          >
            📦 GitHub Repository
          </a>
          <a
            href="https://topheman.github.io/me/"
            target="_blank"
            rel="noopener noreferrer"
            className="block text-primary hover:text-primary-700 transition-colors"
          >
            👨‍💻 My Portfolio
          </a>
        </div>
      </div>

      {locationHref && (
        <div>
          <div className="flex justify-center">
            <div className="bg-white p-4 rounded-lg shadow-md">
              <QRCodeSVG
                value={locationHref}
                size={224}
                level="M"
                fgColor="#900000"
                includeMargin={false}
              />
            </div>
          </div>
          <p className="text-gray-700 text-sm text-center mt-4">
            <a
              href={locationHref}
              target="_blank"
              rel="noopener noreferrer"
              className="underline break-all"
            >
              {locationHref}
            </a>
          </p>
        </div>
      )}
    </div>
  );
};
