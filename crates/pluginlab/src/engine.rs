use crate::cli::Cli;
use crate::permissions::NetworkPermissions;
use anyhow::Result;

use std::path::Path;
use wasmtime::component::{Component, Linker as ComponentLinker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::p2::{WasiCtx, WasiCtxBuilder};

// Import the generated bindings
use crate::api::host_api::HostApi;
use crate::api::plugin_api::PluginApi;
use crate::store::{PluginHost, WasiState};

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

    #[allow(unused)]
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Load a WebAssembly component from either a local file or HTTP URL
    pub async fn load_component(&self, source: &str) -> Result<Component> {
        // Check if the source is a URL (starts with http:// or https://)
        if source.starts_with("http://") || source.starts_with("https://") {
            self.load_component_from_url(source).await
        } else {
            // Treat as local file path
            let path = Path::new(source);
            self.load_component_from_file(path).await
        }
    }

    /// Load a WebAssembly component from a local file
    pub async fn load_component_from_file(&self, path: &Path) -> Result<Component> {
        let component = Component::from_file(&self.engine, path)?;
        Ok(component)
    }

    /// Load a WebAssembly component from an HTTP URL
    pub async fn load_component_from_url(&self, url: &str) -> Result<Component> {
        // Create an HTTP client
        let client = reqwest::Client::new();

        // Download the component bytes
        let response = client.get(url).send().await?;

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to download component from {}: HTTP {}",
                url,
                response.status()
            ));
        }

        // Get the component bytes
        let bytes = response.bytes().await?;

        // Create the component from bytes
        let component = Component::from_binary(&self.engine, &bytes)?;
        Ok(component)
    }

    /// Load a WebAssembly component from bytes
    #[allow(unused)]
    pub fn load_component_from_bytes(&self, bytes: &[u8]) -> Result<Component> {
        let component = Component::from_binary(&self.engine, bytes)?;
        Ok(component)
    }

    pub fn build_wasi_ctx(cli: &Cli) -> Result<WasiCtx> {
        let host_path = cli.dir.clone();
        let guest_path = ".";

        let (dir_perms, file_perms) = if cli.allow_all || cli.allow_read && cli.allow_write {
            (
                wasmtime_wasi::DirPerms::all(),
                wasmtime_wasi::FilePerms::all(),
            )
        } else if cli.allow_read {
            (
                wasmtime_wasi::DirPerms::READ,
                wasmtime_wasi::FilePerms::READ,
            )
        } else if cli.allow_write {
            (
                wasmtime_wasi::DirPerms::MUTATE,
                wasmtime_wasi::FilePerms::WRITE,
            )
        } else {
            (
                wasmtime_wasi::DirPerms::empty(),
                wasmtime_wasi::FilePerms::empty(),
            )
        };

        let mut wasi_builder = WasiCtxBuilder::new();
        // .inherit_stdio()
        // .inherit_args()
        // .inherit_env()
        wasi_builder.preopened_dir(host_path, guest_path, dir_perms, file_perms)?;

        let wasi_ctx = wasi_builder.build();
        Ok(wasi_ctx)
    }

    /// Create a new store with WASI context
    pub fn create_store(&self, wasi_ctx: WasiCtx, cli: &Cli) -> Store<WasiState> {
        let repl_vars =
            std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));

        Store::new(
            &self.engine,
            WasiState {
                ctx: wasi_ctx,
                table: ResourceTable::new(),
                plugin_host: PluginHost {
                    network_permissions: NetworkPermissions::from(cli),
                    repl_vars: repl_vars.clone(),
                },
                repl_vars,
                plugins_names: Vec::new(),
            },
        )
    }

    /// Instantiate a plugin component with the plugin-api world
    pub async fn instantiate_plugin(
        &self,
        store: &mut Store<WasiState>,
        component: Component,
    ) -> Result<PluginApi> {
        let mut linker: ComponentLinker<WasiState> = ComponentLinker::new(&self.engine);

        // The plugins may access to the file system (and other WASI interfaces),
        // wasmtime_wasi uses tokio under the hood. If we use `add_to_linker_sync`, we get the error:
        // > thread 'main' panicked at /Users/tophe/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasmtime-wasi-33.0.0/src/runtime.rs:108:15:
        // > Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks.
        //
        // So we use `add_to_linker_async`, which will not create a private runtime and use the one we already are in.
        //
        // #nested-async-runtime https://github.com/bytecodealliance/wasmtime/issues/9515#issuecomment-2442376571
        wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

        // Add the plugin API interface with host implementation
        PluginApi::add_to_linker(&mut linker, |state: &mut WasiState| &mut state.plugin_host)?;

        // Instantiate the component and get the plugin interface
        let plugin = PluginApi::instantiate_async(store, &component, &linker).await?;
        Ok(plugin)
    }

    /// Instantiate the REPL logic component with the host-api world
    pub async fn instantiate_repl_logic(
        &self,
        store: &mut Store<WasiState>,
        component: Component,
    ) -> Result<HostApi> {
        let mut linker: ComponentLinker<WasiState> = ComponentLinker::new(&self.engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

        // Add the host API interface with host implementation
        HostApi::add_to_linker(&mut linker, |state: &mut WasiState| state)?;

        let repl_logic = HostApi::instantiate_async(store, &component, &linker).await?;
        Ok(repl_logic)
    }
}
