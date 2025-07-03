import { HomePage } from "./components/HomePage";
import { ReplPage } from "./components/ReplPage";
import { useHashNavigation } from "./hooks/navigation";
import { WasmProvider } from "./hooks/wasm";
import { cn } from "./utils/css";

const Header = (props: React.HTMLAttributes<HTMLDivElement>) => {
  const { className, ...rest } = props;
  return (
    <header
      className={cn(
        "bg-gradient-to-r from-[var(--color-wasi-purple)] to-[var(--color-wasi-violet)] text-white shadow-lg",
        className,
      )}
      {...rest}
    >
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
};

const Footer = (props: React.HTMLAttributes<HTMLDivElement>) => {
  const { className, ...rest } = props;
  return (
    <footer
      className={cn(
        "bg-gradient-to-r bg-gray-200 text-white py-6 mt-auto",
        className,
      )}
      {...rest}
    >
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
};

function App() {
  const { currentPage, navigateToRepl, navigateToHome } = useHashNavigation();

  return (
    <WasmProvider>
      <div className="min-h-screen flex flex-col bg-gray-50 text-gray-900">
        <Header
          className="data-[page=repl]:md:block data-[page=repl]:hidden"
          data-page={currentPage}
        />
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
