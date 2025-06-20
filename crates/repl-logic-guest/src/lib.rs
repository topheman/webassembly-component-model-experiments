#[allow(warnings)]
mod bindings;
mod parser;
mod vars;

use crate::bindings::exports::repl::api::repl_logic::Guest as ReplLogicGuest;
use crate::bindings::repl::api::host_state;
use crate::bindings::repl::api::transport;

struct Component {}

impl ReplLogicGuest for Component {
    fn readline(line: String) -> transport::ReadlineResult {
        let vars = host_state::get_repl_vars();
        match parser::parse_line(&line, &vars.into()) {
            parser::ParseResult::Plugin(result) => result,
            parser::ParseResult::Export((key, value)) => {
                host_state::set_repl_var(&transport::ReplVar {
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

bindings::export!(Component with_types_in bindings);
