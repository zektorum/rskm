use std::path::PathBuf;

pub struct RskmSettings {
    pub rskm_home: PathBuf,
    pub default_key_type: String,
}

impl RskmSettings {
    pub fn new() -> Result<Self, Box <dyn std::error::Error>> {
        let rskm_home = dirs::home_dir()
            .ok_or("Unable to find home directory")?
            .join(".rskm");

        Ok(Self {
            rskm_home,
            default_key_type: "ed25519".to_string(),
        })
    }

    pub fn keys_dir(&self) -> PathBuf {
        self.rskm_home.join("keys")
    }
}

// TODO first launch of app should be init