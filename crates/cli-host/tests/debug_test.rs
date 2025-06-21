use rexpect::spawn;

#[test]
fn debug_output() {
    std::env::set_current_dir("/Users/tophe/projects/webassembly-component-model-repl").unwrap();
    let mut session = spawn(
        "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_greet.wasm",
        Some(5000),
    )
    .expect("Can't launch cli-host");

    // Read the startup message
    match session.exp_string("[Host] Starting REPL host...") {
        Ok(output) => println!("Got startup: '{}'", output),
        Err(e) => println!("Startup error: {:?}", e),
    }

    // Read the plugin loading message
    match session.exp_string("[Host] Loading plugin:") {
        Ok(output) => println!("Got plugin loading: '{}'", output),
        Err(e) => println!("Plugin loading error: {:?}", e),
    }

    // Read the plugin configs message
    match session.exp_string("[Host] Loaded plugin configs:") {
        Ok(output) => println!("Got plugin configs: '{}'", output),
        Err(e) => println!("Plugin configs error: {:?}", e),
    }

    // Read the env vars message
    match session.exp_string("[Host] Loaded env vars:") {
        Ok(output) => println!("Got env vars: '{}'", output),
        Err(e) => println!("Env vars error: {:?}", e),
    }

    // Read the REPL prompt
    match session.exp_string("repl>") {
        Ok(output) => println!("Got REPL prompt: '{}'", output),
        Err(e) => println!("REPL prompt error: {:?}", e),
    }

    // Now send a command
    match session.send_line("greet Test") {
        Ok(_) => println!("Sent command successfully"),
        Err(e) => println!("Send command error: {:?}", e),
    }

    // Read the response
    match session.exp_string("Hello, Test!") {
        Ok(output) => println!("Got response: '{}'", output),
        Err(e) => println!("Response error: {:?}", e),
    }
}
