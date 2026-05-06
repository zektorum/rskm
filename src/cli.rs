use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rskm", version = env!("CARGO_PKG_VERSION"), about = "Rust SSH Key Manager")]
struct Rskm {
    #[command(subcommand)]
    command: Commands,    
}

#[derive(Subcommand)]
enum Commands {
    Create {
        key_name: String,
        #[arg(short = 't', default_value = "ed25519")]
        key_type: String,
    },
    Delete {
        key_name: String,
    },
    List,
}