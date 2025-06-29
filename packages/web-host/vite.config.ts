import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "path";
import { defineConfig } from "vite";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "repl:api/http-client": path.resolve(
        __dirname,
        "./src/host/http-client.ts",
      ),
      "repl:api/host-state": path.resolve(
        __dirname,
        "./src/host/host-state.ts",
      ),
    },
  },
});
