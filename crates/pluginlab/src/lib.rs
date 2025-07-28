pub(crate) mod api;
pub(crate) mod cli;
pub(crate) mod engine;
pub(crate) mod helpers;
pub(crate) mod permissions;
pub(crate) mod store;
pub(crate) mod wasm_host;

pub(crate) use engine::WasmEngine;
pub(crate) use wasm_host::WasmHost;

use anyhow::Result;
use api::host_api::repl::api::transport;
use clap::Parser;
use cli::Cli;
use helpers::{StatusHandler, StdoutHandler};
use std::io::Write;

/// Main entry point for the REPL application
pub async fn run_async() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    let debug = cli.debug;
    println!("[Host] Starting REPL host...");

    // Create a WASI context for the host
    // Binding stdio, args, env, preopened dir ...
    let wasi_ctx = WasmEngine::build_wasi_ctx(&cli)?;

    // Create the WebAssembly engine
    let engine = WasmEngine::new()?;

    // Create the host
    let mut host = WasmHost::new(&engine, wasi_ctx, &cli);

    println!("[Host] Loading REPL logic from: {}", cli.repl_logic);
    // Override the REPL logic in the binary with the one passed by params
    host.load_repl_logic(&engine, &cli.repl_logic).await?;

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

    {
        let mut repl_vars = host
            .store
            .data_mut()
            .repl_vars
            .lock()
            .expect("Failed to acquire repl_vars lock");
        repl_vars.insert("ROOT".to_string(), "/Users".to_string());
    }
    {
        let mut repl_vars = host
            .store
            .data_mut()
            .repl_vars
            .lock()
            .expect("Failed to acquire repl_vars lock");
        repl_vars.insert("USER".to_string(), "Tophe".to_string());
        repl_vars.insert("?".to_string(), "0".to_string());
    }
    if debug {
        eprintln!(
            "[Host][Debug] Loaded env vars: {:?}",
            host.store
                .data()
                .repl_vars
                .lock()
                .expect("Failed to acquire repl_vars lock")
        );
    }

    let Some(repl_logic) = host.repl_logic else {
        return Err(anyhow::anyhow!("No REPL logic loaded"));
    };

    loop {
        let mut line = String::new();
        match host
            .store
            .data()
            .repl_vars
            .lock()
            .expect("Failed to acquire repl_vars lock")
            .get("?")
        {
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
