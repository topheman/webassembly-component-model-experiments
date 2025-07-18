use std::path::PathBuf;

pub fn find_project_root() -> PathBuf {
    let mut current = std::env::current_dir().unwrap();
    println!("Starting search from: {:?}", current);

    // Walk up the directory tree looking for the workspace root Cargo.toml
    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            // Check if this is the workspace root by looking for [workspace] section
            if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                if content.contains("[workspace]") {
                    println!("Found workspace root at: {:?}", current);
                    return current;
                }
            }
        }

        if !current.pop() {
            // current.pop() moves up one directory in the path. If we're already at the root, it returns false.
            panic!("Could not find workspace root (Cargo.toml with [workspace])");
        }
    }
}
