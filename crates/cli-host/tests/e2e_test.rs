use rexpect::spawn;

#[test]
fn test_load() {
    let mut session = spawn(
        "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_greet.wasm",
        Some(5000),
    )
    .expect("Can't launch cli-host with plugin greet");

    session
        .exp_string("[Host] Starting REPL host...")
        .expect("Didn't see startup message");
    session
        .exp_string("[Host] Loading plugin:")
        .expect("Didn't see plugin loading message");
    session
        .exp_string("[Host] Loaded plugin configs:")
        .expect("Didn't see plugin configs");
    session
        .exp_string("[Host] Loaded env vars:")
        .expect("Didn't see env vars");
    session.exp_string("repl>").expect("Didn't see REPL prompt");
}

#[test]
fn test_basic_repl() {
    let mut session = spawn(
        "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_greet.wasm",
        Some(5000),
    )
    .expect("Can't launch cli-host with plugin greet");

    session
        .exp_string("[Host] Starting REPL host...")
        .expect("Didn't see startup message");
    session
        .exp_string("[Host] Loading plugin:")
        .expect("Didn't see plugin loading message");
    session
        .exp_string("[Host] Loaded plugin configs:")
        .expect("Didn't see plugin configs");
    session
        .exp_string("[Host] Loaded env vars:")
        .expect("Didn't see env vars");
    session.exp_string("repl>").expect("Didn't see REPL prompt");

    session
        .send_line("greet Tophe")
        .expect("Failed to send command");
    session
        .exp_string("Hello, Tophe!")
        .expect("Didn't get expected greeting");
}

#[test]
fn test_vars_repl() {
    let mut session = spawn(
        "target/debug/cli-host --plugins target/wasm32-wasip1/debug/plugin_greet.wasm",
        Some(5000),
    )
    .expect("Can't launch cli-host with plugin greet");

    session
        .exp_string("[Host] Starting REPL host...")
        .expect("Didn't see startup message");
    session
        .exp_string("[Host] Loading plugin:")
        .expect("Didn't see plugin loading message");
    session
        .exp_string("[Host] Loaded plugin configs:")
        .expect("Didn't see plugin configs");
    session
        .exp_string("[Host] Loaded env vars:")
        .expect("Didn't see env vars");
    session.exp_string("repl>").expect("Didn't see REPL prompt");

    session
        .send_line("greet $USER")
        .expect("Failed to send command");
    session
        .exp_string("Hello, Tophe!")
        .expect("Didn't get expected greeting with variable substitution");
}
