#!/usr/bin/env node --experimental-strip-types --no-warnings
import fs from "node:fs";
import path from "node:path";
import { program } from "commander";

const wasmFiles: Array<{ debug: string; release: string }> = [
  {
    debug: "target/wasm32-wasip1/debug/plugin_echo.wasm",
    release: "target/wasm32-wasip1/release/plugin_echo.wasm",
  },
  {
    debug: "target/wasm32-wasip1/debug/plugin_greet.wasm",
    release: "target/wasm32-wasip1/release/plugin_greet.wasm",
  },
  {
    debug: "target/wasm32-wasip1/debug/plugin_ls.wasm",
    release: "target/wasm32-wasip1/release/plugin_ls.wasm",
  },
  {
    debug: "target/wasm32-wasip1/debug/plugin_weather.wasm",
    release: "target/wasm32-wasip1/release/plugin_weather.wasm",
  },
  {
    debug: "target/wasm32-wasip1/debug/repl_logic_guest.wasm",
    release: "target/wasm32-wasip1/release/repl_logic_guest.wasm",
  },
];

function prepareWasmFiles({ mode }: { mode: "debug" | "release" }) {
  console.log(`Preparing wasm files for mode: ${mode}`);
  const workspaceRoot = path.join(import.meta.dirname, "..", "..", "..");
  const targetDir = path.join(
    workspaceRoot,
    "packages",
    "web-host",
    "src",
    "wasm",
  );
  if (!fs.existsSync(targetDir)) {
    fs.mkdirSync(targetDir);
  }
  for (const wasmFile of wasmFiles) {
    const copyFrom = path.join(workspaceRoot, wasmFile[mode]);
    const copyTo = path.join(targetDir, path.basename(wasmFile[mode]));
    try {
      fs.copyFileSync(copyFrom, copyTo);
    } catch (_e) {
      console.log("");
      console.error(`Failed to copy ${copyFrom} to ${copyTo}`);
      console.log("");
      console.error(
        `Please run the command: just ${mode === "debug" ? "build" : "build-release"}`,
      );
      console.log("");
      process.exit(1);
    }
    console.log(`Copied ${copyFrom} to ${copyTo}`);
  }
}

const ACCEPTED_MODES = ["debug", "release"];

function assertModeIsValid(mode: string): asserts mode is "debug" | "release" {
  if (!ACCEPTED_MODES.includes(mode)) {
    throw new Error(
      `Error: --mode must be one of: ${ACCEPTED_MODES.join(", ")}.`,
    );
  }
}

function run() {
  program
    .description("Prepare wasm files for the web host")
    .requiredOption("-m, --mode <mode>", "Mode to prepare wasm files for")
    .action((options) => {
      const { mode } = options;
      assertModeIsValid(mode);
      prepareWasmFiles({ mode });
    });

  program.parse();
}

run();
