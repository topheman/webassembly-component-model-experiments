#[allow(warnings)]
mod bindings;

use bindings::Guest;
use api::host_api::exports::repl::api::{repl_logic, repl};

struct Component {
    plugins: Vec<repl_logic::PluginConfig>,
    env_vars: Vec<repl_logic::ReplEnvVar>,
}

impl Guest for Component {
    fn new() -> Self {
        Self {
            plugins: Vec::new(),
            env_vars: Vec::new(),
        }
    }

    fn set_plugins(&mut self, plugins: Vec<repl_logic::PluginConfig>) {
        self.plugins = plugins;
        for plugin in &self.plugins {
            println!("Plugin: {} (args: {:?})", plugin.command, plugin.arg_count);
        }
    }

    fn set_env(&mut self, env_var: repl_logic::ReplEnvVar) {
        println!("Setting env var: {} = {}", env_var.key, env_var.value);
        self.env_vars.push(env_var);
    }

    fn list_env(&mut self) -> Vec<repl_logic::ReplEnvVar> {
        self.env_vars.clone()
    }

    fn readline(&mut self, line: String) -> repl::api::transport::ReplResult {
        repl::api::transport::ReplResult {
            color: None,
            status: repl::api::transport::ReplStatus::Success,
            output: Some(format!("Echo: {}", line)),
        }
    }

    fn exec(&mut self, command: String, payload: String) -> repl::api::transport::ReplResult {
        repl::api::transport::ReplResult {
            color: None,
            status: repl::api::transport::ReplStatus::Success,
            output: Some(format!("Command: {}, Payload: {}", command, payload)),
        }
    }
}

bindings::export!(Component with_types_in bindings);
