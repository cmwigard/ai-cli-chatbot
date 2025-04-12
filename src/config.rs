use std::fs;
use crate::error::AppError;

pub fn get_config_value(key: &str) -> Result<String, AppError> {
    let config_content = fs::read_to_string("config.toml")
        .map_err(|e| AppError::IoError(e))?;
    let config: toml::Value = toml::de::from_str(&config_content)
        .map_err(|e| AppError::TomlParseError(e))?;

    config["settings"][key]
        .as_str()
        .ok_or_else(|| AppError::ConfigError(format!("Key '{}' not found in config", key)))
        .map(|s| s.to_string())
}
