use anyhow::Result;
use api::host_api::repl::api::transport;
use clap::Parser;
use cli_host::helpers::{StatusHandler, StdoutHandler};
use cli_host::{WasmEngine, WasmHost};
use std::io::Write;
use std::path::PathBuf;

// Embed the WASM file at compile time
const REPL_LOGIC_WASM: &[u8] = if cfg!(debug_assertions) {
    include_bytes!("../../../target/wasm32-wasip1/debug/repl_logic_guest.wasm")
} else {
    include_bytes!("../../../target/wasm32-wasip1/release/repl_logic_guest.wasm")
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Paths or URLs to WebAssembly plugin files
    #[arg(long)]
    plugins: Vec<String>,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value = ".")]
    dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    let debug = cli.debug;
    println!("[Host] Starting REPL host...");

    // Create a WASI context for the host
    // Binding stdio, args, env, preopened dir ...
    let wasi_ctx = WasmEngine::build_wasi_ctx(&cli.dir)?;

    // Create the WebAssembly engine
    let engine = WasmEngine::new()?;

    // Create the host
    let mut host = WasmHost::new(&engine, wasi_ctx);

    // Load the REPL logic component
    host.load_repl_logic_from_bytes(&engine, REPL_LOGIC_WASM)
        .await?;

    // Load plugins
    for plugin_source in &cli.plugins {
        println!("[Host] Loading plugin: {}", plugin_source);
        host.load_plugin(&engine, plugin_source).await?;
    }

    let mut plugins_config: Vec<(String, String)> = Vec::new();
    for (name, plugin_instance) in &host.plugins {
        let man = plugin_instance
            .plugin
            .repl_api_plugin()
            .call_man(&mut host.store)
            .await?;
        plugins_config.push((name.clone(), man));
        host.store.data_mut().plugins_names.push(name.clone());
    }
    if debug {
        eprintln!("[Host][Debug] Loaded plugins config: {:?}", plugins_config);
    }

    host.store
        .data_mut()
        .repl_vars
        .insert("ROOT".to_string(), "/Users".to_string());
    host.store
        .data_mut()
        .repl_vars
        .insert("USER".to_string(), "Tophe".to_string());
    host.store
        .data_mut()
        .repl_vars
        .insert("?".to_string(), "0".to_string());
    if debug {
        eprintln!(
            "[Host][Debug] Loaded env vars: {:?}",
            host.store.data().repl_vars
        );
    }

    let Some(repl_logic) = host.repl_logic else {
        return Err(anyhow::anyhow!("No REPL logic loaded"));
    };

    loop {
        let mut line = String::new();
        match host.store.data().repl_vars.get("?") {
            Some(last_status) => {
                print!("repl({})> ", last_status);
            }
            None => {
                print!("repl> ");
            }
        }
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut line)?;
        let result = repl_logic
            .repl_api_repl_logic()
            .call_readline(&mut host.store, &line)
            .await?;
        // todo retrieve list of reserved commands from the repl-logic guest

        match result {
            // The built-in commands run in the repl-logic-guest contain stdout, stderr and a status
            // We only need to output them and set the $? variable
            transport::ReadlineResponse::Ready(plugin_response) => {
                if let Some(stdout) = plugin_response.stdout {
                    StdoutHandler::print_and_set_last_result(
                        &mut host.store.data_mut().repl_vars,
                        stdout,
                    );
                }
                if let Some(stderr) = plugin_response.stderr {
                    eprintln!("{}", stderr);
                }
                StatusHandler::set_exit_status(
                    &mut host.store.data_mut().repl_vars,
                    plugin_response.status
                        == api::host_api::repl::api::transport::ReplStatus::Success,
                );
            }
            // The repl-logic-guest parses the command and payload (expanded variables)
            // We run the command of the plugin from the host, which has access to
            // - the plugins
            // - the store
            transport::ReadlineResponse::ToRun(parsed_line) => {
                if debug {
                    eprintln!("[Host][Debug] To run: {:?}", parsed_line);
                }

                // empty line - do nothing
                if parsed_line.command == "" {
                    continue;
                }

                // this is a man command for plugins, we run it from the host
                if parsed_line.command == "man" {
                    let Some(plugin_instance) = host.plugins.get(&parsed_line.payload) else {
                        println!(
                            "Unknown command: {}. Try `help` to see available commands.",
                            parsed_line.payload
                        );
                        StatusHandler::set_exit_status(&mut host.store.data_mut().repl_vars, false);
                        continue;
                    };
                    let man = plugin_instance
                        .plugin
                        .repl_api_plugin()
                        .call_man(&mut host.store)
                        .await?;
                    StdoutHandler::print_and_set_last_result(
                        &mut host.store.data_mut().repl_vars,
                        man,
                    );
                    StatusHandler::set_exit_status(&mut host.store.data_mut().repl_vars, true);
                    continue;
                }

                // this is a plugin command, we run it from the host
                match host.plugins.get(&parsed_line.command) {
                    Some(plugin_instance) => {
                        let result = plugin_instance
                            .plugin
                            .repl_api_plugin()
                            .call_run(&mut host.store, &parsed_line.payload)
                            .await?;
                        if let Ok(result) = result {
                            if let Some(stdout) = result.stdout {
                                StdoutHandler::print_and_set_last_result(
                                    &mut host.store.data_mut().repl_vars,
                                    stdout,
                                );
                            }
                            if let Some(stderr) = result.stderr {
                                eprintln!("{}", stderr);
                            }
                            StatusHandler::set_exit_status(
                                &mut host.store.data_mut().repl_vars,
                                result.status
                                    == api::plugin_api::repl::api::transport::ReplStatus::Success,
                            );
                        } else {
                            eprintln!("Error: {:?}", result);
                            StatusHandler::set_exit_status(
                                &mut host.store.data_mut().repl_vars,
                                false,
                            );
                        }
                    }
                    None => {
                        println!(
                            "Unknown command: {}. Try `help` to see available commands.",
                            parsed_line.command
                        );
                        StatusHandler::set_exit_status(&mut host.store.data_mut().repl_vars, false);
                        continue;
                    }
                }
            }
        }
    }
}
