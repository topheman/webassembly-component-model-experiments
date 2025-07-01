# cli-host

Command-line interface host for Terminal REPL with plugin system (using WebAssembly Component Model).

The goal of this project is to demonstrate the power of the WebAssembly Component Model, with more than a simple hello world.

It is a basic REPL, with a plugin system where:

- plugins can be written in any language compiling to WebAssembly
- plugins are sandboxed by default
- the REPL logic is written in Rust, it also compiles to WebAssembly

There are two kinds of hosts:

- a **CLI host**, written in Rust running in a terminal **(this crate)**
- a web host, written in TypeScript running in a browser (see online demo at [topheman.github.io/webassembly-component-model-experiments](https://topheman.github.io/webassembly-component-model-experiments))

Those hosts then run the same codebase which is compiled to WebAssembly:

- the REPL logic
- the plugins

More details on the github repo: [topheman/webassembly-component-model-experiments](https://github.com/topheman/webassembly-component-model-experiments).

## Install

```bash
cargo install cli-host
```

## Usage

Run the CLI host, loading the plugins from the web (you can also load them from local files).

```bash
cli-host\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/generated/plugin_greet.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/generated/plugin_ls.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/generated/plugin_echo.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/generated/plugin_weather.wasm
```
