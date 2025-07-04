use std::collections::HashMap;

/// Handles setting exit status codes in REPL variables
pub struct StatusHandler;

impl StatusHandler {
    /// Set the exit status in the REPL variables
    pub fn set_exit_status(repl_vars: &mut HashMap<String, String>, success: bool) {
        let status = if success { "0" } else { "1" };
        repl_vars.insert("?".to_string(), status.to_string());
    }
}

pub struct StdoutHandler;

impl StdoutHandler {
    pub fn print_and_set_last_result(repl_vars: &mut HashMap<String, String>, result: String) {
        println!("{}", result);
        repl_vars.insert("0".to_string(), result);
    }
}
