use std::str::FromStr;

impl FromStr for KeyTypes {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ED25519" => Ok(KeyTypes::ED25519),
            "ED25519SK" => Ok(KeyTypes::ED25519SK),
            "ECDSA" => Ok(KeyTypes::ECDSA),
            "ECSASK" => Ok(KeyTypes::ECSASK),
            "XMSS" => Ok(KeyTypes::XMSS),
            "RSA" => Ok(KeyTypes::RSA),
            _ => Err(format!("Unknown key type: {}", s))
        }
    }
}

enum KeyTypes {
    ED25519,
    ED25519SK,
    ECDSA,
    ECSASK,
    XMSS,
    RSA
}