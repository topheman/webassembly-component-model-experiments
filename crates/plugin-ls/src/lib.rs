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
        match std::fs::metadata(&payload) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    match std::fs::read_dir(&payload) {
                        Ok(files) => {
                            let mut files = files
                                .map(|file| {
                                    let file = file.unwrap();
                                    let file_type = file.file_type().unwrap();
                                    let file_type = if file_type.is_dir() {
                                        "D"
                                    } else if file_type.is_file() {
                                        "F"
                                    } else {
                                        "L"
                                    };
                                    let file_path = file.path().to_str().unwrap().to_string();
                                    format!("{}\t{}", file_type, file_path)
                                })
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
                } else if metadata.is_file() {
                    return Ok(transport::PluginResponse {
                        status: transport::ReplStatus::Success,
                        stdout: Some(format!("F\t{}", &payload)),
                        stderr: None,
                    });
                } else {
                    return Ok(transport::PluginResponse {
                        status: transport::ReplStatus::Error,
                        stdout: None,
                        stderr: Some(format!("ls: {}: Unsupported file type", &payload)),
                    });
                }
            }
            Err(err) => {
                // `std::fs::metadata` fails on symlinks, so we need to check if the file is a symlink in the error case
                if let Ok(metadata) = std::fs::symlink_metadata(&payload) {
                    if metadata.is_symlink() {
                        let target = std::fs::read_link(&payload).unwrap();
                        return Ok(transport::PluginResponse {
                            status: transport::ReplStatus::Success,
                            stdout: Some(format!(
                                "L\t{} -> {}",
                                &payload,
                                target.to_str().unwrap()
                            )),
                            stderr: None,
                        });
                    }
                }
                return Ok(transport::PluginResponse {
                    status: transport::ReplStatus::Error,
                    stdout: None,
                    stderr: Some(format!("ls: {}: {}", payload, err.to_string())),
                });
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
