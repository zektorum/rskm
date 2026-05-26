use core::fmt;
use std::io;

#[derive(Debug)]
pub enum RskmError {
    NotInitialized,

    KeyExists(String),
    KeyNotFound(String),

    KeygenFailed,
    UnknownKeyType(String),

    HostExists(String),
    HostNotFound(String),

    ConfigNotFound(String),
    ConfigParseError(String),
    ConfigWriteError(String),

    AgentNotRunning,
    AgentOperationFailed(String),

    Io(io::Error),
    HomeDirectoryNotFound,
    InvalidInput(String),
    InvalidPath(String),
}

impl fmt::Display for RskmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "RSKM_HOME not initialized. Run 'rskm init' first"),

            Self::KeyExists(name) => write!(f, "Key '{}' already exists", name),
            Self::KeyNotFound(name) => write!(f, "Key '{}' not found", name),
            
            Self::KeygenFailed => write!(f, "ssh-keygen failed"), //FIXME
            Self::UnknownKeyType(key_type) => write!(f, "unknown key type: '{}', must be one of: ed25519, ecdsa, xmss, rsa", key_type),

            Self::HostExists(name) => write!(f, "Host '{}' already exists", name),
            Self::HostNotFound(name) => write!(f, "Host '{}' not found", name),

            Self::ConfigNotFound(path) => write!(f, "Config not found: {}", path),
            Self::ConfigParseError(msg) => write!(f, "Config parse error: {}", msg),
            Self::ConfigWriteError(msg) => write!(f, "Failed to write config: {}", msg),

            Self::AgentNotRunning => write!(f, "ssh-agent is not running"),
            Self::AgentOperationFailed(msg) => write!(f, "ssh-agent error: {}", msg),

            Self::Io(err) => write!(f, "IO error: {}", err),
            Self::HomeDirectoryNotFound => write!(f, "Could not determine home directory"),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::InvalidPath(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for RskmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,                  
        }
    }
}

impl From<io::Error> for RskmError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<toml::de::Error> for RskmError {
    fn from(err: toml::de::Error) -> Self {
        Self::ConfigParseError(err.to_string())
    }
}

impl From<toml::ser::Error> for RskmError {
    fn from(err: toml::ser::Error) -> Self {
        Self::ConfigWriteError(err.to_string())
    }
}