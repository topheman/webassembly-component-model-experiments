#[allow(warnings)]
mod bindings;

use crate::bindings::exports::repl::api::plugin::Guest;
use crate::bindings::repl::api::transport;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "weather".to_string()
    }

    fn man() -> String {
        r#"
NAME
    weather - Get the weather for a given city (built with Rust🦀)

USAGE
    weather <city>

DESCRIPTION
    Get the weather for a given city.

        "#
        .to_string()
    }

    fn run(payload: String) -> Result<transport::PluginResponse, ()> {
        Ok(transport::PluginResponse {
            status: transport::ReplStatus::Success,
            stdout: Some(format!("Weather for {}", payload)),
            stderr: None,
        })
    }
}

bindings::export!(Component with_types_in bindings);
