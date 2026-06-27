use clap::{Parser, Subcommand};
use prompted::input;

use crate::{config::RskmSettings, errors::RskmError, keys::key_types::KeyTypes};

#[derive(Parser)]
#[command(name = "rskm", version = env!("CARGO_PKG_VERSION"), about = "Rust SSH Key Manager")]
struct Rskm {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Init,
    Create {
        key_name: String,
        #[arg(short = 't')]
        key_type: Option<String>,
    },
    Rename {
        key_name: String,
        new_name: String,
    },
    Delete {
        key_name: String,
    },
    List,
    Show {
        key_name: String,
    },
    Add {
        key_name: String,
    },
    Remove {
        key_name: String,
    },
    Destroy {
        #[arg(short = 'y', long = "yes")]
        yes: bool,
    },
}

pub(crate) fn execute(command: Commands, settings: &RskmSettings) -> Result<(), RskmError> {
    if !matches!(command, Commands::Init | Commands::Destroy { .. }) && !settings.is_initialized() {
        return Err(RskmError::NotInitialized);
    }

    match command {
        Commands::Init => {
            if settings.is_initialized() {
                println!("Already initialized");
            } else {
                settings.init()?;
                println!("Initialized RSKM_HOME");
            }
        }

        Commands::Create { key_name, key_type } => {
            let key_path = settings.keys_dir().join(&key_name);

            if key_path.exists() {
                return Err(RskmError::KeyExists(key_name));
            }

            let key_type = key_type.unwrap_or_else(|| settings.default_key_type().to_string());
            key_type.parse::<KeyTypes>()?;

            let key_path = key_path
                .to_str()
                .ok_or_else(|| RskmError::InvalidPath(format!("invalid path: {:?}", key_path)))?;

            let status = std::process::Command::new("ssh-keygen")
                .args(["-t", &key_type, "-f", key_path, "-N", ""])
                .stdout(std::process::Stdio::null())
                .status()
                .map_err(|_| RskmError::KeygenFailed)?;

            if !status.success() {
                return Err(RskmError::KeygenFailed);
            }

            println!("Created key '{key_name}' ({key_type})");
        }

        Commands::Rename { key_name, new_name } => {
            let key_path = settings.keys_dir().join(&key_name);

            if !key_path.exists() {
                return Err(RskmError::KeyNotFound(key_name));
            }

            let new_path = settings.keys_dir().join(&new_name);

            std::fs::rename(&key_path, &new_path)?;

            let pub_path = key_path.with_extension("pub");
            if pub_path.exists() {
                std::fs::rename(pub_path, new_path.with_extension("pub"))?;
            }

            println!("Renamed key '{key_name}' to '{new_name}'");
        }

        Commands::Delete { key_name } => {
            let key_path = settings.keys_dir().join(&key_name);

            if !key_path.exists() {
                return Err(RskmError::KeyNotFound(key_name));
            }

            std::fs::remove_file(&key_path)?;

            let pub_key_path = key_path.with_extension("pub");
            if pub_key_path.exists() {
                std::fs::remove_file(pub_key_path)?;
            }

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
                return Err(RskmError::AgentOperationFailed(format!(
                    "failed to add '{key_name}'"
                )));
            }

            println!("Added key '{key_name}' to agent.");
        }

        Commands::Remove { key_name } => {
            let key_path = settings.keys_dir().join(&key_name);

            if !key_path.exists() {
                return Err(RskmError::KeyNotFound(key_name));
            }

            let key_path = key_path
                .to_str()
                .ok_or_else(|| RskmError::InvalidPath(format!("invalid path: {:?}", key_path)))?;

            let status = std::process::Command::new("ssh-add")
                .args(["-d", key_path])
                .status()
                .map_err(|_| RskmError::AgentNotRunning)?;

            if !status.success() {
                return Err(RskmError::AgentOperationFailed(format!(
                    "failed to remove '{key_name}'"
                )));
            }

            println!("Removed key '{key_name}' from agent.");
        }

        Commands::Destroy { yes } => {
            if !settings.is_initialized() {
                println!("Nothing to do: RSKM_HOME dir is not initialized");
                return Ok(());
            }

            if !yes {
                let answer =
                    input!("Do you really want to delete RSKM_HOME? This cannot be undone! [y/N] ");
                if answer.trim().to_lowercase() != "y" {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            std::fs::remove_dir_all(settings.rskm_home())?;
            println!("Destroyed RSKM_HOME.");
        }
    }

    Ok(())
}

pub fn run() -> Result<(), RskmError> {
    let cli = Rskm::parse();
    let settings = RskmSettings::new()?;
    execute(cli.command, &settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_settings(tmp: &tempfile::TempDir) -> RskmSettings {
        unsafe { std::env::set_var("RSKM_HOME", tmp.path()) };
        let settings = RskmSettings::new().unwrap();
        settings.init().unwrap();
        settings
    }

    // ── init ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_init_when_not_initialized() {
        let tmp = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("RSKM_HOME", tmp.path()) };
        let settings = RskmSettings::new().unwrap();

        assert!(!settings.is_initialized());
        execute(Commands::Init, &settings).unwrap();
        assert!(settings.is_initialized());
    }

    #[test]
    fn test_init_when_already_initialized() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);

        // second init should be a no-op, not an error
        execute(Commands::Init, &settings).unwrap();
        assert!(settings.is_initialized());
    }

    // ── not initialized guard ─────────────────────────────────────────────────

    #[test]
    fn test_commands_fail_when_not_initialized() {
        let tmp = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("RSKM_HOME", tmp.path()) };
        let settings = RskmSettings::new().unwrap();

        let cmds = vec![
            Commands::Create { key_name: "k".into(), key_type: None },
            Commands::Rename { key_name: "k".into(), new_name: "k2".into() },
            Commands::Delete { key_name: "k".into() },
            Commands::List,
            Commands::Show { key_name: "k".into() },
            Commands::Add { key_name: "k".into() },
            Commands::Remove { key_name: "k".into() },
        ];

        for cmd in cmds {
            let err = execute(cmd, &settings).unwrap_err();
            assert!(
                matches!(err, RskmError::NotInitialized),
                "expected NotInitialized"
            );
        }
    }

    // ── create ────────────────────────────────────────────────────────────────

    #[test]
    fn test_create_key_already_exists() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);
        fs::write(settings.keys_dir().join("mykey"), "").unwrap();

        let err = execute(
            Commands::Create { key_name: "mykey".into(), key_type: None },
            &settings,
        )
        .unwrap_err();

        assert!(matches!(err, RskmError::KeyExists(n) if n == "mykey"));
    }

    #[test]
    fn test_create_unknown_key_type() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);

        let err = execute(
            Commands::Create { key_name: "mykey".into(), key_type: Some("dsa".into()) },
            &settings,
        )
        .unwrap_err();

        assert!(matches!(err, RskmError::UnknownKeyType(t) if t == "dsa"));
    }

    #[test]
    fn test_create_key_with_ssh_keygen() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);

        execute(
            Commands::Create { key_name: "testkey".into(), key_type: Some("ed25519".into()) },
            &settings,
        )
        .unwrap();

        assert!(settings.keys_dir().join("testkey").exists());
        assert!(settings.keys_dir().join("testkey.pub").exists());
    }

    // ── rename ────────────────────────────────────────────────────────────────

    #[test]
    fn test_rename_key_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);

        let err = execute(
            Commands::Rename { key_name: "ghost".into(), new_name: "other".into() },
            &settings,
        )
        .unwrap_err();

        assert!(matches!(err, RskmError::KeyNotFound(n) if n == "ghost"));
    }

    #[test]
    fn test_rename_key_without_pub() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);
        fs::write(settings.keys_dir().join("oldkey"), "private").unwrap();

        execute(
            Commands::Rename { key_name: "oldkey".into(), new_name: "newkey".into() },
            &settings,
        )
        .unwrap();

        assert!(!settings.keys_dir().join("oldkey").exists());
        assert!(settings.keys_dir().join("newkey").exists());
    }

    #[test]
    fn test_rename_key_with_pub() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);
        fs::write(settings.keys_dir().join("oldkey"), "private").unwrap();
        fs::write(settings.keys_dir().join("oldkey.pub"), "public").unwrap();

        execute(
            Commands::Rename { key_name: "oldkey".into(), new_name: "newkey".into() },
            &settings,
        )
        .unwrap();

        assert!(!settings.keys_dir().join("oldkey").exists());
        assert!(!settings.keys_dir().join("oldkey.pub").exists());
        assert!(settings.keys_dir().join("newkey").exists());
        assert!(settings.keys_dir().join("newkey.pub").exists());
    }

    // ── delete ────────────────────────────────────────────────────────────────

    #[test]
    fn test_delete_key_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);

        let err = execute(Commands::Delete { key_name: "ghost".into() }, &settings).unwrap_err();
        assert!(matches!(err, RskmError::KeyNotFound(n) if n == "ghost"));
    }

    #[test]
    fn test_delete_key_without_pub() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);
        fs::write(settings.keys_dir().join("mykey"), "private").unwrap();

        execute(Commands::Delete { key_name: "mykey".into() }, &settings).unwrap();

        assert!(!settings.keys_dir().join("mykey").exists());
    }

    #[test]
    fn test_delete_key_with_pub() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);
        fs::write(settings.keys_dir().join("mykey"), "private").unwrap();
        fs::write(settings.keys_dir().join("mykey.pub"), "public").unwrap();

        execute(Commands::Delete { key_name: "mykey".into() }, &settings).unwrap();

        assert!(!settings.keys_dir().join("mykey").exists());
        assert!(!settings.keys_dir().join("mykey.pub").exists());
    }

    // ── list ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_list_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);

        // should not error on empty keys dir
        execute(Commands::List, &settings).unwrap();
    }

    #[test]
    fn test_list_excludes_pub_files() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);
        fs::write(settings.keys_dir().join("alpha"), "").unwrap();
        fs::write(settings.keys_dir().join("alpha.pub"), "").unwrap();
        fs::write(settings.keys_dir().join("beta"), "").unwrap();

        // No error — output goes to stdout; we just assert no error is returned
        execute(Commands::List, &settings).unwrap();
    }

    // ── show ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_show_key_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);

        let err = execute(Commands::Show { key_name: "ghost".into() }, &settings).unwrap_err();
        assert!(matches!(err, RskmError::KeyNotFound(n) if n == "ghost"));
    }

    #[test]
    fn test_show_key_success() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);
        fs::write(settings.keys_dir().join("mykey.pub"), "ssh-ed25519 AAAA...").unwrap();

        execute(Commands::Show { key_name: "mykey".into() }, &settings).unwrap();
    }

    // ── add / remove (key not found) ─────────────────────────────────────────

    #[test]
    fn test_add_key_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);

        let err = execute(Commands::Add { key_name: "ghost".into() }, &settings).unwrap_err();
        assert!(matches!(err, RskmError::KeyNotFound(n) if n == "ghost"));
    }

    #[test]
    fn test_remove_key_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);

        let err = execute(Commands::Remove { key_name: "ghost".into() }, &settings).unwrap_err();
        assert!(matches!(err, RskmError::KeyNotFound(n) if n == "ghost"));
    }

    // ── destroy ───────────────────────────────────────────────────────────────

    #[test]
    fn test_destroy_when_not_initialized() {
        let tmp = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("RSKM_HOME", tmp.path()) };
        let settings = RskmSettings::new().unwrap();

        // should return Ok without doing anything
        execute(Commands::Destroy { yes: true }, &settings).unwrap();
    }

    #[test]
    fn test_destroy_with_yes_flag() {
        let tmp = tempfile::tempdir().unwrap();
        let settings = setup_settings(&tmp);

        execute(Commands::Destroy { yes: true }, &settings).unwrap();

        assert!(!settings.rskm_home().exists());
    }
}
