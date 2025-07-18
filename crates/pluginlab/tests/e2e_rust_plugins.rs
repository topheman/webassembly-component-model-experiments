mod utils;

#[cfg(test)]
mod e2e_rust_plugins {

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
            command.push_str(format!(" --plugins {}/{}", prefix, file).as_str());
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
                &build_command(&["plugin_echo.wasm"], "repl_logic_guest.wasm")
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
            .send_line("echo hello")
            .expect("Failed to send command");
        session
            .exp_string("hello\r\n")
            .expect("Didn't get expected output from echo plugin");
    }

    #[test]
    fn test_greet_plugin() {
        let project_root = find_project_root();
        println!("Setting current directory to: {:?}", project_root);
        std::env::set_current_dir(&project_root).unwrap();
        let mut session = spawn(
            &format!(
                "{} --dir tmp/filesystem --allow-read",
                &build_command(&["plugin_greet.wasm"], "repl_logic_guest.wasm")
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
            .send_line("greet World")
            .expect("Failed to send command");
        session
            .exp_string("Hello, World!\r\n")
            .expect("Didn't get expected output from greet plugin");
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
}
