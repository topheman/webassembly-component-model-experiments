#[allow(warnings)]
mod bindings;

use crate::bindings::exports::repl::api::plugin::Guest;
use crate::bindings::repl::api::transport;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "echo".to_string()
    }

    fn man() -> String {
        r#"
NAME
    echo - Echo a message (built with Rust🦀)

USAGE
    echo <message>

DESCRIPTION
    Echo a message.

        "#
        .to_string()
    }

    fn run(payload: String) -> Result<transport::PluginResponse, ()> {
        Ok(transport::PluginResponse {
            status: transport::ReplStatus::Success,
            stdout: Some(format!("{}", payload)),
            stderr: None,
        })
    }
}

bindings::export!(Component with_types_in bindings);
