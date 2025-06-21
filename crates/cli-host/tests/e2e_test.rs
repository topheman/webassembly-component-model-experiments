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
            .exp_string("[Host] Loaded plugins config:")
            .expect("Didn't see plugin configs");
        session
            .exp_string("[Host] Loaded env vars:")
            .expect("Didn't see env vars");
        session.exp_string("repl>").expect("Didn't see REPL prompt");
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
            .exp_string("[Host] Loaded plugins config:")
            .expect("Didn't see plugin configs");
        session
            .exp_string("[Host] Loaded env vars:")
            .expect("Didn't see env vars");
        session.exp_string("repl>").expect("Didn't see REPL prompt");

        session
            .send_line("greet Tophe")
            .expect("Failed to send command");
        session
            .exp_string("PluginResponse { status: ReplStatus::Success, stdout: Some(\"Plugin 'greet' executed with payload: Tophe\"), stderr: None }")
            .expect("Didn't get expected PluginResponse");
        session
            .exp_string("repl>")
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
            .exp_string("[Host] Loaded plugins config:")
            .expect("Didn't see plugin configs");
        session
            .exp_string("[Host] Loaded env vars:")
            .expect("Didn't see env vars");
        session.exp_string("repl>").expect("Didn't see REPL prompt");

        session
            .send_line("greet $USER")
            .expect("Failed to send command");
        session
            .exp_string("PluginResponse { status: ReplStatus::Success, stdout: Some(\"Plugin 'greet' executed with payload: Tophe\"), stderr: None }")
            .expect("Didn't get expected PluginResponse with variable substitution");
        session
            .exp_string("repl>")
            .expect("Didn't see next REPL prompt");
    }
}
