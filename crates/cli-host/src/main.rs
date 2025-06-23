use anyhow::Result;
use api::host_api::repl::api::transport;
use clap::Parser;
use cli_host::{StatusHandler, WasmEngine, WasmHost};
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Paths to WebAssembly plugin files
    #[arg(long)]
    plugins: Vec<PathBuf>,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    let debug = cli.debug;
    println!("[Host] Starting REPL host...");

    // Create the WebAssembly engine
    let engine = WasmEngine::new()?;

    // Create the host
    let mut host = WasmHost::new(&engine);

    // Load the REPL logic component
    let repl_logic_path = PathBuf::from("target/wasm32-wasip1/debug/repl_logic_guest.wasm"); // todo use a config file ?
    host.load_repl_logic(&engine, repl_logic_path).await?;

    // // Load plugins
    for plugin_path in &cli.plugins {
        println!("[Host] Loading plugin: {}", plugin_path.display());
        host.load_plugin(&engine, plugin_path.clone()).await?;
    }

    let mut plugins_config: Vec<(String, Option<i8>, String)> = Vec::new();
    for (name, plugin_instance) in &host.plugins {
        let arg_count = plugin_instance
            .plugin
            .repl_api_plugin()
            .call_arg_count(&mut host.store)
            .await?;
        let man = plugin_instance
            .plugin
            .repl_api_plugin()
            .call_man(&mut host.store)
            .await?;
        plugins_config.push((name.clone(), arg_count, man));
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
                    println!("{}", stdout);
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
                match host.plugins.get(&parsed_line.command) {
                    Some(plugin_instance) => {
                        let result = plugin_instance
                            .plugin
                            .repl_api_plugin()
                            .call_run(&mut host.store, &parsed_line.payload)
                            .await?;
                        if let Ok(result) = result {
                            if let Some(stdout) = result.stdout {
                                println!("{}", stdout);
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
                    }
                }
            }
        }
    }
}
