use crate::bindings::repl::api::transport;
use crate::env::EnvVars;

pub fn parse_line(line: &str, env_vars: &EnvVars) -> transport::ReadlineResult {
    // Split the line into command and arguments
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.is_empty() {
        return transport::ReadlineResult {
            command: String::new(),
            payload: String::new(),
        };
    }

    let command = parts[0].to_string();
    let payload = if parts.len() > 1 {
        // Expand variables in the payload
        let raw_payload = parts[1..].join(" ");
        env_vars.expand_variables(&raw_payload)
    } else {
        String::new()
    };

    transport::ReadlineResult {
        command,
        payload,
    }
}

mod tests {
  use super::*;

  fn make_env_vars() -> EnvVars {
    let mut env_vars = EnvVars::new();
    env_vars.set("HOME".to_string(), "/home/user".to_string());
    env_vars.set("USER".to_string(), "john".to_string());
    env_vars
  }

  #[test]
  fn basic_parse() {
    let env_vars = make_env_vars();
    let result = parse_line("echo Hello, world!", &env_vars);
    assert_eq!(result.command, "echo");
    assert_eq!(result.payload, "Hello, world!");
  }

  #[test]
  fn parse_with_args() {
    let env_vars = make_env_vars();
    let result = parse_line("echo Hello, world! -n", &env_vars);
    assert_eq!(result.command, "echo");
    assert_eq!(result.payload, "Hello, world! -n");
  }

  #[test]
  fn parse_with_variable_to_expand() {
    let env_vars = make_env_vars();
    let result = parse_line("echo $HOME", &env_vars);
    assert_eq!(result.command, "echo");
    assert_eq!(result.payload, "/home/user");
  }

  #[test]
  fn parse_with_multiple_variables() {
    let env_vars = make_env_vars();
    let result = parse_line("echo $HOME/$USER", &env_vars);
    assert_eq!(result.command, "echo");
    assert_eq!(result.payload, "/home/user/john");
  }

  #[test]
  fn parse_with_unknown_variable() {
    let env_vars = make_env_vars();
    let result = parse_line("echo $UNKNOWN", &env_vars);
    assert_eq!(result.command, "echo");
    assert_eq!(result.payload, "");
  }

  #[test]
  fn parse_empty_line() {
    let env_vars = make_env_vars();
    let result = parse_line("", &env_vars);
    assert_eq!(result.command, "");
    assert_eq!(result.payload, "");
  }

  #[test]
  fn parse_command_only() {
    let env_vars = make_env_vars();
    let result = parse_line("ls", &env_vars);
    assert_eq!(result.command, "ls");
    assert_eq!(result.payload, "");
  }
}
