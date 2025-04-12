use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("API request error: {0}")]
    ApiRequestError(String),
    #[error("Error reading configuration file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error parsing TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),
    #[error("Unexpected error occurred: {0}")]
    UnexpectedError(String),
}
