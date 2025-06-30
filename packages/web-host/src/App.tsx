import { HomePage } from "./components/HomePage";
import { ReplPage } from "./components/ReplPage";
import { useHashNavigation } from "./hooks/useHashNavigation";
import { WasmProvider } from "./hooks/wasm";

const Header = () => (
  <header className="bg-gradient-to-r from-[var(--color-wasi-purple)] to-[var(--color-wasi-violet)] text-white shadow-lg">
    <div className="container mx-auto px-4 py-4 flex items-center gap-4">
      <img
        src="./wasi.png"
        alt="WASI Logo"
        className="h-12 w-auto shadow-md bg-white p-1 object-contain"
      />
      <h1 className="text-2xl font-extrabold tracking-tight drop-shadow-sm">
        WebAssembly Component Model Experiments
      </h1>
    </div>
  </header>
);

const Footer = () => (
  <footer className="bg-gradient-to-r bg-gray-200 text-white py-6 mt-auto">
    <div className="container mx-auto text-center">
      <p className="text-sm opacity-80">
        <a
          href="https://topheman.github.io/me/"
          target="_blank"
          rel="noopener noreferrer"
          className="no-underline text-md hover:underline text-primary"
        >
          © 2025 - Christophe Rosset
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
