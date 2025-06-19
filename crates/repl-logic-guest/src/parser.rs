use crate::bindings::repl::api::transport;
use crate::env::EnvVars;

pub enum ParseResult {
    Plugin(transport::ReadlineResult),
    Export((String, String)),
}

pub fn parse_line(line: &str, env_vars: &EnvVars) -> ParseResult {
    // Split the line into command and arguments
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.is_empty() {
        return ParseResult::Plugin(transport::ReadlineResult {
            command: String::new(),
            payload: String::new(),
        });
    }

    let command = parts[0].to_string();
    let payload = if parts.len() > 1 {
        // Expand variables in the payload
        let raw_payload = parts[1..].join(" ");
        env_vars.expand_variables(&raw_payload)
    } else {
        String::new()
    };

    if command == "export" {
        let (key, value) = payload.split_once('=').unwrap();
        return ParseResult::Export((key.to_string(), value.to_string()));
    }

    ParseResult::Plugin(transport::ReadlineResult { command, payload })
}

#[cfg(test)]
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
        let ParseResult::Plugin(result) = parse_line("echo Hello, world!", &env_vars) else {
            panic!("Expected a plugin result");
        };
        assert_eq!(result.command, "echo".to_string());
        assert_eq!(result.payload, "Hello, world!");
    }

    #[test]
    fn parse_with_args() {
        let env_vars = make_env_vars();
        let ParseResult::Plugin(result) = parse_line("echo Hello, world! -n", &env_vars) else {
            panic!("Expected a plugin result");
        };
        assert_eq!(result.command, "echo");
        assert_eq!(result.payload, "Hello, world! -n");
    }

    #[test]
    fn parse_with_variable_to_expand() {
        let env_vars = make_env_vars();
        let ParseResult::Plugin(result) = parse_line("echo $HOME", &env_vars) else {
            panic!("Expected a plugin result");
        };
        assert_eq!(result.command, "echo");
        assert_eq!(result.payload, "/home/user");
    }

    #[test]
    fn parse_with_multiple_variables() {
        let env_vars = make_env_vars();
        let ParseResult::Plugin(result) = parse_line("echo $HOME/$USER", &env_vars) else {
            panic!("Expected a plugin result");
        };
        assert_eq!(result.command, "echo");
        assert_eq!(result.payload, "/home/user/john");
    }

    #[test]
    fn parse_with_unknown_variable() {
        let env_vars = make_env_vars();
        let ParseResult::Plugin(result) = parse_line("echo $UNKNOWN", &env_vars) else {
            panic!("Expected a plugin result");
        };
        assert_eq!(result.command, "echo");
        assert_eq!(result.payload, "");
    }

    #[test]
    fn parse_empty_line() {
        let env_vars = make_env_vars();
        let ParseResult::Plugin(result) = parse_line("", &env_vars) else {
            panic!("Expected a plugin result");
        };
        assert_eq!(result.command, "");
        assert_eq!(result.payload, "");
    }

    #[test]
    fn parse_command_only() {
        let env_vars = make_env_vars();
        let ParseResult::Plugin(result) = parse_line("ls", &env_vars) else {
            panic!("Expected a plugin result");
        };
        assert_eq!(result.command, "ls");
        assert_eq!(result.payload, "");
    }

    #[test]
    fn parse_export() {
        let env_vars = make_env_vars();
        let ParseResult::Export((key, value)) = parse_line("export FOO=BAR", &env_vars) else {
            panic!("Expected an export result");
        };
        assert_eq!(key, "FOO");
        assert_eq!(value, "BAR");
    }
}
