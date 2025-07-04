# Build all crates with appropriate commands
build:
    just build-repl-logic-guest
    just build-pluginlab
    just build-plugins

# Build all crates in release mode
build-release:
    just build-repl-logic-guest-release
    just build-pluginlab-release
    just build-plugins-release

# Build all plugins in debug mode
build-plugins:
    #!/usr/bin/env bash
    just --list|grep build-plugin-|awk '{print $1}'|grep -v release|xargs just

# Build all plugins in release mode
build-plugins-release:
    #!/usr/bin/env bash
    just --list|grep build-plugin-|awk '{print $1}'|grep release|xargs just

# Build the plugin-weather component
build-plugin-weather:
    cargo component build -p plugin-weather

# Build the plugin-weather component in release mode
build-plugin-weather-release:
    cargo component build --release -p plugin-weather

# Build the pluginlab (normal Rust build)
build-pluginlab:
    cargo build -p pluginlab

# Build the pluginlab in release mode
build-pluginlab-release:
    cargo build --release -p pluginlab

# Publish the pluginlab crate
publish-pluginlab:
    cargo publish -p pluginlab

# Publish the pluginlab crate (dry run)
publish-pluginlab-dry-run:
    cargo publish --dry-run -p pluginlab

# Build the plugin-greet component
build-plugin-echo:
    cargo component build -p plugin-echo

# Build the plugin-greet component in release mode
build-plugin-echo-release:
    cargo component build --release -p plugin-echo

# Build the plugin-greet component
build-plugin-greet:
    cargo component build -p plugin-greet

# Build the plugin-greet component in release mode
build-plugin-greet-release:
    cargo component build --release -p plugin-greet

# Build the plugin-ls component
build-plugin-ls:
    cargo component build -p plugin-ls

# Build the plugin-ls component in release mode
build-plugin-ls-release:
    cargo component build --release -p plugin-ls

# Build the REPL logic guest as a component
build-repl-logic-guest:
    cargo component build -p repl-logic-guest

# Build the REPL logic guest as a component in release mode
build-repl-logic-guest-release:
    cargo component build --release -p repl-logic-guest

# Clean all build artifacts
clean:
    cargo clean
    cargo component clean

# Show help
default:
    @just --list

test:
    cargo test

# Run the e2e tests for the pluginlab
test-e2e-pluginlab:
    cargo test -p pluginlab

test-e2e-pluginlab-nocapture:
    cargo test -p pluginlab -- --nocapture
