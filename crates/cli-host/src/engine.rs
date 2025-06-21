use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use wasmtime::{Engine, Config, Store};
use wasmtime::component::{Component, Linker as ComponentLinker, ResourceTable};
use wasmtime_wasi::p2::{WasiCtxBuilder};

// Import the generated bindings
use api::plugin_api::PluginApi;
use api::host_api::HostApi;
use crate::store::{WasiState, PluginHost};

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

    /// Create a new store with WASI context
    pub fn create_store(&self) -> Store<WasiState> {
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()
            .inherit_env()
            .build();
        Store::new(&self.engine, WasiState {
            ctx: wasi_ctx,
            table: ResourceTable::new(),
            plugin_host: PluginHost {},
            repl_vars: HashMap::new(),
        })
    }

    /// Instantiate a plugin component with the plugin-api world
    pub async fn instantiate_plugin(&self, store: &mut Store<WasiState>, component: Component) -> Result<PluginApi> {
        let mut linker: ComponentLinker<WasiState> = ComponentLinker::new(&self.engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

        // Add the plugin API interface with host implementation
        PluginApi::add_to_linker(&mut linker, |state: &mut WasiState| &mut state.plugin_host)?;

        // Instantiate the component and get the plugin interface
        let plugin = PluginApi::instantiate_async(store, &component, &linker).await?;
        Ok(plugin)
    }

    /// Instantiate the REPL logic component with the host-api world
    pub async fn instantiate_repl_logic(&self, store: &mut Store<WasiState>, component: Component) -> Result<HostApi> {
        let mut linker: ComponentLinker<WasiState> = ComponentLinker::new(&self.engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

        // Add the host API interface with host implementation
        HostApi::add_to_linker(&mut linker, |state: &mut WasiState| state)?;

        let repl_logic = HostApi::instantiate_async(store, &component, &linker).await?;
        Ok(repl_logic)
    }
}
