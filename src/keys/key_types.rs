use std::str::FromStr;

use crate::errors::RskmError;

pub enum KeyTypes {
    Ed25519,
    Ecdsa,
    Xmss,
    Rsa,
}

impl FromStr for KeyTypes {
    type Err = RskmError;

    fn from_str(s: &str) -> Result<Self, RskmError> {
        match s {
            "ed25519"   => Ok(KeyTypes::Ed25519),
            "ecdsa"     => Ok(KeyTypes::Ecdsa),
            "xmss"      => Ok(KeyTypes::Xmss), //TODO: make feature-toggle for xmss support
            "rsa"       => Ok(KeyTypes::Rsa),
            _           => Err(RskmError::UnknownKeyType(s.to_string())),
        }
    }
}