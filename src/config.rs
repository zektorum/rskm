use std::path::PathBuf;

pub struct RskmSettings {
    pub rskm_home: PathBuf,
    pub keys_dir: PathBuf,
    pub default_key_type: String,
}

impl Default for RskmSettings {
    fn default() -> Self {
        let rskm_home = dirs::home_dir() //TODO make it possible to change rskm_home dir
            .expect("Unable to find home directory!")
            .join(".rskm"); // TODO create error type

        let keys_dir = rskm_home.join("keys");

        Self {
            rskm_home,
            keys_dir,
            default_key_type: "ed25519".to_string()
        }
    }
}

// TODO first launch of app should be init