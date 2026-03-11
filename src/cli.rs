use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rskm", about = "Rust SSH Key Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Key {
        #[command(subcommand)]
        action: KeyAction,
    },
    Host {
        #[command(subcommand)]
        action: HostAction,
    },
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    Agent {
        #[command(subcommand)]
        action: AgentAction,
    },
}

#[derive(Subcommand)]
pub enum KeyAction {
    Generate {
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "ed25519")]
        r#type: String,
        #[arg(long)]
        comment: Option<String>,
        #[arg(long)]
        passphrase: bool,
    },
    List,
    Delete { name: String },
    Rotate { name: String },
}

#[derive(Subcommand)]
pub enum HostAction {
    Add {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        hostname: Option<String>,
        #[arg(long)]
        user: Option<String>,
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        port: Option<u16>,
        #[arg(long)]
        proxy_jump: Option<String>,
    },
    List {
        #[arg(long)]
        json: bool,
    },
    Edit {
        name: String,
        #[arg(long)]
        hostname: Option<String>,
        #[arg(long)]
        user: Option<String>,
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        port: Option<u16>,
        #[arg(long)]
        proxy_jump: Option<String>,
    },
    Remove {
        name: String,
        #[arg(long, short)]
        force: bool,
    },
    Show {
        name: String,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    Generate {
        #[arg(long)]
        dry_run: bool,
        #[arg(long, short)]
        output: Option<String>,
        #[arg(long)]
        no_backup: bool,
    },
    Diff,
    Path,
}

#[derive(Subcommand)]
pub enum AgentAction {
    Load {
        /// Specific key name; if omitted, loads all managed keys
        name: Option<String>,
        /// Key lifetime in seconds (passed as -t to ssh-add)
        #[arg(long, short)]
        lifetime: Option<u64>,
    },
    Remove {
        /// Specific key name; if omitted, removes all managed keys
        name: Option<String>,
    },
    /// Remove all keys from ssh-agent
    Clear, /// TODO print warning
    /// List keys currently loaded in ssh-agent
    List,
    /// Check if ssh-agent is running (depends on implementation)
    Status,
}