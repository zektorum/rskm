use std::{error::Error, path::PathBuf};
use serde::{Deserialize, Serialize};
use crate::errors::RskmError;

#[derive(Serialize, Deserialize)]
pub struct RskmSettings {
    #[serde(skip)]
    pub rskm_home: PathBuf,
    pub default_key_type: String,
}

impl RskmSettings {
    pub fn new() -> Result<Self, RskmError> {
        let rskm_home = if let Ok(path) = std::env::var("RSKM_HOME") {
            PathBuf::from(path)
        } else {
            dirs::home_dir()
                .ok_or(RskmError::HomeDirectoryNotFound)?
                .join(".rskm")
        };

        let config_file = rskm_home.join("rskm.toml");

        let settings = if config_file.exists() {
            let content = std::fs::read_to_string(config_file)?;
            let from_file : RskmSettings = toml::from_str(&content)?;
            Self {
                rskm_home,
                ..from_file
            }
        } else {
            Self {
                rskm_home,
                default_key_type: "ed25519".to_string(),
            }
        };

        Ok(settings)
    }

    pub fn keys_dir(&self) -> PathBuf {
        self.rskm_home.join("keys")
    }

    pub fn config_file(&self) -> PathBuf {
        self.rskm_home.join("rskm.toml")
    }

    pub fn is_initialized(&self) -> bool {
        self.rskm_home.exists() && self.keys_dir().exists()
    }

    pub fn init(&self) -> Result<(), Box<dyn Error>> { // TODO: add RskmErrors here
        std::fs::create_dir_all(self.keys_dir())?;

        if !self.config_file().exists() {
           let content = toml::to_string(&self)?;
           std::fs::write(self.config_file(), content)?;
        }

        Ok(())
    }
}
