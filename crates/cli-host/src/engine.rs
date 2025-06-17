use anyhow::Result;
use std::path::Path;
use wasmtime::{Engine, Config, Store};
use wasmtime::component::{Component, Linker as ComponentLinker, ResourceTable};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView};

// Import the generated bindings
use api::plugin_api::PluginApi;
use api::host_api::HostApi;

/// Host implementation for plugin API
pub struct PluginHost {
    // Add any plugin-specific state here
}

impl api::plugin_api::repl::api::http_client::Host for PluginHost {
    async fn get(&mut self, _url: String, _headers: Vec<api::plugin_api::repl::api::http_client::HttpHeader>) -> api::plugin_api::repl::api::http_client::HttpResponse {
        // TODO: Implement HTTP client functionality
        api::plugin_api::repl::api::http_client::HttpResponse {
            status: 501,
            headers: vec![],
            body: "HTTP client not implemented yet".to_string(),
        }
    }

    async fn post(&mut self, _url: String, _headers: Vec<api::plugin_api::repl::api::http_client::HttpHeader>, _body: String) -> api::plugin_api::repl::api::http_client::HttpResponse {
        // TODO: Implement HTTP client functionality
        api::plugin_api::repl::api::http_client::HttpResponse {
            status: 501,
            headers: vec![],
            body: "HTTP client not implemented yet".to_string(),
        }
    }
}

impl api::plugin_api::repl::api::transport::Host for PluginHost {
    // This trait has no methods, so no implementation needed
}

/// Host implementation for host API
pub struct HostApiHost {
    // Add any host API-specific state here
}

impl api::host_api::repl::api::transport::Host for HostApiHost {
    // This trait has no methods, so no implementation needed
}

/// State that implements both WasiView and IoView for WASI support
///
/// This struct combines all the necessary state for running WebAssembly components
/// with WASI (WebAssembly System Interface) support. It serves as the single
/// state object for the Wasmtime store, providing access to:
/// - WASI system context (files, network, environment, etc.)
/// - Resource management table (handles for files, sockets, etc.)
/// - Host API implementations for plugin and REPL logic components
pub struct WasiState {
    /// WASI system context containing file descriptors, environment variables,
    /// command line arguments, and other system-level state
    pub ctx: WasiCtx,

    /// Resource table that manages handles to host resources (files, sockets, etc.)
    /// This allows the WebAssembly guest to reference host resources through
    /// opaque handles while the host maintains the actual resource objects
    pub table: ResourceTable,

    /// Host implementation for plugin API interfaces (HTTP client, transport, etc.)
    /// This provides the functionality that plugins can call from WebAssembly
    pub plugin_host: PluginHost,

    /// Host implementation for REPL logic API interfaces
    /// This provides the functionality that the REPL logic component can call
    pub host_api_host: HostApiHost,
}

/// Implementation of IoView trait for resource management
///
/// This trait provides access to the ResourceTable, which is responsible for:
/// - Creating and tracking resource handles (files, sockets, etc.)
/// - Managing resource lifecycle (creation, sharing, cleanup)
/// - Allowing WebAssembly components to reference host resources safely
impl IoView for WasiState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

/// Implementation of WasiView trait for system-level WASI operations
///
/// This trait provides access to the WasiCtx, which contains:
/// - File system state (file descriptors, pre-opened directories)
/// - Process state (environment variables, command line arguments)
/// - Network capabilities and socket state
/// - Time and random number generation state
/// - Terminal I/O state (stdin, stdout, stderr)
impl WasiView for WasiState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

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
            host_api_host: HostApiHost {},
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
        HostApi::add_to_linker(&mut linker, |state: &mut WasiState| &mut state.host_api_host)?;

        let repl_logic = HostApi::instantiate_async(store, &component, &linker).await?;
        Ok(repl_logic)
    }
}
