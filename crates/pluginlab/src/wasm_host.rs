use crate::api::host_api::HostApi;
use crate::api::plugin_api::PluginApi;
use crate::cli::Cli;
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
    pub fn new(engine: &WasmEngine, wasi_ctx: WasiCtx, cli: &Cli) -> Self {
        Self {
            store: engine.create_store(wasi_ctx, &cli),
            plugins: HashMap::new(),
            repl_logic: None,
        }
    }

    pub async fn load_plugin(&mut self, engine: &WasmEngine, source: &str) -> Result<()> {
        let component = engine.load_component(source).await?;
        match engine.instantiate_plugin(&mut self.store, component).await {
            Ok(plugin) => {
                // Get the plugin name from the plugin itself
                let plugin_name = plugin.repl_api_plugin().call_name(&mut self.store).await?;
                self.plugins.insert(plugin_name, PluginInstance { plugin });
                return Ok(());
            }
            Err(e) => {
                if e.to_string()
                    .contains("failed to convert function to given type")
                {
                    let plugin_filename = source.split("/").last().unwrap();
                    let crate_version = env!("CARGO_PKG_VERSION");
                    eprintln!("[Host]");
                    eprintln!("[Host] Error: Failed instanciating {}", source);
                    eprintln!(
                        "[Host] You are most likely trying to use a plugin not compatible with pluginlab@{}",
                        crate_version
                    );
                    eprintln!("[Host]");
                    eprintln!("[Host] Try using a compatible version of the plugin by passing the following flag:");
                    eprintln!("[Host] --plugins https://github.com/topheman/webassembly-component-model-experiments/releases/download/pluginlab@{}/{}", crate_version, plugin_filename);
                    eprintln!("[Host]");
                    eprintln!("[Host] If it doesn't work, make sure to use the latest version of pluginlab: `cargo install pluginlab`");
                    eprintln!("[Host]");
                    eprintln!("[Host] Original error:");
                }
                return Err(e);
            }
        }
    }

    pub async fn load_repl_logic(&mut self, engine: &WasmEngine, source: &str) -> Result<()> {
        let component = engine.load_component(source).await?;
        let repl_logic = engine
            .instantiate_repl_logic(&mut self.store, component)
            .await?;
        self.repl_logic = Some(repl_logic);
        Ok(())
    }

    #[allow(unused)]
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
