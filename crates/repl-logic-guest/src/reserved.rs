use crate::bindings::repl::api::{host_state, transport};

struct ReservedCommand {
    name: &'static str,
    man: &'static str,
    run: fn(payload: &str) -> transport::PluginResponse,
}

/// A list of the commands that are reserved (not overridable by plugins)
const RESERVED_COMMANDS: &[ReservedCommand] = &[
    ReservedCommand {
        name: "export",
        man: r#"
    export <key>=<value>

    Export a variable to the environment.
    "#,
        run: |payload| {
            let (key, value) = payload.split_once('=').unwrap();
            host_state::set_repl_var(&transport::ReplVar {
                key: key.to_string(),
                value: value.to_string(),
            });
            transport::PluginResponse {
                status: transport::ReplStatus::Success,
                stdout: Some(format!("")),
                stderr: None,
            }
        },
    },
    ReservedCommand {
        name: "help",
        man: r#"
        help <command>

        Show the manual for a command.
        "#,
        run: |_payload| man("help").unwrap(),
    },
];

pub fn run(command: &str, payload: &str) -> Option<transport::PluginResponse> {
    for reserved_command in RESERVED_COMMANDS {
        if reserved_command.name == command {
            let result = (reserved_command.run)(payload);
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
