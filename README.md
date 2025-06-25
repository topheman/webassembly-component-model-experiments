# WebAssembly Component Model REPL

The goal of this project is to demonstrate the power of the WebAssembly Component Model, with more than a simple hello world.

It is a basic REPL, with a plugin system where:

- plugins can be written in any language compiling to WebAssembly
- plugins are sandboxed by default
- the REPL logic is written in Rust, it also compiles to WebAssembly

There are two kinds of hosts:

- a CLI host, written in Rust running in a terminal
- a web host, written in TypeScript running in a browser

Those hosts then run the same codebase which is compiled to WebAssembly:

- the REPL logic
- the plugins

## Usage

TODO: add usage instructions for both hosts

## Development

### Prerequisites

- Rust 1.87+
- Node.js 22+

### Setup

#### Rust

- [just](https://github.com/casey/just?tab=readme-ov-file#installation)
- [cargo component 0.21.1+](https://github.com/bytecodealliance/cargo-component?tab=readme-ov-file#installation)
- [wasm-tools 1.235.0](https://github.com/bytecodealliance/wasm-tools?tab=readme-ov-file#installation)
- [wasm-opt 116](https://github.com/WebAssembly/binaryen?tab=readme-ov-file#installation)

Add WebAssembly targets:

```bash
rustup target add wasm32-unknown-unknown wasm32-wasip1
```

#### JavaScript

```bash
npm install
```

### Build

Run `just build` to build the rust part of the project.

Run `npm run build` for the JavaScript part of the project.

### Run

Example for the CLI host:

```bash
./target/debug/cli-host\
  --plugins ./target/wasm32-wasip1/debug/plugin_greet.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_ls.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_echo.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_weather.wasm
```

You can pass the `--dir` argument to specify the directory to be preopened. By default, it will be the current directory.

You can pass the `--debug` argument to run the host in debug mode.

Other example:

```bash
./target/debug/cli-host\
  --plugins ./target/wasm32-wasip1/debug/plugin_ls.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_echo.wasm\
  --dir /tmp
```

### Test

```bash
just test
```

### Make a rust plugin

```bash
cargo component new --lib crates/plugin-hello
```
