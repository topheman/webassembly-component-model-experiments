#[allow(warnings)]
mod bindings;
mod parser;
mod env;

use crate::bindings::exports::repl::api::repl_logic::Guest as ReplLogicGuest;
use crate::bindings::exports::repl::api::plugin_runner::Guest as PluginRunnerGuest;
use crate::bindings::exports::repl::api::repl_logic;
use crate::bindings::repl::api::transport;

use crate::env::EnvVars;

struct Component {
    plugins: Vec<repl_logic::PluginConfig>,
    env_vars: EnvVars,
}

impl Component {
    fn new() -> Self {
        Self {
            plugins: Vec::new(),
            env_vars: EnvVars::new(),
        }
    }
}

impl ReplLogicGuest for Component {
    fn set_plugins(plugins: Vec<repl_logic::PluginConfig>) {
        println!("Setting plugins: {:?}", plugins);
        // TODO: Store plugins in the component
    }

    fn set_env(env_var: repl_logic::ReplEnvVar) {
        println!("Setting env var: {} = {}", env_var.key, env_var.value);
        // TODO: Get component instance and update env_vars
        // For now, we'll need to implement this differently since we don't have access to self
    }

    fn list_env() -> Vec<repl_logic::ReplEnvVar> {
        // TODO: Get component instance and return env_vars
        // For now, return empty vector
        Vec::new()
    }

    fn readline(line: String) -> transport::ReadlineResult {
        // TODO: Get component instance and use its env_vars
        // Temporary envVars for testing
        let mut env_vars = EnvVars::new();
        env_vars.set("HOME".to_string(), "/home/user".to_string());
        env_vars.set("USER".to_string(), "john".to_string());
        parser::parse_line(&line, &env_vars)
    }
}

impl PluginRunnerGuest for Component {
    fn exec(command: String, payload: String) -> transport::ReplResult {
        transport::ReplResult {
            color: None,
            status: transport::ReplStatus::Success,
            output: Some(format!("[PluginRunnerGuest.exec]Command: {}, Payload: {}", command, payload)),
        }
    }
}

bindings::export!(Component with_types_in bindings);
