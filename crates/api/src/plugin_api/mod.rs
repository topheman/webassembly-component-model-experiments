use wasmtime;

pub mod wit {
  wasmtime::component::bindgen!({
    path: "./wit",
    world: "plugin-api",
    async: true,
  });
}
