#[allow(warnings)]
mod bindings;

use crate::bindings::exports::repl::api::repl_logic::Guest as ReplLogicGuest;
use crate::bindings::exports::repl::api::plugin_runner::Guest as PluginRunnerGuest;
use crate::bindings::exports::repl::api::repl_logic;
use crate::bindings::repl::api::transport;

struct Component {
    plugins: Vec<repl_logic::PluginConfig>,
    env_vars: Vec<repl_logic::ReplEnvVar>,
}

impl ReplLogicGuest for Component {
    fn set_plugins(plugins: Vec<repl_logic::PluginConfig>) {
        println!("Setting plugins: {:?}", plugins);
    }

    fn set_env(env_var: repl_logic::ReplEnvVar) {
        println!("Setting env var: {} = {}", env_var.key, env_var.value);
    }

    fn list_env() -> Vec<repl_logic::ReplEnvVar> {
        Vec::new()
    }

    fn readline(line: String) -> transport::ReplResult {
        transport::ReplResult {
            color: None,
            status: transport::ReplStatus::Success,
            output: Some(format!("Echo: {}", line)),
        }
    }
}

impl PluginRunnerGuest for Component {
    fn exec(command: String, payload: String) -> transport::ReplResult {
        transport::ReplResult {
            color: None,
            status: transport::ReplStatus::Success,
            output: Some(format!("Command: {}, Payload: {}", command, payload)),
        }
    }
}

bindings::export!(Component with_types_in bindings);
