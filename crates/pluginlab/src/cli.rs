use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Paths or URLs to WebAssembly plugin files
    #[arg(long)]
    pub plugins: Vec<String>,

    /// Path or URL to WebAssembly REPL logic file
    #[arg(long)]
    pub repl_logic: Option<String>,

    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Path to the directory to mount (the runtime will only have access to this directory)
    #[arg(long, default_value = ".")]
    pub dir: PathBuf,

    /// Allow network access
    #[arg(short = 'N', long, num_args = 0..=1, default_missing_value = "@")]
    // How it works:
    // no flag -> None
    // --allow-net -> Some("@") - because "@" is not a valid value for a domain nor an IP address
    // --allow-net google.com,example.com -> Some("google.com,example.com")
    pub allow_net: Option<String>,

    /// Allow file system read access
    #[arg(short = 'R', long, default_value_t = false)]
    pub allow_read: bool,

    /// Allow file system write access
    #[arg(short = 'W', long, default_value_t = false)]
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

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate completions for your own shell (shipped with the homebrew version)
    GenerateCompletions {
        /// Specify which shell you target - accepted values: bash, fish, zsh
        #[arg(long, value_enum)]
        shell: AvailableShells,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum AvailableShells {
    Bash,
    Fish,
    Zsh,
}
