#[allow(warnings)]
mod bindings;

use crate::bindings::exports::repl::api::plugin::Guest;
use crate::bindings::repl::api::transport;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "greet".to_string()
    }

    fn man() -> String {
        r#"
NAME
    greet - Greet the user

SYNOPSIS
    greet <name>

DESCRIPTION
    Greet the user with the given name.

EXAMPLES
> greet Tophe
Hello, Tophe!

        "#
        .to_string()
    }

    fn arg_count() -> Option<i8> {
        Some(127) // todo use enums for arg count
    }

    fn run(payload: String) -> Result<transport::PluginResponse, ()> {
        Ok(transport::PluginResponse {
            status: transport::ReplStatus::Success,
            stdout: Some(format!("Hello, {}!", payload)),
            stderr: None,
        })
    }
}

bindings::export!(Component with_types_in bindings);
