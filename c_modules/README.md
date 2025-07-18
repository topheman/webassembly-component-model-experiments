# Plugins written in C

You can find here plugins written in C, some are re-implementations of the plugins written in Rust.

The `plugin_api.c`, `plugin_api.h` and `plugin_api.component_type.o` are generated with [`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen), based on the [wit files](../crates/pluginlab/wit) of the project.

The wasm files are compiled with the [wasi-sdk](https://github.com/WebAssembly/wasi-sdk) you downloaded with `just dl-wasi-sdk`, which contain the `clang` compiler.

All you have to do is run `just build` to build everything (including the C plugins) and `just test` to run the tests (including the C plugins).
