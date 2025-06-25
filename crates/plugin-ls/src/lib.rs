#[allow(warnings)]
mod bindings;

use crate::bindings::exports::repl::api::plugin::Guest;
use crate::bindings::repl::api::transport;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "ls".to_string()
    }

    fn man() -> String {
        r#"
NAME
    ls - List the files in the directory passed as argument (built with Rust🦀)

SYNOPSIS
    ls

DESCRIPTION
    List the files in the directory passed as argument.

EXAMPLES
> ls .
README.md
Cargo.toml

        "#
        .to_string()
    }

    fn run(payload: String) -> Result<transport::PluginResponse, ()> {
        match std::fs::read_dir(&payload) {
            Ok(files) => {
                let mut files = files
                    .map(|file| file.unwrap().path().to_str().unwrap().to_string())
                    .collect::<Vec<_>>();
                files.sort();
                let files = files.join("\n");
                return Ok(transport::PluginResponse {
                    status: transport::ReplStatus::Success,
                    stdout: Some(files),
                    stderr: None,
                });
            }
            Err(err) => {
                return Ok(transport::PluginResponse {
                    status: transport::ReplStatus::Error,
                    stdout: None,
                    stderr: Some(format!(
                        "Could not list files in directory: {}\nError: {}",
                        payload,
                        err.to_string()
                    )),
                });
            }
        };
    }
}

bindings::export!(Component with_types_in bindings);
