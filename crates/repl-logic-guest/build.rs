use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Run cargo component build with RUSTFLAGS to disable fibers
    let status = Command::new("cargo")
        .arg("component")
        .arg("build")
        .arg("--release")
        .env("RUSTFLAGS", "--cfg=wasmtime_fiber_platform=disabled")
        .current_dir(&manifest_dir)
        .status()
        .expect("Failed to run cargo component build");

    if !status.success() {
        panic!("cargo component build failed");
    }

    // The component will be built in target/wasm32-unknown-unknown/release/repl-logic-guest.wasm
    println!("cargo:warning=Component built successfully");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
}
