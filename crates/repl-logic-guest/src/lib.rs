#[allow(warnings)]
mod bindings;
mod parser;
mod env;

use crate::bindings::exports::repl::api::repl_logic::Guest as ReplLogicGuest;
use crate::bindings::exports::repl::api::plugin_runner::Guest as PluginRunnerGuest;
use crate::bindings::exports::repl::api::repl_logic;
use crate::bindings::repl::api::transport;

use crate::env::EnvVars;

struct Component {}

impl ReplLogicGuest for Component {
    fn set_plugins(_plugins: Vec<repl_logic::PluginConfig>) {}

    fn set_env(_env_var: repl_logic::ReplEnvVar) {}

    fn list_env() -> Vec<repl_logic::ReplEnvVar> {
        let mut env_vars = EnvVars::new();
        env_vars.set("HOME".to_string(), "/home/user".to_string());
        env_vars.set("USER".to_string(), "john".to_string());
        env_vars.into()
    }

    fn readline(line: String) -> transport::ReadlineResult {
        // TODO: Get component instance and use its env_vars
        // Temporary envVars for testing
        let mut env_vars = EnvVars::new();
        env_vars.set("HOME".to_string(), "/home/user".to_string());
        env_vars.set("USER".to_string(), "john".to_string());
        match parser::parse_line(&line, &env_vars) {
            parser::ParseResult::Plugin(result) => result,
            parser::ParseResult::Export((key, value)) => {
                Component::set_env(repl_logic::ReplEnvVar { key: key.clone(), value: value.clone() });
                transport::ReadlineResult {
                    command: "export".to_string(),
                    payload: format!("{}={}", key.clone(), value.clone())
                }
            }
        }
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
