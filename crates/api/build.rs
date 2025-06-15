use wasmtime;
use std::fs;
use std::path::Path;

fn main() {
    let code = wasmtime::component::bindgen!({
        path: "./wit/world.wit",
        world: "api",
        async: true,
        stringify: true
    });
    let bindings_path = Path::new("src").join("bindings.rs");
    fs::write(&bindings_path, code).unwrap();
}
