#[allow(warnings)]
mod bindings;

use crate::bindings::exports::repl::api::plugin::Guest;
use crate::bindings::repl::api::transport;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "cat".to_string()
    }

    fn man() -> String {
        r#"
NAME
    cat - Print the contents of the file passed as argument (built with Rust🦀)

SYNOPSIS
    cat

DESCRIPTION
    Print the contents of the file passed as argument.

EXAMPLES
> cat README.md
# This is a README file
It contains some information about the project
It is written in Markdown
It is used to describe the project

        "#
        .to_string()
    }

    fn run(payload: String) -> Result<transport::PluginResponse, ()> {
        match std::fs::metadata(&payload) {
            Ok(metadata) => {
                if metadata.is_file() {
                    let file = std::fs::read_to_string(&payload).unwrap();
                    return Ok(transport::PluginResponse {
                        status: transport::ReplStatus::Success,
                        stdout: Some(file),
                        stderr: None,
                    });
                } else if metadata.is_dir() {
                    return Ok(transport::PluginResponse {
                        status: transport::ReplStatus::Error,
                        stdout: None,
                        stderr: Some(format!("cat: {}: Is a directory", payload)),
                    });
                } else {
                    return Ok(transport::PluginResponse {
                        status: transport::ReplStatus::Error,
                        stdout: None,
                        stderr: Some(format!("cat: {}: Unsupported file type", payload)),
                    });
                }
            }
            Err(err) => {
                return Ok(transport::PluginResponse {
                    status: transport::ReplStatus::Error,
                    stdout: None,
                    stderr: Some(format!("cat: {}: {}", payload, err.to_string())),
                });
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
