#[allow(warnings)]
mod bindings;
mod parser;
mod env;

use crate::bindings::exports::repl::api::repl_logic::Guest as ReplLogicGuest;
use crate::bindings::exports::repl::api::plugin_runner::Guest as PluginRunnerGuest;
use crate::bindings::repl::api::host_state;
use crate::bindings::repl::api::transport;

struct Component {}

impl ReplLogicGuest for Component {
    fn readline(line: String) -> transport::ReadlineResult {
        let env_vars = host_state::get_env_vars();
        match parser::parse_line(&line, &env_vars.into()) {
            parser::ParseResult::Plugin(result) => result,
            parser::ParseResult::Export((key, value)) => {
                host_state::set_env_var(&transport::ReplEnvVar {
                    key: key.clone(),
                    value: value.clone()
                });
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
