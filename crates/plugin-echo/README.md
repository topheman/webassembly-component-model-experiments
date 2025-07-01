# plugin-echo

Basic plugin for this REPL. Behaves like the `echo` command.

## Notes

This crate was initialized with `cargo component new`.

The building process is handled by the [`justfile`](../../justfile) in the root of the project.

The `cargo component build` command is used to build the plugin.

- It generates the `src/bindings.rs` file, based on the `package.metadata.component` section in the `Cargo.toml` file that describes where to find the component definition (wit files).
- It then compiles the plugin to WebAssembly.
