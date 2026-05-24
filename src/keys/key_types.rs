use std::str::FromStr;

pub enum KeyTypes {
    Ed25519,
    Ecdsa,
    Xmss,
    Rsa,
}

impl FromStr for KeyTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ed25519"   => Ok(KeyTypes::Ed25519),
            "ecdsa"     => Ok(KeyTypes::Ecdsa),
            "xmss"      => Ok(KeyTypes::Xmss), //TODO: make feature-toggle for xmss support
            "rsa"       => Ok(KeyTypes::Rsa),
            _           => Err(format!("Unknown key type: {}", s)),
        }
    }
}