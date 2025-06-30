# WebAssembly Component Model Experiments

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

<p align="center"><a href="https://topheman.github.io/webassembly-component-model-experiments/"><img src="./packages/web-host/public/wasi.png" alt="Demo" /></a></p>
<p align="center">
  Check the online demo at<br/><a href="https://topheman.github.io/webassembly-component-model-experiments/">topheman.github.io/webassembly-component-model-experiments</a>
</p>

## Usage

### cli-host (rust)

#### Install

```bash
# Install the cli-host binary - todo: make it available on crates.io
cargo install --git https://github.com/topheman/webassembly-component-model-experiments.git --branch main cli-host
```

#### Run

```bash
./target/debug/cli-host\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/generated/plugin_greet.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/generated/plugin_ls.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/generated/plugin_echo.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/generated/plugin_weather.wasm
```

### web-host (typescript)

Go check [topheman.github.io/webassembly-component-model-experiments](https://topheman.github.io/webassembly-component-model-experiments) online demo.

## Development

### Prerequisites

- Rust 1.87+
- Node.js 22.6.0+ (needs `--experimental-strip-types` flag)
- [just](https://github.com/casey/just?tab=readme-ov-file#installation)

### Setup

```bash
# Add WebAssembly targets
rustup target add wasm32-unknown-unknown wasm32-wasip1
```

```bash
npm install
```

### cli-host (rust)

#### Build

```bash
just build
```

This will (see [justfile](./justfile)):

- compile the cli-host crate from rust to a binary file
- compile the repl-logic-guest crate from rust to wasm
- compile the plugin-* crates from rust to wasm

#### Run

```bash
./target/debug/cli-host\
  --plugins ./target/wasm32-wasip1/debug/plugin_greet.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_ls.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_echo.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_weather.wasm
```

This will run the `cli-host` binary which will itself:

- load and compile the `repl_logic_guest.wasm` file inside the embedded `wasmtime` engine injecting the [`host-api`](./wit/host-api.wit) interface
- load and compile the `plugin_*.wasm` files into the engine, injecting the [`plugin-api`](./wit/plugin-api.wit) interface
- launch the REPL loop executing the code from the `repl_logic_guest.wasm` file which will:
  - readline from the user
  - parse the command
  - dispatch the command to the plugin(s) if needed (run the `run`, `man` functions of the plugins via the [`host-api`](./wit/host-api.wit) interface)
  - display the result

You can pass the `--dir` argument to specify the directory to be preopened. By default, it will be the current directory.
You can pass the `--debug` argument to run the host in debug mode.

Other example:

```bash
./target/debug/cli-host\
  --plugins ./target/wasm32-wasip1/debug/plugin_ls.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_echo.wasm\
  --dir /tmp
```

#### Test

```bash
# Runs unit tests and e2e tests on the REPL
just test
```

#### Make a rust plugin

```bash
cargo component new --lib crates/plugin-hello
```

### web-host (typescript)

#### Dev

```bash
npm run web-host:dev
```

This Will (see [packages/web-host/package.json](./packages/web-host/package.json)):

- generate types from the [wit](./wit) files using the [jco](https://github.com/bytecodealliance/jco) tool
- build the plugins from rust to wasm (so that you don't have to do it manually)
- build the repl-logic-guest from rust to wasm (so that you don't have to do it manually)
- copy the wasm files in `target/wasm32-wasip1/release` to the `packages/web-host/src/wasm/generated` directory
- transpile the wasm files to javascript using the [jco](https://github.com/bytecodealliance/jco) tool into `packages/web-host/src/wasm/generated/*/transpiled` (this is the glue code wrapping the wasm files which is needed to interact with in the browser or node)
- start the vite dev server

Go to [http://localhost:5173](http://localhost:5173) to see the web host.

#### Build

```bash
npm run web-host:build
```

Will do the same as the dev command, small changes:

- the build tasks called on the rust side are `just *-release` (release mode)
- it doesn't start the vite dev server, it builds the static files in the `dist` directory

You can then run `npm run web-host:preview` to preview the build.

## Developer experience

### Formating and linting

- I use [biome](https://biomejs.dev/) for formating and linting the TypeScript code
- I use cargo fmt for formating the rust code
- They are both configured to run on save in the editor

### Git hooks

- I use [husky](https://github.com/typicode/husky) to run lint-staged on pre-commit
- I use [lint-staged](https://github.com/okonet/lint-staged) to run linting and formating on the changed files - the following are automatically run on pre-commit:
  - formating / linting the TypeScript code
  - formating the rust code
  - typechecking the TypeScript code

## Resources

### Optional tools

Those are **optional** tools that are handy for WebAssembly development:

- [cargo-binstall](https://github.com/cargo-bins/cargo-binstall?tab=readme-ov-file#installation)
- [cargo component 0.21.1+](https://github.com/bytecodealliance/cargo-component?tab=readme-ov-file#installation)
- [wasm-tools 1.235.0](https://github.com/bytecodealliance/wasm-tools?tab=readme-ov-file#installation)
- [wasm-opt 116](https://github.com/WebAssembly/binaryen?tab=readme-ov-file#installation)

```bash
# latest versions
cargo binstall cargo-component wasm-tools wasm-opt
```

```bash
# specific versions I used for this project
cargo binstall cargo-component@0.21.1 wasm-tools@1.235.0 wasm-opt@116
```
