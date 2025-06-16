use anyhow::Result;
use std::path::Path;
use wasmtime::{Engine, Config, Store};
use wasmtime::component::{Component, Linker as ComponentLinker};

/// A generic WebAssembly engine wrapper that handles component loading and instantiation
pub struct WasmEngine {
    engine: Engine,
}

impl WasmEngine {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = Engine::new(&config)?;
        Ok(Self { engine })
    }

    pub async fn load_component(&self, path: &Path) -> Result<Component> {
        let component = Component::from_file(&self.engine, path)?;
        Ok(component)
    }

    pub async fn instantiate_component(&self, component: Component) -> Result<Store<()>> {
        let mut store = Store::new(&self.engine, ());
        let linker = ComponentLinker::new(&self.engine);

        // Instantiate the component
        let _instance = linker.instantiate(&mut store, &component)?;

        Ok(store)
    }
}
