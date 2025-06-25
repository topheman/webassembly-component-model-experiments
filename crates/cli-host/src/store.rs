use std::collections::HashMap;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView};

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

    /// Bellow is the state maintained by the host itself and shared with the guest
    /// implementing Host traits on the host side (here, the cli)
    /// and shared with the guest via the Guest bindings

    /// Custom environment variables stored by the REPL
    pub repl_vars: HashMap<String, String>,

    /// Names of the plugins loaded in the host
    pub plugins_names: Vec<String>,
}

/// --- Host implementation for plugin API ---
pub struct PluginHost {}

impl api::plugin_api::repl::api::http_client::Host for PluginHost {
    async fn get(
        &mut self,
        _url: String,
        _headers: Vec<api::plugin_api::repl::api::http_client::HttpHeader>,
    ) -> Result<api::plugin_api::repl::api::http_client::HttpResponse, String> {
        // TODO: Implement HTTP client functionality
        Ok(api::plugin_api::repl::api::http_client::HttpResponse {
            status: 200,
            headers: vec![],
            body: "HTTP client not implemented yet".to_string(),
        })
    }

    async fn post(
        &mut self,
        _url: String,
        _headers: Vec<api::plugin_api::repl::api::http_client::HttpHeader>,
        _body: String,
    ) -> Result<api::plugin_api::repl::api::http_client::HttpResponse, String> {
        // TODO: Implement HTTP client functionality
        Ok(api::plugin_api::repl::api::http_client::HttpResponse {
            status: 501,
            headers: vec![],
            body: "HTTP client not implemented yet".to_string(),
        })
    }
}

/// It is necessary to implement this trait on PluginHost because other parts rely on it.
impl api::plugin_api::repl::api::transport::Host for PluginHost {
    // This trait has no methods, so no implementation needed
}

impl api::host_api::repl::api::transport::Host for WasiState {
    // This trait has no methods, so no implementation needed
}

/// --- Implementations of traits for WasiState ---

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

impl api::host_api::repl::api::host_state::Host for WasiState {
    async fn get_plugins_names(&mut self) -> wasmtime::component::__internal::Vec<String> {
        self.plugins_names.clone()
    }

    async fn set_repl_vars(
        &mut self,
        vars: wasmtime::component::__internal::Vec<api::host_api::repl::api::transport::ReplVar>,
    ) {
        // Store environment variables in the WasiState
        for var in vars {
            self.repl_vars.insert(var.key.clone(), var.value.clone());
            println!("Setting repl var: {} = {}", var.key, var.value);
        }
    }

    async fn get_repl_vars(
        &mut self,
    ) -> wasmtime::component::__internal::Vec<api::host_api::repl::api::transport::ReplVar> {
        // Return the stored environment variables
        self.repl_vars
            .iter()
            .map(
                |(key, value)| api::host_api::repl::api::transport::ReplVar {
                    key: key.clone(),
                    value: value.clone(),
                },
            )
            .collect()
    }

    async fn set_repl_var(&mut self, var: api::host_api::repl::api::transport::ReplVar) {
        // Set a single environment variable
        self.repl_vars.insert(var.key.clone(), var.value.clone());
    }
}
