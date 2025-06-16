use wasmtime;

pub mod wit {
  wasmtime::component::bindgen!({
    path: "./wit",
    world: "plugin-api",
    async: true,
  });
}

include!(concat!(env!("OUT_DIR"), "/plugin_api.rs"));
