use anyhow::Result;
use std::path::Path;
use wasmtime::{Engine, Config, Store, Linker};
use wasmtime::component::{Component, Linker as ComponentLinker};

mod engine;
mod host;

pub use engine::WasmEngine;
pub use host::{Host, Plugin};

pub struct WasmHost {
    engine: Engine,
}

impl WasmHost {
    pub fn new() -> Result<Self> {
        // Configure the engine with component model support
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(true);

        // Create the engine with our configuration
        let engine = Engine::new(&config)?;

        Ok(Self { engine })
    }

    pub async fn load_component(&self, path: &Path) -> Result<Component> {
        // Load the component from the file
        let component = Component::from_file(&self.engine, path)?;
        Ok(component)
    }

    pub async fn instantiate_component(&self, component: Component) -> Result<Store<()>> {
        // Create a new store for this instance
        let mut store = Store::new(&self.engine, ());

        // Create a linker for the component
        let linker = ComponentLinker::new(&self.engine);

        // TODO: Add host functions to the linker

        // Instantiate the component
        let _instance = linker.instantiate(&mut store, &component)?;

        Ok(store)
    }
}
