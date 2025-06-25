# Build all crates with appropriate commands
build:
    just build-api
    just build-cli-host
    just build-repl-logic-guest
    just build-plugin-greet
    just build-plugin-ls
    just build-plugin-echo
    just build-plugin-weather

# Build all crates in release mode
build-release:
    just build-api-release
    just build-cli-host-release
    just build-repl-logic-guest-release
    just build-plugin-greet-release
    just build-plugin-ls-release
    just build-plugin-echo-release
    just build-plugin-weather-release

# Build the plugin-weather component
build-plugin-weather:
    cargo component build -p plugin-weather

# Build the plugin-weather component in release mode
build-plugin-weather-release:
    cargo component build --release -p plugin-weather

# Build the API crate (normal Rust build)
build-api:
    cargo build -p api

# Build the API crate in release mode
build-api-release:
    cargo build --release -p api

# Build the CLI host (normal Rust build)
build-cli-host:
    cargo build -p cli-host

# Build the CLI host in release mode
build-cli-host-release:
    cargo build --release -p cli-host

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

# Build just the Rust crates (no components)
build-rust:
    just build-api
    just build-cli-host
    just build-plugin-greet

# Build just the Rust crates in release mode (no components)
build-rust-release:
    just build-api-release
    just build-cli-host-release
    just build-plugin-greet-release

# Build just the component
build-component:
    just build-repl-logic-guest

# Build just the component in release mode
build-component-release:
    just build-repl-logic-guest-release

# Clean all build artifacts
clean:
    cargo clean
    cargo component clean

# Show help
default:
    @just --list

test:
    cargo test

# Run the e2e tests for the CLI host
test-e2e-cli-host:
    cargo test -p cli-host

test-e2e-cli-host-nocapture:
    cargo test -p cli-host -- --nocapture
