use api::host_api::exports::repl::api::{repl_logic};
pub struct ReplLogic;

impl ReplLogic {
    pub fn new() -> Self {
        Self
    }

    pub fn set_plugins(&mut self, plugins: Vec<repl_logic::PluginConfig>) {
        for plugin in plugins {
            println!("Plugin: {} (args: {:?})", plugin.command, plugin.arg_count);
        }
    }

    pub fn set_env(&mut self, env_var: repl_logic::ReplEnvVar) {
        println!("Setting env var: {} = {}", env_var.key, env_var.value);
    }

    pub fn list_env(&mut self) -> Vec<repl_logic::ReplEnvVar> {
        Vec::new()
    }

    pub fn readline(&mut self, line: String) -> repl::api::transport::ReplResult {
        repl::api::transport::ReplResult {
            color: None,
            status: repl::api::transport::ReplStatus::Success,
            output: Some(format!("Echo: {}", line)),
        }
    }

    pub fn exec(&mut self, command: String, payload: String) -> repl::api::transport::ReplResult {
        repl::api::transport::ReplResult {
            color: None,
            status: repl::api::transport::ReplStatus::Success,
            output: Some(format!("Command: {}, Payload: {}", command, payload)),
        }
    }
}

