#[derive(Debug)]
pub(crate) enum Error {
    Io(std::io::Error),
    TomlSer(toml::ser::Error),
    TomlDe(toml::de::Error),
    GroupNotFound(String),
}
