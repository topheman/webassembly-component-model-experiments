#[allow(warnings)]
mod bindings;

use std::io::Write;

use crate::bindings::exports::repl::api::plugin::Guest;
use crate::bindings::repl::api::host_state_plugin;
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
        match run_inner(payload) {
            Ok(content) => Ok(transport::PluginResponse {
                status: transport::ReplStatus::Success,
                stdout: Some(format!("{}", content)),
                stderr: None,
            }),
            Err(e) => {
                // e.kind() - verify if the error is a permission error
                return Ok(transport::PluginResponse {
                    status: transport::ReplStatus::Error,
                    stdout: None,
                    stderr: Some(format!("{}", e)),
                });
            }
        }
    }
}

fn run_inner(payload: String) -> Result<String, String> {
    let is_append = payload.starts_with("-a") || payload.starts_with("--append");
    let filepath = if is_append {
        let Some((_, filepath)) = payload.split_once(" ") else {
            return Err("Invalid arguments. Usage: tee <file> or tee -a <file>".to_string());
        };
        filepath.to_string()
    } else {
        payload
    };

    let content = host_state_plugin::get_repl_var("0").unwrap_or("".to_string());
    let content_as_bytes = content.as_bytes();

    if !is_append {
        let mut file = std::fs::File::create(&filepath)
            .map_err(|e| format!("Failed to create file '{}': {}", filepath, e))?;
        file.write_all(content_as_bytes)
            .map_err(|e| format!("Failed to write to file '{}': {}", filepath, e))?;
        return Ok(content);
    } else {
        let mut file = std::fs::File::options()
            .append(true)
            .open(&filepath)
            .map_err(|e| format!("Failed to open file in append mode '{}': {}", filepath, e))?;
        // Add a newline before the content in append mode
        file.write_all(b"\n")
            .map_err(|e| format!("Failed to write newline to file '{}': {}", filepath, e))?;
        file.write_all(content_as_bytes)
            .map_err(|e| format!("Failed to write to file '{}': {}", filepath, e))?;
        return Ok(content);
    }
}

bindings::export!(Component with_types_in bindings);
