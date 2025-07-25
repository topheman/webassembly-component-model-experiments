mod utils;

#[cfg(test)]
mod e2e_cli_host {

    use crate::utils::*;

    use rexpect::spawn;

    const TEST_TIMEOUT: u64 = 10000;

    /**
     * Lets us change the target directory for the plugins and repl logic.
     *
     * See the justfile for examples where we switch to testing both plugins from the filesystem and from the HTTP server.
     */
    fn build_command(plugin_files: &[&str], repl_logic_file: &str) -> String {
        let prefix =
            std::env::var("WASM_TARGET_DIR").unwrap_or("target/wasm32-wasip1/debug".to_string());
        let mut command = String::from("target/debug/pluginlab");
        command.push_str(format!(" --repl-logic {}/{}", prefix, repl_logic_file).as_str());
        plugin_files.iter().for_each(|file| {
            command.push_str(format!(" --plugins {}", file).as_str());
        });
        println!("Running command: {}", command);
        command
    }

    #[test]
    fn test_without_permission_allow_network() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &build_command(
                &["target/wasm32-wasip1/debug/plugin_weather.wasm"],
                "repl_logic_guest.wasm",
            ),
            Some(TEST_TIMEOUT),
        )
        .expect("Can't launch pluginlab with plugin greet");

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
            .send_line("weather Paris")
            .expect("Failed to send command");
        session
            .exp_string("Error fetching weather: PermissionDenied: network access to wttr.in is not allowed")
            .expect("Didn't get expected error output");
    }

    #[test]
    fn test_without_permission_allow_read() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &format!(
                "{} --dir tmp/filesystem",
                &build_command(
                    &["target/wasm32-wasip1/debug/plugin_ls.wasm"],
                    "repl_logic_guest.wasm"
                )
            ),
            Some(TEST_TIMEOUT),
        )
        .expect("Can't launch pluginlab with plugin greet");

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
            .exp_string("ls: : Operation not permitted")
            .expect("Didn't get expected error output");
    }
}
