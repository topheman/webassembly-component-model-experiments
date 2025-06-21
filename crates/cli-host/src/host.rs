use crate::engine::WasmEngine;
use crate::store::WasiState;
use anyhow::Result;
use api::host_api::HostApi;
use api::plugin_api::PluginApi;
use std::collections::HashMap;
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
    pub plugins: HashMap<String, PluginInstance>,
}

impl Host {
    pub fn new(engine: &WasmEngine) -> Self {
        Self {
            store: engine.create_store(),
            plugins: HashMap::new(),
            repl_logic: None,
        }
    }

    pub async fn load_plugin(&mut self, engine: &WasmEngine, path: PathBuf) -> Result<()> {
        let component = engine.load_component(&path).await?;
        let plugin = engine
            .instantiate_plugin(&mut self.store, component)
            .await?;

        // Get the plugin name from the plugin itself
        let plugin_name = plugin.repl_api_plugin().call_name(&mut self.store).await?;

        self.plugins.insert(plugin_name, PluginInstance { plugin });

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
