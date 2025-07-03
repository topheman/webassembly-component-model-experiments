#[cfg(test)]
mod e2e_test {

    use rexpect::spawn;
    use std::path::PathBuf;

    const TEST_TIMEOUT: u64 = 10000;

    fn find_project_root() -> PathBuf {
        let mut current = std::env::current_dir().unwrap();
        println!("Starting search from: {:?}", current);

        // Walk up the directory tree looking for the workspace root Cargo.toml
        loop {
            let cargo_toml = current.join("Cargo.toml");
            if cargo_toml.exists() {
                // Check if this is the workspace root by looking for [workspace] section
                if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                    if content.contains("[workspace]") {
                        println!("Found workspace root at: {:?}", current);
                        return current;
                    }
                }
            }

            if !current.pop() {
                // current.pop() moves up one directory in the path. If we're already at the root, it returns false.
                panic!("Could not find workspace root (Cargo.toml with [workspace])");
            }
        }
    }

    #[test]
    fn test_load() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_greet.wasm",
            Some(TEST_TIMEOUT),
        )
        .expect("Can't launch cli-host with plugin greet");

        session
            .exp_string("[Host] Starting REPL host...")
            .expect("Didn't see startup message");
        session
            .exp_string("[Host] Loading plugin:")
            .expect("Didn't see plugin loading message");
        session
            .exp_string("repl(0)>")
            .expect("Didn't see REPL prompt");
    }

    #[test]
    fn test_basic_repl() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_greet.wasm",
            Some(TEST_TIMEOUT),
        )
        .expect("Can't launch cli-host with plugin greet");

        session
            .exp_string("[Host] Starting REPL host...")
            .expect("Didn't see startup message");
        session
            .exp_string("[Host] Loading plugin:")
            .expect("Didn't see plugin loading message");
        session
            .exp_string("repl(0)>")
            .expect("Didn't see REPL prompt");

        session
            .send_line("greet Tophe")
            .expect("Failed to send command");
        session
            .exp_string("Hello, Tophe!")
            .expect("Didn't get expected greeting output");
        session
            .exp_string("repl(0)>")
            .expect("Didn't see next REPL prompt");
    }

    #[test]
    fn test_vars_repl() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_greet.wasm",
            Some(TEST_TIMEOUT),
        )
        .expect("Can't launch cli-host with plugin greet");

        session
            .exp_string("[Host] Starting REPL host...")
            .expect("Didn't see startup message");
        session
            .exp_string("[Host] Loading plugin:")
            .expect("Didn't see plugin loading message");
        session
            .exp_string("repl(0)>")
            .expect("Didn't see REPL prompt");

        session
            .send_line("greet $USER")
            .expect("Failed to send command");
        session
            .exp_string("Hello, Tophe!")
            .expect("Didn't get expected greeting output with variable substitution");
        session
            .exp_string("repl(0)>")
            .expect("Didn't see next REPL prompt");
    }

    #[test]
    fn test_unknown_command() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_greet.wasm",
            Some(TEST_TIMEOUT),
        )
        .expect("Can't launch cli-host with plugin greet");

        session
            .exp_string("[Host] Starting REPL host...")
            .expect("Didn't see startup message");
        session
            .exp_string("[Host] Loading plugin:")
            .expect("Didn't see plugin loading message");
        session
            .exp_string("repl(0)>")
            .expect("Didn't see REPL prompt");

        session
            .send_line("nonexistingcmd foo bar")
            .expect("Failed to send command");
        session
            .exp_string("Unknown command: nonexistingcmd. Try `help` to see available commands")
            .expect("Didn't get expected error output");
        session
            .exp_string("repl(1)>")
            .expect("Didn't see next REPL prompt with $? set to 1");
    }

    #[test]
    fn test_empty_line() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_greet.wasm",
            Some(TEST_TIMEOUT),
        )
        .expect("Can't launch cli-host with plugin greet");

        session
            .exp_string("[Host] Starting REPL host...")
            .expect("Didn't see startup message");
        session
            .exp_string("[Host] Loading plugin:")
            .expect("Didn't see plugin loading message");
        session
            .exp_string("repl(0)>")
            .expect("Didn't see REPL prompt");

        session.send_line("").expect("Failed to send command");
        session
            .exp_string("repl(0)>")
            .expect("Empty command should lead to a new prompt");

        session
            .send_line("nonexistingcmd foo bar")
            .expect("Failed to send command");
        session
            .exp_string("Unknown command: nonexistingcmd. Try `help` to see available commands")
            .expect("Didn't get expected error output");
        session
            .exp_string("repl(1)>")
            .expect("Didn't see next REPL prompt with $? set to 1");

        session.send_line("").expect("Failed to send command");
        session
            .exp_string("repl(1)>")
            .expect("Empty command should lead to a new prompt, without changing $?");
    }

    #[test]
    fn test_ls_plugin() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_ls.wasm --dir wit",
            Some(TEST_TIMEOUT),
        )
        .expect("Can't launch cli-host with plugin greet");

        session
            .exp_string("[Host] Starting REPL host...")
            .expect("Didn't see startup message");
        session
            .exp_string("[Host] Loading plugin:")
            .expect("Didn't see plugin loading message");
        session
            .exp_string("repl(0)>")
            .expect("Didn't see REPL prompt");
        session.send_line("ls").expect("Failed to send command");
        session
            .exp_string("host-api.wit\r\nplugin-api.wit\r\nshared.wit\r\n")
            .expect("Didn't get listing of current directory");
    }

    #[test]
    fn test_0_variable_set() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_echo.wasm",
            Some(TEST_TIMEOUT),
        )
        .expect("Can't launch cli-host with plugin greet");

        session
            .exp_string("[Host] Starting REPL host...")
            .expect("Didn't see startup message");
        session
            .exp_string("[Host] Loading plugin:")
            .expect("Didn't see plugin loading message");
        session
            .exp_string("repl(0)>")
            .expect("Didn't see REPL prompt");
        session
            .send_line("echo foo bar")
            .expect("Failed to send command");
        session
            .exp_string("foo bar")
            .expect("Didn't get expected echo output");
        session
            .send_line("echo $0")
            .expect("Failed to send command");
        session
            .exp_string("foo bar")
            .expect("Didn't get expected echo output for $0");

        session
            .send_line("nonexistingcmd foo bar")
            .expect("Failed to send command");
        session
            .exp_string("Unknown command: nonexistingcmd. Try `help` to see available commands")
            .expect("Didn't get expected error output");
        session
            .send_line("echo $0 yolo") // add yolo so that the expect doesn't match previous instances of "foo bar" only
            .expect("Failed to send command");
        session
            .exp_string("foo bar yolo")
            .expect("Didn't get expected echo output for $0 - should be the same as the last non failingcommand");
    }
}
