use wasmtime;

pub mod wit {
  wasmtime::component::bindgen!({
    path: "./wit",
    world: "host-api",
    async: true,
  });
}

include!(concat!(env!("OUT_DIR"), "/host_api.rs"));
