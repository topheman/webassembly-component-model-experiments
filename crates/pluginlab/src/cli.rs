use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Paths or URLs to WebAssembly plugin files
    #[arg(long)]
    pub plugins: Vec<String>,

    /// Path or URL to WebAssembly REPL logic file
    #[arg(long)]
    pub repl_logic: String,

    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Path to the directory to mount (the runtime will only have access to this directory) - default is the current directory
    #[arg(long, default_value = ".")]
    pub dir: PathBuf,

    /// Allow network access
    #[arg(long, num_args = 0..=1, default_missing_value = "true")]
    pub allow_net: Option<String>,

    /// Allow file system read access
    #[arg(long, default_value_t = false)]
    pub allow_read: bool,

    /// Allow file system write access
    #[arg(long, default_value_t = false)]
    pub allow_write: bool,

    /// Allow all permissions
    #[arg(
        short = 'A',
        long,
        default_value_t = false,
        conflicts_with = "allow_net",
        conflicts_with = "allow_read",
        conflicts_with = "allow_write"
    )]
    pub allow_all: bool,
}
