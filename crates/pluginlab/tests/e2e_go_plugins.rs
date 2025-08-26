mod utils;

#[cfg(test)]
mod e2e_go_plugins {

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
    fn test_echo_plugin() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &format!(
                "{} --dir tmp/filesystem --allow-read",
                &build_command(
                    &["go_modules/plugin-echo/plugin-echo-go.wasm"],
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
        session
            .send_line("echogo hello")
            .expect("Failed to send command");
        session
            .exp_string("hello\r\nrepl(0)>")
            .expect("Didn't get expected output from echogo plugin");
    }
}
