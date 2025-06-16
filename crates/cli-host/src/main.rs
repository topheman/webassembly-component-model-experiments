use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::info;
use cli_host::Host;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Paths to WebAssembly plugin files
    #[arg(required = true)]
    plugins: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();
    info!("Starting REPL host...");

    // Initialize host
    let mut host = Host::new()?;
    info!("WebAssembly engine initialized");

    // Load plugins
    for plugin_path in &cli.plugins {
        info!("Loading plugin: {}", plugin_path.display());
        host.load_plugin(plugin_path.clone()).await?;
    }

    // Print welcome message
    println!("Welcome to repl");
    println!("Active plugins: {}", host.plugin_names().join(", "));

    // TODO: Load REPL logic
    // TODO: Start REPL loop with command parsing and plugin dispatch

    Ok(())
}
