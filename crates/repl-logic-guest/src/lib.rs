#[allow(warnings)]
mod bindings;
mod parser;
mod env;

use std::sync::{LazyLock, RwLock};

use crate::bindings::exports::repl::api::repl_logic::Guest as ReplLogicGuest;
use crate::bindings::exports::repl::api::plugin_runner::Guest as PluginRunnerGuest;
use crate::bindings::exports::repl::api::repl_logic;
use crate::bindings::repl::api::transport;

use crate::env::EnvVars;

static mut STORED_PLUGINS: Vec<repl_logic::PluginConfig> = Vec::new();
static STORED_ENV_VARS: LazyLock<RwLock<EnvVars>> = LazyLock::new(|| RwLock::new({
    let mut env_vars = EnvVars::new();
    env_vars.set("HOME".to_string(), "/home/user".to_string());
    env_vars.set("USER".to_string(), "john".to_string());
    env_vars
}));

struct EncapsulatedImplementation {}

impl EncapsulatedImplementation {

    fn set_env_var(env_var: repl_logic::ReplEnvVar) -> () {
        let mut env_vars = STORED_ENV_VARS.write().unwrap();
        // env_vars.set(env_var.key, env_var.value);
    }

}

struct Component {}

impl ReplLogicGuest for Component {
    fn readline(line: String) -> transport::ReadlineResult {
        // TODO: Get component instance and use its env_vars
        // Temporary envVars for testing
        // let mut env_vars = EnvVars::new();
        // env_vars.set("HOME".to_string(), "/home/user".to_string());
        // env_vars.set("USER".to_string(), "john".to_string());
        let mut env_vars = STORED_ENV_VARS.write().unwrap();
        match parser::parse_line(&line, &mut env_vars) {
            parser::ParseResult::Plugin(result) => result,
            parser::ParseResult::Export((key, value)) => {
                EncapsulatedImplementation::set_env_var(repl_logic::ReplEnvVar { key: key.clone(), value: value.clone() });
                transport::ReadlineResult {
                    command: "export".to_string(),
                    payload: format!("{}={}", key.clone(), value.clone())
                }
            }
        }
    }

    fn set_plugins(plugins: Vec<repl_logic::PluginConfig>) -> () {
        todo!()
    }

    fn get_plugins() -> Vec<repl_logic::PluginConfig> {
        todo!()
    }

    fn set_env_vars(env_vars: Vec<repl_logic::ReplEnvVar>) -> () {
        todo!()
    }

    fn get_env_vars() -> Vec<repl_logic::ReplEnvVar> {
        todo!()
    }

    fn set_env_var(env_var: repl_logic::ReplEnvVar) -> () {
        EncapsulatedImplementation::set_env_var(env_var);
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
