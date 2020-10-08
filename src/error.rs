use std::error::Error as StdError;

#[derive(Debug)]
pub(crate) enum Error {
    Io(std::io::Error),
    TomlSer(toml::ser::Error),
    TomlDe(toml::de::Error),
    GroupNotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "IO error: {:?}", error),
            Self::TomlSer(error) => write!(f, "Serialization error: {:?}", error),
            Self::TomlDe(error) => write!(f, "Serialization error: {:?}", error),
            Self::GroupNotFound(error) => write!(f, "Group could not be found: {}", error),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::TomlSer(error) => Some(error),
            Self::TomlDe(error) => Some(error),
            Self::GroupNotFound(_) => None,
        }
    }
}
