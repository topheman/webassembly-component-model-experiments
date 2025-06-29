import { useEffect, useState } from "react";

type Page = "home" | "repl";

export const useHashNavigation = () => {
  const [currentPage, setCurrentPage] = useState<Page>(() => {
    // Initialize based on URL hash
    const hash = window.location.hash.slice(1);
    return hash === "repl" ? "repl" : "home";
  });

  // Sync URL hash with current page
  useEffect(() => {
    const newHash = currentPage === "repl" ? "#repl" : "#home";
    if (window.location.hash !== newHash) {
      window.history.replaceState(null, "", newHash);
    }
  }, [currentPage]);

  // Listen for hash changes (back/forward buttons)
  useEffect(() => {
    const handleHashChange = () => {
      const hash = window.location.hash.slice(1);
      const newPage: Page = hash === "repl" ? "repl" : "home";
      setCurrentPage(newPage);
    };

    window.addEventListener("hashchange", handleHashChange);
    return () => window.removeEventListener("hashchange", handleHashChange);
  }, []);

  const navigateToRepl = () => {
    setCurrentPage("repl");
    window.history.pushState(null, "", "#repl");
  };

  const navigateToHome = () => {
    setCurrentPage("home");
    window.history.pushState(null, "", "#home");
  };

  return {
    currentPage,
    navigateToRepl,
    navigateToHome,
  };
};
