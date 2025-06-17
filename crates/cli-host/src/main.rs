use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::info;
use cli_host::{Host, WasmEngine};

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

    // TODO: Load REPL logic
    // TODO: Start REPL loop with command parsing and plugin dispatch

    Ok(())
}
