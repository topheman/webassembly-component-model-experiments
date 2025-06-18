use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::io::Write;
use tracing::info;
use cli_host::{Host, WasmEngine};
use api::host_api::exports::repl::api::repl_logic::ReplEnvVar;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Paths to WebAssembly plugin files
    #[arg(long)]
    plugins: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();
    info!("Starting REPL host...");

    // Create the WebAssembly engine
    let engine = WasmEngine::new()?;

    // Create the host
    let mut host = Host::new(&engine);

    // Load the REPL logic component
    let repl_logic_path = PathBuf::from("target/wasm32-wasip1/debug/repl_logic_guest.wasm"); // todo use a config file ?
    host.load_repl_logic(&engine, repl_logic_path).await?;

    // Load plugins
    for plugin_path in &cli.plugins {
        info!("Loading plugin: {}", plugin_path.display());
        host.load_plugin(&engine, plugin_path.clone()).await?;
    }

    // Get plugin names
    let names = host.plugin_names().await;
    println!("Loaded plugins: {:?}", names);

    let Some(repl_logic) = host.repl_logic else {
        return Err(anyhow::anyhow!("No REPL logic loaded"));
    };
    let result = repl_logic.repl_api_repl_logic().call_readline(&mut host.store, "Hello, world!").await?;
    println!("[Host] REPL logic result: {:?}", result);

    // TODO: Load REPL logic
    // TODO: Start REPL loop with command parsing and plugin dispatch

    // it will be will be handled by the repl-logic component
    let env_vars = vec![
        ReplEnvVar {
            key: "HOME".to_string(),
            value: "/home/user".to_string(),
        },
        ReplEnvVar {
            key: "USER".to_string(),
            value: "john".to_string(),
        },
    ];

    // let repl_state = repl_logic.repl_api_repl_logic().

    loop {
        let mut line = String::new();
        print!("repl> ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut line)?;
        let result = repl_logic.repl_api_repl_logic().call_readline(&mut host.store, &line).await?;
        println!("{:?}", result);
    }

    Ok(())
}
