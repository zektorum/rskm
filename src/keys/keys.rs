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
            "ED25519"   => Ok(KeyTypes::ED25519),
            "ECDSA"     => Ok(KeyTypes::ECDSA),
            "XMSS"      => Ok(KeyTypes::XMSS),
            "RSA"       => Ok(KeyTypes::RSA),
            _           => Err(format!("Unknown key type: {}", s)),
        }
    }
}
