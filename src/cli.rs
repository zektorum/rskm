use clap::{Parser, Subcommand};

use crate::{config::RskmSettings, errors::RskmError, keys::key_types::KeyTypes};

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
        key_type: Option<String>,
    },
    Delete {
        key_name: String,
    },
    List {
        #[arg(long)]
        loaded: bool,
    },
    Show {
        key_name: String,
    },
    Add {
        key_name: String,
    },
    Remove {
        key_name: String,
    },
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

            let key_type = key_type.unwrap_or_else(|| settings.default_key_type().to_string());
            key_type.parse::<KeyTypes>()?;

            let key_path_str = key_path
                .to_str()
                .ok_or_else(|| RskmError::InvalidPath(format!("invalid path: {:?}", key_path)))?;

            let status = std::process::Command::new("ssh-keygen")
                .args(["-t", &key_type, "-f", key_path_str, "-N", ""])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
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
            std::fs::remove_file(key_path.with_extension("pub")).ok(); // FIXME
            println!("Deleted key '{key_name}'");
        }

        Commands::List { loaded } => { // TODO: refactor me
            if loaded {
                let output = std::process::Command::new("ssh-add")
                    .arg("-L")
                    .output()
                    .map_err(|_| RskmError::AgentNotRunning)?;

                if output.status.code() == Some(2) {
                    return Err(RskmError::AgentNotRunning);
                }

                let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                let agent_keys: Vec<(String, String)> = stdout
                    .lines()
                    .filter_map(|line| {
                        let mut parts = line.splitn(3, ' ');
                        Some((parts.next()?.to_string(), parts.next()?.to_string()))
                    })
                    .collect();

                let mut found = false;
                for entry in std::fs::read_dir(settings.keys_dir())? {
                    let entry = entry?;
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if !name.ends_with(".pub") {
                        continue;
                    }
                    let content = std::fs::read_to_string(entry.path())?;
                    let mut parts = content.trim().splitn(3, ' ');
                    if let (Some(t), Some(k)) = (parts.next(), parts.next()) {
                        if agent_keys.iter().any(|(at, ak)| at == t && ak == k) {
                            println!("{}", name.trim_end_matches(".pub"));
                            found = true;
                        }
                    }
                }

                if !found {
                    println!("No rskm keys loaded in agent.");
                }
            } else {
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

        Commands::Show { key_name } => {
            let pub_key_path = settings.keys_dir().join(&key_name).with_extension("pub");

            if !pub_key_path.exists() {
                return Err(RskmError::KeyNotFound(key_name));
            }

            let content = std::fs::read_to_string(pub_key_path)?;
            print!("{content}");
        }

        Commands::Add { key_name } => {
            let key_path = settings.keys_dir().join(&key_name);

            if !key_path.exists() {
                return Err(RskmError::KeyNotFound(key_name));
            }

            let status = std::process::Command::new("ssh-add")
                .arg(&key_path)
                .status()
                .map_err(|_| RskmError::AgentNotRunning)?;

            if !status.success() {
                return Err(RskmError::AgentOperationFailed(format!("failed to add '{key_name}'")));
            }

            println!("Added key '{key_name}' to agent.");
        }

        Commands::Remove { key_name } => {
            let key_path = settings.keys_dir().join(&key_name);

            if !key_path.exists() {
                return Err(RskmError::KeyNotFound(key_name));
            }

            let status = std::process::Command::new("ssh-add")
                .args(["-d", key_path.to_str().unwrap()])
                .status()
                .map_err(|_| RskmError::AgentNotRunning)?;

            if !status.success() {
                return Err(RskmError::AgentOperationFailed(format!("failed to remove '{key_name}'")));
            }

            println!("Removed key '{key_name}' from agent.");
        }
    }

    Ok(())
}
