# WebAssembly Component Model Experiments

[![Crates.io](https://img.shields.io/crates/v/pluginlab.svg)](https://crates.io/crates/pluginlab)
[![Demo](https://img.shields.io/badge/demo-online-blue.svg)](https://topheman.github.io/webassembly-component-model-experiments/)

> The WebAssembly Component Model is a broad-reaching architecture for building interoperable WebAssembly libraries, applications, and environments.

It is still very early days, but it is a very promising technology. However, the examples out there are either too simple or too complex.

The goal of this project is to demonstrate the power of the WebAssembly Component Model, with more than a simple hello world.

It is a basic REPL, with a plugin system where:

- plugins can be written in any language compiling to WebAssembly
- plugins are sandboxed by default
- the REPL logic is written in Rust, it also compiles to WebAssembly (you could swap it for your implementation in your own language)

There are two kinds of hosts:

- a CLI host `pluginlab`, written in Rust running in a terminal
- a web host, written in TypeScript running in a browser

Those hosts then run the same codebase which is compiled to WebAssembly:

- the REPL logic
- the plugins

<p align="center"><a href="https://topheman.github.io/webassembly-component-model-experiments/"><img src="./packages/web-host/public/wasi.png" alt="Demo" /></a></p>
<p align="center">
  Check the online demo at<br/><a href="https://topheman.github.io/webassembly-component-model-experiments/">topheman.github.io/webassembly-component-model-experiments</a>
</p>

## Previous work with WebAssembly

In the last seven years I've done a few projects involving rust and WebAssembly:

- [topheman/bevy-rust-wasm-experiments](https://github.com/topheman/bevy-rust-wasm-experiments): Proof of concept that aims to demonstrate how to code a video game in rust that compiles both to a binary file and a web site (via WebAssembly, binding the accelerometer and gyroscope sensors from the mobile device)
- [topheman/webassembly-wasi-experiments](https://github.com/topheman/webassembly-wasi-experiments): Discover WebAssembly System Interface (WASI) with C/Rust targetting NodeJS, python, Wasmtime and the browser
- [topheman/rust-wasm-experiments](https://github.com/topheman/rust-wasm-experiments): Discover how to use Rust to generate WebAssembly, called by JavaScript
  - [📺🇫🇷 Utiliser WebAssembly, dès aujourd'hui - ParisJS #86](https://www.youtube.com/watch?v=F3wOfWIFzVc&list=PLWhFHBFsRtquZ6hVXVjXmJ-l51ZXuSBtb)


## Usage

### pluginlab (rust)

#### Install

```bash
# Install the pluginlab binary
cargo install pluginlab
```

#### Run

```bash
pluginlab\
  --repl-logic https://topheman.github.io/webassembly-component-model-experiments/plugins/repl_logic_guest.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_greet.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_ls.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_echo.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_weather.wasm
```

Other flags:

- `--dir`: directory to be preopened (by default, the current directory)
- `--help`: displays manual
- `--debug`: run the host in debug mode (by default, the host runs in release mode)

<details>
<summary>🚀 Example of running the CLI host</summary>
<pre>
pluginlab\
  --repl-logic https://topheman.github.io/webassembly-component-model-experiments/plugins/repl_logic_guest.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_greet.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_ls.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_echo.wasm\
  --plugins https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_weather.wasm
[Host] Starting REPL host...
[Host] Loading REPL logic from: https://topheman.github.io/webassembly-component-model-experiments/plugins/repl_logic_guest.wasm
[Host] Loading plugin: https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_greet.wasm
[Host] Loading plugin: https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_ls.wasm
[Host] Loading plugin: https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_echo.wasm
[Host] Loading plugin: https://topheman.github.io/webassembly-component-model-experiments/plugins/plugin_weather.wasm
repl(0)> echo foo
foo
repl(0)> echo $ROOT/$USER
/Users/Tophe
repl(0)> export FOO=toto

repl(0)> echo $FOO
toto
repl(0)> greet $FOO
Hello, toto!
repl(0)> ls wit
wit/host-api.wit
wit/plugin-api.wit
wit/shared.wit
repl(0)> weather Paris
Sunny
repl(0)> weather New York
Partly cloudy
repl(0)> azertyuiop
Unknown command: azertyuiop. Try `help` to see available commands.
repl(1)> echo $?
1
repl(0)> greet $USER
Hello, Tophe!
repl(0)> echo $0
Hello, Tophe!
repl(0)>
</pre>
</details>

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

### pluginlab (rust)

#### Build

```bash
just build
```

This will (see [justfile](./justfile)):

- compile the pluginlab crate from rust to a binary file
- compile the repl-logic-guest crate from rust to wasm
- compile the plugin-* crates from rust to wasm

#### Run

```bash
./target/debug/pluginlab\
  --repl-logic ./target/wasm32-wasip1/debug/repl_logic_guest.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_greet.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_ls.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_echo.wasm\
  --plugins ./target/wasm32-wasip1/debug/plugin_weather.wasm
```

This will run the `pluginlab` binary which will itself:

- load and compile the `repl_logic_guest.wasm` file inside the embedded `wasmtime` engine injecting the [`host-api`](./wit/host-api.wit) interface
- load and compile the `plugin_*.wasm` files into the engine, injecting the [`plugin-api`](./wit/plugin-api.wit) interface
- launch the REPL loop executing the code from the `repl_logic_guest.wasm` file which will:
  - readline from the user
  - parse the command
  - dispatch the command to the plugin(s) if needed (run the `run`, `man` functions of the plugins via the [`host-api`](./wit/host-api.wit) interface)
  - display the result

Other example:

```bash
./target/debug/pluginlab\
  --repl-logic ./target/wasm32-wasip1/debug/repl_logic_guest.wasm\
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

#### Publish

```bash
# Dry run
just publish-pluginlab-dry-run
```

Once you're happy with the changes, you can publish the pluginlab crate:

```bash
just publish-pluginlab
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
- copy the wasm files in `target/wasm32-wasip1/release` to the `packages/web-host/public/plugins` directory (to make them available via http for the `pluginlab` binary)
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

### plugins (TypeScript)

You can write plugins in rust in [`crates/plugin-*`](./crates), you can also write plugins in TypeScript in [`packages/plugin-*`](./packages), thanks to `jco componentize` (based on [componentize-js](https://github.com/bytecodealliance/componentize-js)).

There is a [`packages/plugin-echo`](./packages/plugin-echo/) example plugin in TypeScript.

The downsides of writing plugins in TypeScript is mostly that your `.wasm` file will be **much larger** than the one compiled from rust:

- ~100KB of wasm for the rust plugin
- 11MB of wasm for the TypeScript plugin

The reason is that a JavaScript runtime needs to be embedded in the `.wasm` file, which is not the case for the rust plugin.

More about the [SpiderMonkey runtime embedding](https://github.com/bytecodealliance/ComponentizeJS?tab=readme-ov-file#explainer).

### plugins (Other languages)

Coming soon.

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
