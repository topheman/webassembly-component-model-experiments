use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashMap;
use wasmtime::Store;
use crate::engine::WasmEngine;

/// Represents a loaded plugin
pub struct Plugin {
    name: String,
    store: Store<()>,
}

/// The main host that manages plugins and the REPL logic
pub struct Host {
    engine: WasmEngine,
    plugins: HashMap<String, Plugin>,
    repl_logic: Option<Store<()>>,
}

impl Host {
    pub fn new() -> Result<Self> {
        Ok(Self {
            engine: WasmEngine::new()?,
            plugins: HashMap::new(),
            repl_logic: None,
        })
    }

    pub async fn load_plugin(&mut self, path: PathBuf) -> Result<()> {
        let name = path.file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let component = self.engine.load_component(&path).await?;
        let store = self.engine.instantiate_component(component).await?;

        self.plugins.insert(name.clone(), Plugin {
            name,
            store,
        });

        Ok(())
    }

    pub async fn load_repl_logic(&mut self, path: PathBuf) -> Result<()> {
        let component = self.engine.load_component(&path).await?;
        let store = self.engine.instantiate_component(component).await?;
        self.repl_logic = Some(store);
        Ok(())
    }

    pub fn plugin_names(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }
}
