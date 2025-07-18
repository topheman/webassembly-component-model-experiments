set dotenv-load

# Show help
default:
    @just --list

# Initialize the .env file tracking the WASI SDK version
init-env-file:
    cp .env.original .env
    cat .env
    @echo ""
    @echo "Currently, in .env file, WASI_OS=$WASI_OS and WASI_ARCH=$WASI_ARCH, please update them if needed."

#Download the WASI SDK into ./c_deps/wasi-sdk folder - run `just init-env-file` before
dl-wasi-sdk:
    #!/usr/bin/env bash
    mkdir -p c_deps
    FILENAME=wasi-sdk-${WASI_VERSION_FULL}-${WASI_ARCH}-${WASI_OS}.tar.gz
    curl -L -o c_deps/${FILENAME} https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/${FILENAME}
    tar -C c_deps -xvf c_deps/${FILENAME}
    mv c_deps/wasi-sdk-${WASI_VERSION_FULL}-${WASI_ARCH}-${WASI_OS} c_deps/wasi-sdk

wasi-sdk-name:
    @echo wasi-sdk-${WASI_VERSION_FULL}-${WASI_ARCH}-${WASI_OS}.tar.gz

# Build all crates with appropriate commands
build: build-repl-logic-guest build-plugins
    just build-pluginlab

# Build all crates in release mode
build-release: build-repl-logic-guest-release build-plugins-release
    just build-pluginlab-release

# Build all plugins in debug mode
build-plugins:
    #!/usr/bin/env bash
    just list-rust-plugins|xargs -I {} cargo component build -p {}

# Build all plugins in release mode
build-plugins-release:
    #!/usr/bin/env bash
    just list-rust-plugins|xargs -I {} cargo component build --release -p {}

# Build a specific plugin
build-plugin plugin:
    cargo component build -p {{plugin}}

# Build a specific plugin in release mode
build-plugin-release plugin:
    cargo component build --release -p {{plugin}}

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

# List all the rust plugins
list-rust-plugins:
    #!/usr/bin/env bash
    ls -1 crates|grep plugin-

# Run the tests for the pluginlab
test: build-repl-logic-guest build-plugins prepare-fixtures
    cargo test

# Run the e2e tests for the pluginlab
test-e2e-pluginlab: build-repl-logic-guest build-plugins prepare-fixtures
    cargo test -p pluginlab

# Run the e2e tests for the pluginlab with no capture
test-e2e-pluginlab-nocapture: build-repl-logic-guest build-plugins prepare-fixtures
    cargo test -p pluginlab -- --nocapture

# Run the e2e tests for the pluginlab retrieving the plugins from the HTTP server
test-e2e-pluginlab-http: build-repl-logic-guest build-plugins prepare-fixtures
    WASM_TARGET_DIR=https://topheman.github.io/webassembly-component-model-experiments/plugins cargo test -p pluginlab

# Run the e2e tests for the pluginlab retrieving the plugins from the HTTP server
test-e2e-pluginlab-http-nocapture: build-repl-logic-guest build-plugins prepare-fixtures
    WASM_TARGET_DIR=https://topheman.github.io/webassembly-component-model-experiments/plugins cargo test -p pluginlab -- --nocapture

# Prepare the fixtures for the e2e tests
prepare-fixtures:
    mkdir -p tmp/filesystem
    rm -rf tmp/filesystem/*
    cp -r fixtures/filesystem tmp
    mv tmp/filesystem/README.rust.md tmp/filesystem/README.md
    rm tmp/filesystem/README.browser.md
