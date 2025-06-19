use anyhow::Result;
use std::path::PathBuf;
use wasmtime::Store;
use api::host_api::HostApi;
use api::plugin_api::PluginApi;
use crate::engine::{WasmEngine, WasiState};

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

    /// Set a custom environment variable in the store
    pub fn set_store_env_var(&mut self, key: String, value: String) {
        self.store.data_mut().repl_env_vars.insert(key, value);
    }

    /// Get a custom environment variable from the store
    pub fn get_store_env_var(&self, key: &str) -> Option<&String> {
        self.store.data().repl_env_vars.get(key)
    }

    /// Get all custom environment variables from the store
    pub fn get_all_store_env_vars(&self) -> &std::collections::HashMap<String, String> {
        &self.store.data().repl_env_vars
    }

    /// Add a plugin configuration to the store
    pub fn add_plugin_config(&mut self, config: api::host_api::repl::api::transport::PluginConfig) {
        self.store.data_mut().plugin_configs.push(config);
    }

    /// Get all plugin configurations from the store
    pub fn get_plugin_configs(&self) -> &Vec<api::host_api::repl::api::transport::PluginConfig> {
        &self.store.data().plugin_configs
    }

    /// Access the WASI context from the store
    pub fn get_wasi_ctx(&mut self) -> &mut wasmtime_wasi::p2::WasiCtx {
        &mut self.store.data_mut().ctx
    }

    /// Access the resource table from the store
    pub fn get_resource_table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.store.data_mut().table
    }
}
