import { Book, Github } from "lucide-react";
import { QRCodeSVG } from "qrcode.react";
import { useEffect, useState } from "react";

interface HomePageProps {
  onStartRepl: () => void;
}

function StartReplButton({
  onStartRepl,
  "data-testid-button": testId,
}: {
  onStartRepl: () => void;
  "data-testid-button": string;
}) {
  return (
    <div className="text-center mb-14">
      <button
        type="button"
        onClick={onStartRepl}
        className="cursor-pointer bg-gradient-to-r from-[var(--color-wasi-violet)] to-[var(--color-wasi-purple)] shadow-lg hover:from-[var(--color-wasi-purple)] hover:to-[var(--color-wasi-violet)] text-white font-bold py-4 px-12 rounded-full text-xl transition-transform duration-300 ease-out transform hover:scale-[1.3] focus:outline-none focus:ring-4 focus:ring-[var(--color-wasi-violet)]/40 animate-[pulse110_2s_ease-in-out_infinite]"
        data-testid={testId}
      >
        ✨ Start REPL ✨
      </button>
    </div>
  );
}

export const HomePage = ({ onStartRepl }: HomePageProps) => {
  const [targetUrl, setTargetUrl] = useState<string | null>(null);

  useEffect(() => {
    const [url] = window.location.href.split("#");
    setTargetUrl(url);
  }, []);

  return (
    <div className="container mx-auto px-4 py-12 max-w-4xl">
      <StartReplButton
        onStartRepl={onStartRepl}
        data-testid-button="start-repl-button-top"
      />

      <div className="bg-white rounded-2xl p-8 border border-[var(--color-wasi-purple)]/20 shadow-lg mb-12">
        <h3 className="text-2xl font-bold mb-4">What is it?</h3>
        <p className="text-gray-700 mb-6 text-lg">
          A <strong>REPL with a plugin system</strong> that demonstrates the
          WebAssembly Component Model's power. Plugins can be written in{" "}
          <strong>any language that compiles to WASM</strong> and run in a{" "}
          <strong>sandboxed environment</strong>.
          <br />
          Existing examples are either <strong>too simple</strong> or{" "}
          <strong>too complex</strong> - this project goes{" "}
          <strong>beyond "hello world"</strong> with a{" "}
          <strong>practical implementation</strong>.
        </p>

        <h3 className="text-2xl font-bold mb-4">This Web Version</h3>
        <p className="text-gray-700 mb-6 text-lg">
          You're using the <strong>web version</strong> built with
          React/TypeScript. There's also a <strong>Rust CLI version</strong>.
          Both load the <strong>same WebAssembly components</strong>,
          demonstrating true cross-platform compatibility.
        </p>

        <h4 className="text-xl font-bold mb-3">Key Features</h4>
        <ul className="text-gray-700 space-y-2 mb-3 text-base pl-4 list-disc">
          <li>
            <strong>WebAssembly Component Model</strong> architecture
          </li>
          <li>
            <strong>Dual host support</strong> (CLI + Web)
          </li>
          <li>
            <strong>Plugin system</strong> with sandboxed WASM components
          </li>
          <li>
            <strong>Language-agnostic</strong> plugins
          </li>
          <li>
            <strong>Virtual filesystem</strong> on the browser with WASI shim
            for filesystem (including <strong>WRITE</strong> operations)
          </li>
          <li>
            <strong>Modern React + TypeScript</strong> interface
          </li>
        </ul>

        <div className="flex flex-row gap-4 items-center justify-center mt-8">
          <a
            href="https://github.com/topheman/webassembly-component-model-experiments"
            rel="noopener noreferrer"
            className="text-[var(--color-wasi-purple)] font-medium transition-colors bg-primary-50 p-2 rounded-md flex items-center gap-2"
            title="GitHub Repository"
          >
            <Github className="w-4 h-4" />
            <span>GitHub Repository</span>
          </a>
          <a
            href="https://dev.to/topheman/webassembly-component-model-building-a-plugin-system-58o0"
            rel="noopener noreferrer"
            className="text-[var(--color-wasi-purple)] font-medium transition-colors bg-primary-50 p-2 rounded-md flex items-center gap-2"
            title="Blog Post"
          >
            <Book className="w-4 h-4" />
            <span>Blog Post</span>
          </a>
        </div>
      </div>

      <StartReplButton
        onStartRepl={onStartRepl}
        data-testid-button="start-repl-button-bottom"
      />

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
          </div>
          <p className="text-gray-700 text-[0.8rem] text-center mt-2">
            <a
              href={targetUrl}
              rel="noopener noreferrer"
              className="underline break-all text-color-white"
            >
              {targetUrl}
            </a>
          </p>
        </div>
      )}
    </div>
  );
};
