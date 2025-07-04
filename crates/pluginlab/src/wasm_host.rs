use crate::api::host_api::HostApi;
use crate::api::plugin_api::PluginApi;
use crate::engine::WasmEngine;
use crate::store::WasiState;
use anyhow::Result;
use std::collections::HashMap;
use wasmtime::Store;
use wasmtime_wasi::p2::WasiCtx;

/// Represents a loaded plugin
pub struct PluginInstance {
    pub plugin: PluginApi,
}

/// The main host that manages plugins and the REPL logic
pub struct WasmHost {
    pub store: Store<WasiState>,
    pub repl_logic: Option<HostApi>,
    pub plugins: HashMap<String, PluginInstance>,
}

impl WasmHost {
    pub fn new(engine: &WasmEngine, wasi_ctx: WasiCtx) -> Self {
        Self {
            store: engine.create_store(wasi_ctx),
            plugins: HashMap::new(),
            repl_logic: None,
        }
    }

    pub async fn load_plugin(&mut self, engine: &WasmEngine, source: &str) -> Result<()> {
        let component = engine.load_component(source).await?;
        let plugin = engine
            .instantiate_plugin(&mut self.store, component)
            .await?;

        // Get the plugin name from the plugin itself
        let plugin_name = plugin.repl_api_plugin().call_name(&mut self.store).await?;

        self.plugins.insert(plugin_name, PluginInstance { plugin });

        Ok(())
    }

    pub async fn load_repl_logic(&mut self, engine: &WasmEngine, source: &str) -> Result<()> {
        let component = engine.load_component(source).await?;
        let repl_logic = engine
            .instantiate_repl_logic(&mut self.store, component)
            .await?;
        self.repl_logic = Some(repl_logic);
        Ok(())
    }

    pub async fn load_repl_logic_from_bytes(
        &mut self,
        engine: &WasmEngine,
        bytes: &[u8],
    ) -> Result<()> {
        let component = engine.load_component_from_bytes(bytes)?;
        let repl_logic = engine
            .instantiate_repl_logic(&mut self.store, component)
            .await?;
        self.repl_logic = Some(repl_logic);
        Ok(())
    }
}
