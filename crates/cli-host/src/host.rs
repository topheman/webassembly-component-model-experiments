use anyhow::Result;
use std::path::PathBuf;
use wasmtime::Store;
use api::{PluginApi, HostApi};
use crate::engine::WasmEngine;

/// Represents a loaded plugin
pub struct PluginInstance {
    pub plugin: PluginApi,
}

/// The main host that manages plugins and the REPL logic
pub struct Host {
    pub store: Store<()>,
    pub repl_logic: Option<HostApi>,
    pub plugins: Vec<PluginInstance>,
}

impl Host {
    pub fn new(engine: &WasmEngine) -> Self {
        Self {
            store: Store::new(engine.engine(), ()),
            plugins: Vec::new(),
            repl_logic: None,
        }
    }

    pub async fn load_plugin(&mut self, engine: &WasmEngine, path: PathBuf) -> Result<()> {
        let component = engine.load_component(&path).await?;
        let plugin = engine.instantiate_plugin(&mut self.store, component).await?;
        self.plugins.push(PluginInstance { plugin });
        Ok(())
    }

    pub async fn load_repl_logic(&mut self, engine: &WasmEngine, path: PathBuf) -> Result<()> {
        let component = engine.load_component(&path).await?;
        let repl_logic = engine.instantiate_repl_logic(&mut self.store, component).await?;
        self.repl_logic = Some(repl_logic);
        Ok(())
    }

    pub async fn plugin_names(&mut self) -> Vec<String> {
        let mut names = Vec::new();
        for plugin_instance in &self.plugins {
            match plugin_instance.plugin.repl_api_plugin().call_name(&mut self.store).await {
                Ok(name) => names.push(name.to_string()),
                Err(_) => names.push("<error>".to_string()),
            }
        }
        names
    }
}
