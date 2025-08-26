#!/bin/bash

# Prepare wasm files for the web host
# Usage: ./scripts/prepare-wasm-files.sh --mode debug|release [--target-dir <dir>]

set -e

usage() {
    echo "Usage: $0 --mode <mode> [--target-dir <dir>]"
    echo "  mode: debug or release"
    echo "  target-dir: optional custom target directory (default: packages/web-host/public/plugins)"
    echo ""
    echo "Example:"
    echo "  $0 --mode debug"
    echo "  $0 --mode release"
    echo "  $0 --mode debug --target-dir ./custom/plugins"
    exit 1
}

validate_mode() {
    local mode="$1"
    if [[ "$mode" != "debug" && "$mode" != "release" ]]; then
        echo "Error: --mode must be one of: debug, release."
        exit 1
    fi
}

prepare_wasm_files() {
    local mode="$1"
    local target_dir="$2"
    echo "Preparing wasm files for mode: $mode"

    local workspace_root="$(cd "$(dirname "$0")/.." && pwd)"

    mkdir -p "$target_dir"

    local wasm_files=(
        "target/wasm32-wasip1/$mode/plugin_echo.wasm"
        "target/wasm32-wasip1/$mode/plugin_greet.wasm"
        "target/wasm32-wasip1/$mode/plugin_ls.wasm"
        "target/wasm32-wasip1/$mode/plugin_cat.wasm"
        "target/wasm32-wasip1/$mode/plugin_weather.wasm"
        "target/wasm32-wasip1/$mode/plugin_tee.wasm"
        "c_modules/plugin-echo/plugin-echo-c.wasm"
        "go_modules/plugin-echo/plugin-echo-go.wasm"
        "target/wasm32-wasip1/$mode/repl_logic_guest.wasm"
    )

    # Copy each wasm file
    for wasm_file in "${wasm_files[@]}"; do
        local copy_from="$workspace_root/$wasm_file"
        local copy_to="$target_dir/$(basename "$wasm_file")"

        if [[ ! -f "$copy_from" ]]; then
            echo ""
            echo "Failed to copy $copy_from to $copy_to"
            echo ""
            if [[ "$mode" == "debug" ]]; then
                echo "Please run the command: just build"
            else
                echo "Please run the command: just build-release"
            fi
            echo ""
            exit 1
        fi

        cp "$copy_from" "$copy_to"
        echo "Copied $copy_from to $copy_to"
    done
}

# Parse command line arguments
MODE=""
TARGET_DIR=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --mode)
            MODE="$2"
            shift 2
            ;;
        --target-dir)
            TARGET_DIR="$2"
            shift 2
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo "Unknown option: $1"
            usage
            ;;
    esac
done

# Check if mode is provided
if [[ -z "$MODE" ]]; then
    echo "Error: --mode is required"
    usage
fi

# Set default target directory if not provided
if [[ -z "$TARGET_DIR" ]]; then
    WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
    TARGET_DIR="$WORKSPACE_ROOT/packages/web-host/public/plugins"
fi

# Validate mode and prepare files
validate_mode "$MODE"
prepare_wasm_files "$MODE" "$TARGET_DIR"
