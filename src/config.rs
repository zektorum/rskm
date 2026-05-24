use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::{errors::RskmError, keys::key_types::KeyTypes};

#[derive(Serialize, Deserialize)]
pub struct RskmSettings {
    #[serde(skip)]
    rskm_home: PathBuf,
    default_key_type: String,
}

const DEFAULT_KEY_TYPE: &str = "ed25519";
const CONFIG_FILE_NAME: &str = "rskm.toml";
const KEYS_DIR_NAME: &str = "keys";

impl RskmSettings {
    pub fn new() -> Result<Self, RskmError> {
        let rskm_home = if let Ok(path) = std::env::var("RSKM_HOME") {
            PathBuf::from(path)
        } else {
            dirs::home_dir()
                .ok_or(RskmError::HomeDirectoryNotFound)?
                .join(".rskm")
        };

        let config_file = rskm_home.join(CONFIG_FILE_NAME);

        let settings = if config_file.exists() {
            let content = std::fs::read_to_string(config_file)?;
            let from_file: RskmSettings = toml::from_str(&content)?;
            Self {
                rskm_home,
                ..from_file
            }
        } else {
            Self {
                rskm_home,
                default_key_type: DEFAULT_KEY_TYPE.to_string(),
            }
        };

        Ok(settings)
    }

    pub fn keys_dir(&self) -> PathBuf {
        self.rskm_home.join(KEYS_DIR_NAME)
    }

    pub fn config_file(&self) -> PathBuf {
        self.rskm_home.join(CONFIG_FILE_NAME)
    }

    pub fn default_key_type(&self) -> &str {
        &self.default_key_type
    }

    pub fn validate(&self) -> Result<(), RskmError> {
        self.default_key_type.parse::<KeyTypes>()?;
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.rskm_home.exists() && self.keys_dir().exists()
    }

    pub fn init(&self) -> Result<(), RskmError> {
        std::fs::create_dir_all(self.keys_dir())?;
        let content = toml::to_string(&self)?;
        std::fs::write(self.config_file(), content)?;
        Ok(())
    }
}
