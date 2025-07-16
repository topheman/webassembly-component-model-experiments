use crate::bindings::repl::api::{host_state, transport};

struct ReservedCommand {
    name: &'static str,
    man: fn() -> transport::PluginResponse,
    run: Option<fn(payload: &str) -> transport::PluginResponse>,
}

/// A list of the commands that are reserved (not overridable by plugins)
const RESERVED_COMMANDS: &[ReservedCommand] = &[
    ReservedCommand {
        name: "export",
        man: || transport::PluginResponse {
            status: transport::ReplStatus::Success,
            stdout: Some(format!(
                r#"
NAME
    export - Export a variable to the environment

SYNOPSIS
    export <key>=<value>

DESCRIPTION
    Export a variable to the environment.

EXAMPLES
> export USER=Tophe
    "#
            )),
            stderr: None,
        },
        run: Some(|payload| {
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
        }),
    },
    ReservedCommand {
        name: "help",
        man: || {
            let plugins = host_state::get_plugins_names();
            let reserved_commands = get_reserved_commands();
            transport::PluginResponse {
                status: transport::ReplStatus::Success,
                stdout: Some(format!(
                    r#"
NAME
    help - Show the manual for a command

SYNOPSIS
    help <command>

DESCRIPTION
    Show the manual for a command.

    Plugins: {}
    Reserved commands: {}
      "#,
                    plugins.join(", "),
                    reserved_commands.join(", "),
                )),
                stderr: None,
            }
        },
        run: None,
    },
    ReservedCommand {
        name: "man",
        man: || transport::PluginResponse {
            status: transport::ReplStatus::Success,
            stdout: Some(format!(
                r#"
NAME
    man - Show the manual for a command

SYNOPSIS
    man <command>

DESCRIPTION
    Show the manual for a command.
      "#,
            )),
            stderr: None,
        },
        run: None,
    },
    ReservedCommand {
        name: "list-commands",
        man: || transport::PluginResponse {
            status: transport::ReplStatus::Success,
            stdout: Some(format!(
                r#"
NAME
    list-commands - List the plugins loaded in the host and the reserved commands (not overridable by plugins) included in the REPL logic.

SYNOPSIS
    list-commands

DESCRIPTION
    List the plugins loaded in the host and the reserved commands (not overridable by plugins) included in the REPL logic.

EXAMPLES
    "#
            )),
            stderr: None,
        },
        run: Some(|_payload| {
            let plugins = host_state::get_plugins_names();
            let reserved_commands = get_reserved_commands();
            transport::PluginResponse {
                status: transport::ReplStatus::Success,
                stdout: Some(format!(
                    "Plugins: {} - Reserved commands: {}",
                    plugins.join(", "),
                    reserved_commands.join(", ")
                )),
                stderr: None,
            }
        }),
    },
];

pub fn run(command: &str, payload: &str) -> Option<transport::PluginResponse> {
    for reserved_command in RESERVED_COMMANDS {
        if reserved_command.name == command {
            // try to exec the `run` function first, then the `man` function
            // if no `run` function is provided (like for help)
            if let Some(run) = reserved_command.run {
                let result = run(payload);
                return Some(result);
            } else {
                return Some((reserved_command.man)());
            }
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
            return Some((reserved_command.man)());
        }
    }
    None
}

pub fn get_reserved_commands() -> Vec<String> {
    RESERVED_COMMANDS
        .iter()
        .map(|c| c.name.to_string())
        .collect()
}
