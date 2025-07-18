use std::env;
use std::fs;
use std::path::Path;
use wasmtime;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=./wit");

    // Generate host-api bindings
    let bindings = wasmtime::component::bindgen!({
        path: "./wit",
        world: "host-api",
        async: true,
        stringify: true,
    });
    fs::write(Path::new(&out_dir).join("host_api.rs"), bindings).unwrap();

    // Generate plugin-api bindings
    let bindings = wasmtime::component::bindgen!({
        path: "./wit",
        world: "plugin-api",
        async: true,
        stringify: true,
    });
    fs::write(Path::new(&out_dir).join("plugin_api.rs"), bindings).unwrap();
}
