use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::io::Write;
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

    // // Load plugins
    // for plugin_path in &cli.plugins {
    //     info!("Loading plugin: {}", plugin_path.display());
    //     host.load_plugin(&engine, plugin_path.clone()).await?;
    // }

    // // Get plugin names
    // let names = host.plugin_names().await;
    // println!("[Host] Loaded plugins: {:?}", names);

    // // Example: Set some environment variables in the store from the host level
    // host.set_store_env_var("ROOT".to_string(), "/Users".to_string());
    // host.set_store_env_var("USER".to_string(), "Tophe".to_string());

    // // Example: Add a plugin configuration to the store
    // let plugin_config = api::host_api::repl::api::transport::PluginConfig {
    //     command: "example".to_string(),
    //     arg_count: Some(1),
    //     man: "Example plugin for demonstration".to_string(),
    // };
    // host.add_plugin_config(plugin_config);

    // // Example: Access store data from host level
    // println!("[Host] Stored env vars: {:?}", host.get_all_store_env_vars());
    // println!("[Host] Stored plugin configs: {:?}", host.get_plugin_configs());

    host.store.data_mut().repl_env_vars.insert("ROOT".to_string(), "/Users".to_string());
    host.store.data_mut().repl_env_vars.insert("USER".to_string(), "Tophe".to_string());

    let Some(repl_logic) = host.repl_logic else {
        return Err(anyhow::anyhow!("No REPL logic loaded"));
    };

    // let repl_state = repl_logic.repl_api_repl_logic().

    loop {
        let mut line = String::new();
        print!("repl> ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut line)?;
        let result = repl_logic.repl_api_repl_logic().call_readline(&mut host.store, &line).await?;
        println!("{:?}", result);
    }
}
