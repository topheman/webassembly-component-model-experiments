#[allow(warnings)]
mod bindings;
mod parser;
mod vars;

use crate::bindings::exports::repl::api::repl_logic::Guest as ReplLogicGuest;
use crate::bindings::repl::api::host_state;
use crate::bindings::repl::api::plugin_runner;
use crate::bindings::repl::api::transport;

struct Component {}

impl ReplLogicGuest for Component {
    fn readline(line: String) -> transport::PluginResponse {
        let vars = host_state::get_repl_vars();
        match parser::parse_line(&line, &vars.into()) {
            parser::ParseResult::Plugin(result) => {
                match plugin_runner::run(&result.command, &result.payload) {
                    Ok(response) => transport::PluginResponse {
                        status: response.status,
                        stdout: response.stdout,
                        stderr: response.stderr,
                    },
                    Err(e) => transport::PluginResponse {
                        status: transport::ReplStatus::Error,
                        stdout: None,
                        stderr: Some(format!("Error running plugin \"{}\"", &result.command)),
                    },
                }
            }
            parser::ParseResult::Export((key, value)) => {
                host_state::set_repl_var(&transport::ReplVar {
                    key: key.clone(),
                    value: value.clone(),
                });
                transport::PluginResponse {
                    status: transport::ReplStatus::Success,
                    stdout: Some(format!("{}={}", key.clone(), value.clone())),
                    stderr: None,
                }
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
