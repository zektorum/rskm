use std::str::FromStr;

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
            "ed25519"   => Ok(KeyTypes::ED25519),
            "ecdsa"     => Ok(KeyTypes::ECDSA),
            "xmss"      => Ok(KeyTypes::XMSS),
            "rsa"       => Ok(KeyTypes::RSA),
            _           => Err(format!("Unknown key type: {}", s)),
        }
    }
}