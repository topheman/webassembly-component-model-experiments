#[allow(warnings)]
mod bindings;
mod parser;
mod reserved;
mod vars;

use crate::bindings::exports::repl::api::guest_state::Guest as GuestStateGuest;
use crate::bindings::exports::repl::api::repl_logic::Guest as ReplLogicGuest;
use crate::bindings::repl::api::host_state;
use crate::bindings::repl::api::transport;

struct Component {}

impl ReplLogicGuest for Component {
    fn readline(line: String) -> transport::ReadlineResponse {
        let vars = host_state::get_repl_vars();

        // parse the line into a command and payload + expand variables
        let parsed_line = parser::parse_line(&line, &vars.into());

        // try to run reserved commands or show their manual
        // must be done before running plugins, because plugins must not override reserved commands
        if let Some(response) = reserved::run(&parsed_line.command, &parsed_line.payload) {
            return transport::ReadlineResponse::Ready(response);
        }

        if parsed_line.command == "man"
            && (parsed_line.payload.is_empty() || parsed_line.payload == "man")
        {
            if let Some(response) = reserved::man(&parsed_line.command) {
                return transport::ReadlineResponse::Ready(response);
            }
        }

        // if no reserved command was run return the parsed line to be passed to the plugin to run from the host
        transport::ReadlineResponse::ToRun(parsed_line)
    }
}

impl GuestStateGuest for Component {
    fn get_reserved_commands() -> Vec<String> {
        reserved::get_reserved_commands()
    }
}

bindings::export!(Component with_types_in bindings);
