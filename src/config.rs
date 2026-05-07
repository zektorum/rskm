use std::path::PathBuf;

use crate::errors::RskmError;

pub struct RskmSettings {
    pub rskm_home: PathBuf,
    pub default_key_type: String,
}

impl RskmSettings {
    pub fn new() -> Result<Self, RskmError> {
        let rskm_home = dirs::home_dir()
            .ok_or(RskmError::HomeDirectoryNotFound)?
            .join(".rskm");

        Ok(Self {
            rskm_home,
            default_key_type: "ed25519".to_string(),
        })
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
}

// TODO first launch of app should be init