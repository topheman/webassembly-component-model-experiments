use crate::engine::WasmEngine;
use crate::store::WasiState;
use anyhow::Result;
use api::host_api::HostApi;
use api::plugin_api::PluginApi;
use std::path::PathBuf;
use wasmtime::Store;

/// Represents a loaded plugin
pub struct PluginInstance {
    pub plugin: PluginApi,
}

/// The main host that manages plugins and the REPL logic
pub struct Host {
    pub store: Store<WasiState>,
    pub repl_logic: Option<HostApi>,
    pub plugins: Vec<PluginInstance>,
}

impl Host {
    pub fn new(engine: &WasmEngine) -> Self {
        Self {
            store: engine.create_store(),
            plugins: Vec::new(),
            repl_logic: None,
        }
    }

    pub async fn load_plugin(&mut self, engine: &WasmEngine, path: PathBuf) -> Result<()> {
        let component = engine.load_component(&path).await?;
        let plugin = engine
            .instantiate_plugin(&mut self.store, component)
            .await?;
        self.plugins.push(PluginInstance { plugin });
        Ok(())
    }

    pub async fn load_repl_logic(&mut self, engine: &WasmEngine, path: PathBuf) -> Result<()> {
        let component = engine.load_component(&path).await?;
        let repl_logic = engine
            .instantiate_repl_logic(&mut self.store, component)
            .await?;
        self.repl_logic = Some(repl_logic);
        Ok(())
    }
}
