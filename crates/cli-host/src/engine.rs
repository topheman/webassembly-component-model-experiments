use anyhow::Result;
use std::path::Path;
use wasmtime::{Engine, Config, Store};
use wasmtime::component::{Component, Linker as ComponentLinker};

// Import the generated bindings
use api::{PluginApi, HostApi};

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

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub async fn load_component(&self, path: &Path) -> Result<Component> {
        let component = Component::from_file(&self.engine, path)?;
        Ok(component)
    }

    /// Instantiate a plugin component with the plugin-api world
    pub async fn instantiate_plugin(&self, store: &mut Store<()>, component: Component) -> Result<PluginApi> {
        let linker = ComponentLinker::new(&self.engine);
        // Add the http-client interface (implement this trait in your host)
        // PluginApi::add_to_linker(&mut linker, |_| &mut ())?;
        // Instantiate the component and get the plugin interface
        let plugin = PluginApi::instantiate_async(store, &component, &linker).await?;
        Ok(plugin)
    }

    /// Instantiate the REPL logic component with the host-api world
    pub async fn instantiate_repl_logic(&self, store: &mut Store<()>, component: Component) -> Result<HostApi> {
        let linker = ComponentLinker::new(&self.engine);
        // HostApi::add_to_linker(&mut linker, |_| &mut ())?;
        let repl_logic = HostApi::instantiate_async(store, &component, &linker).await?;
        Ok(repl_logic)
    }
}
