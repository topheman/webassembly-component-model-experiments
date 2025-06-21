use anyhow::Result;
use clap::Parser;
use cli_host::{Host, WasmEngine};
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Paths to WebAssembly plugin files
    #[arg(long)]
    plugins: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    println!("[Host] Starting REPL host...");

    // Create the WebAssembly engine
    let engine = WasmEngine::new()?;

    // Create the host
    let mut host = Host::new(&engine);

    // Load the REPL logic component
    let repl_logic_path = PathBuf::from("target/wasm32-wasip1/debug/repl_logic_guest.wasm"); // todo use a config file ?
    host.load_repl_logic(&engine, repl_logic_path).await?;

    // // Load plugins
    for plugin_path in &cli.plugins {
        println!("[Host] Loading plugin: {}", plugin_path.display());
        host.load_plugin(&engine, plugin_path.clone()).await?;
    }

    let mut plugins_config: Vec<(String, Option<i8>, String)> = Vec::new();
    for plugin in &host.plugins {
        let name = plugin
            .plugin
            .repl_api_plugin()
            .call_name(&mut host.store)
            .await?;
        let arg_count = plugin
            .plugin
            .repl_api_plugin()
            .call_arg_count(&mut host.store)
            .await?;
        let man = plugin
            .plugin
            .repl_api_plugin()
            .call_man(&mut host.store)
            .await?;
        plugins_config.push((name, arg_count, man));
    }
    println!("[Host] Loaded plugin configs: {:?}", plugins_config);

    host.store
        .data_mut()
        .repl_vars
        .insert("ROOT".to_string(), "/Users".to_string());
    host.store
        .data_mut()
        .repl_vars
        .insert("USER".to_string(), "Tophe".to_string());
    println!("[Host] Loaded env vars: {:?}", host.store.data().repl_vars);

    let Some(repl_logic) = host.repl_logic else {
        return Err(anyhow::anyhow!("No REPL logic loaded"));
    };

    loop {
        let mut line = String::new();
        print!("repl> ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut line)?;
        let result = repl_logic
            .repl_api_repl_logic()
            .call_readline(&mut host.store, &line)
            .await?;
        println!("{:?}", result);
    }
}
