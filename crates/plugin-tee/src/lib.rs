#[allow(warnings)]
mod bindings;

use crate::bindings::exports::repl::api::plugin::Guest;
use crate::bindings::repl::api::transport;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "tee".to_string()
    }

    fn man() -> String {
        r#"
NAME
    tee - Copy $0 content to a file (built with Rust🦀)

USAGE
    tee <file>
    tee -a <file>

OPTIONS
    -a, --append  Append to the file instead of overwriting it

DESCRIPTION
    Copy $0 content to a file.

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
