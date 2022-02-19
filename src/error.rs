#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse config file: {0}")]
    ConfigSerialization(#[from] toml::de::Error),
    #[error("File read error: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
