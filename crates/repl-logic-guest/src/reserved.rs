use crate::bindings::repl::api::{host_state, transport};

struct ReservedCommand {
    name: &'static str,
    man: &'static str,
    run: fn(command: &str, payload: &str) -> transport::PluginResponse,
}

/// A list of the commands that are reserved (not overridable by plugins)
const RESERVED_COMMANDS: &[ReservedCommand; 1] = &[ReservedCommand {
    name: "export",
    man: r#"
    export <key>=<value>

    Export a variable to the environment.
    "#,
    run: |command, payload| {
        let (key, value) = payload.split_once('=').unwrap();
        host_state::set_repl_var(&transport::ReplVar {
            key: key.to_string(),
            value: value.to_string(),
        });
        transport::PluginResponse {
            status: transport::ReplStatus::Success,
            stdout: Some(format!("{}={}", key, value)),
            stderr: None,
        }
    },
}];

pub fn run(command: &str, payload: &str) -> Option<transport::PluginResponse> {
    for reserved_command in RESERVED_COMMANDS {
        if reserved_command.name == command {
            let result = (reserved_command.run)(command, payload);
            return Some(result);
        }
        if command == "man" {
            return man(payload);
        }
    }
    None
}

pub fn man(command: &str) -> Option<transport::PluginResponse> {
    for reserved_command in RESERVED_COMMANDS {
        if reserved_command.name == command {
            return Some(transport::PluginResponse {
                status: transport::ReplStatus::Success,
                stdout: Some(reserved_command.man.to_string()),
                stderr: None,
            });
        }
    }
    None
}
