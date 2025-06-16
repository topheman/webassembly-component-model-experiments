use wasmtime::component::__internal::anyhow;
use api::host_api::exports::repl::api::{repl_logic, plugin_runner};

wasmtime::component::bindgen!({
    path: "../api/wit",
    world: "host-api",
    async: true,
    with: {
        "repl:api/repl-logic": ReplLogic,
        "repl:api/plugin-runner": ReplLogic,
    },
});

pub struct ReplLogic {}

impl ReplLogic {
    pub fn new() -> Self {
        Self {}
    }
}

#[export_name = "set-plugins"]
pub extern "C" fn set_plugins(plugins: Vec<repl_logic::PluginConfig>) {
    for plugin in plugins {
        println!("Plugin: {} (args: {:?})", plugin.command, plugin.arg_count);
    }
}

#[export_name = "set-env"]
pub extern "C" fn set_env(env_var: repl_logic::ReplEnvVar) {
    println!("Setting env var: {} = {}", env_var.key, env_var.value);
}

#[export_name = "list-env"]
pub extern "C" fn list_env() -> Vec<repl_logic::ReplEnvVar> {
    Vec::new()
}

#[export_name = "readline"]
pub extern "C" fn readline(line: String) -> repl::api::transport::ReplResult {
    repl::api::transport::ReplResult {
        color: None,
        status: repl::api::transport::ReplStatus::Success,
        output: Some(format!("Echo: {}", line)),
    }
}

#[export_name = "exec"]
pub extern "C" fn exec(command: String, payload: String) -> repl::api::transport::ReplResult {
    repl::api::transport::ReplResult {
        color: None,
        status: repl::api::transport::ReplStatus::Success,
        output: Some(format!("Command: {}, Payload: {}", command, payload)),
    }
}

