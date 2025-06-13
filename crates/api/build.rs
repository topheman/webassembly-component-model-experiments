use std::path::PathBuf;
use wasmtime;

fn main() {
    // let wit_dir = PathBuf::from("../../wit");
    let gen_dir = PathBuf::from("src/generated");
    std::fs::create_dir_all(&gen_dir).unwrap();

    wasmtime::component::bindgen!({
        async: true,
        path: "../../wit/plugin.wit"
    });
}
