#[allow(warnings)]
mod bindings;
mod parser;
mod reserved;
mod vars;

use crate::bindings::exports::repl::api::repl_logic::Guest as ReplLogicGuest;
use crate::bindings::repl::api::host_state;
use crate::bindings::repl::api::transport;

struct Component {}

impl ReplLogicGuest for Component {
    fn readline(line: String) -> transport::ReadlineResponse {
        let vars = host_state::get_repl_vars();
        let result = parser::parse_line(&line, &vars.into());

        // builtins::run(&result.command, &result.payload);

        // builtin export
        if result.command == "export" {
            let (key, value) = result.payload.split_once('=').unwrap();
            host_state::set_repl_var(&transport::ReplVar {
                key: key.to_string(),
                value: value.to_string(),
            });
        }

        // keep track of the last command in $0
        host_state::set_repl_var(&transport::ReplVar {
            key: "0".to_string(),
            value: line,
        });

        result
    }
}

bindings::export!(Component with_types_in bindings);
