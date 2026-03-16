use std::path::PathBuf;
use std::str::FromStr;

pub enum KeyTypes {
    ED25519,
    ED25519SK,
    ECDSA,
    ECDSASK,
    XMSS,
    RSA,
}

pub enum KeySizes {
    // RSA, ECDSA
    Bits(u32),
    // XMSS
    XmssParameterSet(String)
}

impl FromStr for KeyTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ED25519"   => Ok(KeyTypes::ED25519),
            "ED25519SK" => Ok(KeyTypes::ED25519SK),
            "ECDSA"     => Ok(KeyTypes::ECDSA),
            "ECDSASK"   => Ok(KeyTypes::ECDSASK),
            "XMSS"      => Ok(KeyTypes::XMSS),
            "RSA"       => Ok(KeyTypes::RSA),
            _           => Err(format!("Unknown key type: {}", s)),
        }
    }
}

impl KeyTypes {
    fn ssh_keygen_args(&self) -> Vec<&str> {
        match self {
            KeyTypes::ED25519   => vec!["-t", "ed25519"],
            KeyTypes::ED25519SK => vec!["-t", "ed25519-sk"],
            KeyTypes::ECDSA     => vec!["-t", "ecdsa", "-b", "521"],
            KeyTypes::ECDSASK   => vec!["-t", "ecdsa-sk"],
            KeyTypes::XMSS      => vec!["-t", "xmss"],
            KeyTypes::RSA       => vec!["-t", "rsa", "-b", "4096"],
        }
    }
}

pub struct KeyGenOptions {
    key_name: String,
    key_type: KeyTypes,
    key_size: Option<KeySizes>,
    comment: Option<String>,
    passphrase: Option<String>,
    no_passphrase: bool,
    use_ssh_dir: bool,
    output_dir: Option<PathBuf>
}