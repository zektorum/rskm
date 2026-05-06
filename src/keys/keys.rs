use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use dirs::home_dir;

pub enum KeyTypes {
    ED25519,
    ECDSA,
    XMSS,
    RSA,
}

impl FromStr for KeyTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ED25519"   => Ok(KeyTypes::ED25519),
            "ECDSA"     => Ok(KeyTypes::ECDSA),
            "XMSS"      => Ok(KeyTypes::XMSS),
            "RSA"       => Ok(KeyTypes::RSA),
            _           => Err(format!("Unknown key type: {}", s)),
        }
    }
}

impl KeyTypes {
    // Default ssh-keygen args for given key type
    fn ssh_keygen_args(&self) -> Vec<&str> {
        match self {
            KeyTypes::ED25519   => vec!["-t", "ed25519"],
            KeyTypes::ECDSA     => vec!["-t", "ecdsa", "-b", "521"],
            KeyTypes::XMSS      => vec!["-t", "xmss"],
            KeyTypes::RSA       => vec!["-t", "rsa", "-b", "4096"],
        }
    }
}

pub struct KeyGenSettings {
    key_type: KeyTypes,
    no_passphrase: bool,
    output_dir: PathBuf
}

impl Default for KeyGenSettings {
    fn default() -> Self {
        let output_dir = home_dir()
            .unwrap()
            .join(".rskm");
        KeyGenSettings {  
            key_type: KeyTypes::ED25519,
            no_passphrase: true,
            output_dir
        }
    }
}