use clap::{Parser, Subcommand};

use crate::{config::RskmSettings, errors::RskmError};

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
        #[arg(short = 't')]
        key_type: String,
    },
    Delete {
        key_name: String,
    },
    List,
}

pub fn run() -> Result<(), RskmError> {
    let cli = Rskm::parse();
    let settings = RskmSettings::new()?;

    if !settings.is_initialized() {
        settings.init()?;
    }

    match cli.command {
        Commands::Create { key_name, key_type } => {
            let key_path = settings.keys_dir().join(&key_name);

            if key_path.exists() {
                return Err(RskmError::KeyExists(key_name));
            }

            let status = std::process::Command::new("ssh-keygen")
                .args(["-t", &key_type, "-f", key_path.to_str().unwrap(), "-N", ""])
                .status()
                .map_err(|_| RskmError::KeygenFailed)?;

            if !status.success() {
                return Err(RskmError::KeygenFailed);
            }

            println!("Created key '{key_name}' ({key_type})");
        }

        Commands::Delete { key_name } => {
            let key_path = settings.keys_dir().join(&key_name);

            if !key_path.exists() {
                return Err(RskmError::KeyNotFound(key_name));
            }

            std::fs::remove_file(&key_path)?;
            std::fs::remove_file(key_path.with_extension("pub")).ok();
            println!("Deleted key '{key_name}'");
        }

        Commands::List => {
            let mut keys: Vec<String> = std::fs::read_dir(settings.keys_dir())?
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|name| !name.ends_with(".pub"))
                .collect();

            keys.sort();

            if keys.is_empty() {
                println!("No keys found.");
            } else {
                keys.iter().for_each(|k| println!("{k}"));
            }
        }
    }

    Ok(())
}
