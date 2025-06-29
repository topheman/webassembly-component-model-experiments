import { HomePage } from "./components/HomePage";
import { ReplPage } from "./components/ReplPage";
import { WasmProvider } from "./components/Wasm/Wasm";
import { useHashNavigation } from "./hooks/useHashNavigation";

const Header = () => (
  <header className="bg-primary text-white shadow-lg">
    <div className="container mx-auto px-4 py-4">
      <div className="flex items-center gap-3">
        <h1 className="text-xl font-bold">WebAssembly Component Model REPL</h1>
      </div>
    </div>
  </header>
);

const Footer = () => (
  <footer className="bg-gray-200 text-gray-300 py-6 mt-auto">
    <div className="container mx-auto text-center">
      <p className="text-sm text-gray-800">
        <a
          href="https://topheman.github.io/me/"
          target="_blank"
          rel="noopener noreferrer"
          className="no-underline text-md"
        >
          @ 2025 - Christophe Rosset
        </a>
      </p>
    </div>
  </footer>
);

function App() {
  const { currentPage, navigateToRepl, navigateToHome } = useHashNavigation();

  return (
    <WasmProvider>
      <div className="min-h-screen flex flex-col bg-gray-50 text-gray-900">
        <Header />
        <main className="flex-1">
          {currentPage === "home" ? (
            <HomePage onStartRepl={navigateToRepl} />
          ) : (
            <ReplPage onBackToHome={navigateToHome} />
          )}
        </main>
        <Footer />
      </div>
    </WasmProvider>
  );
}

export default App;
