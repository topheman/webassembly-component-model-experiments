#[cfg(test)]
mod e2e_test {

    use rexpect::spawn;
    use std::path::PathBuf;

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
            command.push_str(format!(" --plugins {}/{}", prefix, file).as_str());
        });
        println!("Running command: {}", command);
        command
    }

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
            &build_command(&["plugin_greet.wasm"], "repl_logic_guest.wasm"),
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
    }

    #[test]
    fn test_basic_repl() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &build_command(&["plugin_greet.wasm"], "repl_logic_guest.wasm"),
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
            &build_command(&["plugin_greet.wasm"], "repl_logic_guest.wasm"),
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
            &build_command(&["plugin_greet.wasm"], "repl_logic_guest.wasm"),
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
    fn test_empty_line_and_dollar_question_mark() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &build_command(&["plugin_echo.wasm"], "repl_logic_guest.wasm"),
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
        session
            .send_line("echo $?")
            .expect("Failed to send command");
        session
            .exp_string("1")
            .expect("Didn't get expected echo output for $? after previous command failed");
        session
            .send_line("echo $?")
            .expect("Failed to send command");
        session
            .exp_string("0")
            .expect("Didn't get expected echo output for $? after previous command succeeded");
    }

    #[test]
    fn test_ls_plugin() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &format!(
                "{} --dir tmp/filesystem --allow-read",
                &build_command(&["plugin_ls.wasm"], "repl_logic_guest.wasm")
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
            .exp_string(
                "D\tdata\r\nD\tdocuments\r\nD\tlogs\r\nF\t.config\r\nF\t.hidden_file\r\nF\tREADME.md\r\n",
            )
            .expect("Didn't get listing of current directory");
        session
            .send_line("ls documents")
            .expect("Failed to send command");
        session
            .exp_string("D\tdocuments/work\r\nF\tdocuments/README.md\r\nF\tdocuments/config.json\r\nF\tdocuments/notes.txt\r\n")
            .expect("Didn't get listing of documents directory");
        session
            .send_line("ls documents/work/projects")
            .expect("Failed to send command");
        session
            .exp_string("D\tdocuments/work/projects/alpha\r\nD\tdocuments/work/projects/beta\r\nF\tdocuments/work/projects/.gitkeep\r\n")
            .expect("Didn't get listing of projects directory");
        session
            .send_line("ls data/sample.csv")
            .expect("Failed to send command");
        session
            .exp_string("F\tdata/sample.csv\r\n")
            .expect("Didn't get listing for a single file");
    }

    #[test]
    fn test_cat_plugin() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &format!(
                "{} --dir tmp/filesystem --allow-read",
                &build_command(&["plugin_cat.wasm"], "repl_logic_guest.wasm")
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
            .send_line("cat documents")
            .expect("Failed to send command");
        session
            .exp_string("cat: documents: Is a directory")
            .expect("Didn't get expected error output for trying to cat a directory");
        session
            .send_line("cat documents/README.md")
            .expect("Failed to send command");
        session
            .exp_string("# Documents")
            .expect("Didn't get expected contents of README.md");
    }

    #[test]
    fn test_0_variable_set() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &build_command(&["plugin_echo.wasm"], "repl_logic_guest.wasm"),
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

    #[test]
    fn test_without_permission_allow_network() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &build_command(&["plugin_weather.wasm"], "repl_logic_guest.wasm"),
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
                &build_command(&["plugin_ls.wasm"], "repl_logic_guest.wasm")
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

    #[test]
    fn test_list_commands() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &build_command(
                &["plugin_echo.wasm", "plugin_greet.wasm", "plugin_cat.wasm"],
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
            .send_line("list-commands")
            .expect("Failed to send command");
        session
            .exp_string("cat\tplugin\r\necho\tplugin\r\nexport\treserved\r\ngreet\tplugin\r\nhelp\treserved\r\nlist-commands\treserved\r\nman\treserved\r\n")
            .expect("Didn't get expected list of commands");
    }

    #[test]
    fn test_man_command() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &build_command(&["plugin_greet.wasm"], "repl_logic_guest.wasm"),
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
        session.send_line("man").expect("Failed to send command");
        session
            .exp_string("man - Show the manual for a command")
            .expect("Didn't get expected manual output");
        session
            .send_line("man man")
            .expect("Failed to send command");
        session
            .exp_string("man - Show the manual for a command")
            .expect("Didn't get expected manual output");
    }

    #[test]
    fn test_man_echo() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &build_command(&["plugin_echo.wasm"], "repl_logic_guest.wasm"),
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
            .send_line("man echo")
            .expect("Failed to send command");
        session
            .exp_string("echo - Echo a message (built with Rust")
            .expect("Didn't get expected manual output");
    }
}
