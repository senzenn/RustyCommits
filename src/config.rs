use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use dirs;
use crate::errors::{CommitError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub default_model: String,
    pub max_diff_lines: usize,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            default_model: "openai/gpt-3.5-turbo".to_string(),
            max_diff_lines: 1000,
            temperature: 0.7,
            max_tokens: 150,
        }
    }
}

pub fn load_config(config_path: Option<&str>) -> Result<Config> {
    let config_file = find_config_file(config_path)?;

    if config_file.exists() {
        let content = fs::read_to_string(&config_file)?;
        let config: toml::Value = toml::from_str(&content)?;
        let config: Config = config.try_into()?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

pub fn save_config(config: &Config, config_path: Option<&str>) -> Result<()> {
    let config_file = find_config_file(config_path)?;

    // Create directory if it doesn't exist
    if let Some(parent) = config_file.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(config)?;
    fs::write(config_file, content)?;
    Ok(())
}

fn find_config_file(explicit_path: Option<&str>) -> Result<PathBuf> {
    if let Some(path) = explicit_path {
        return Ok(PathBuf::from(path));
    }

    // Check current directory first
    let local_config = PathBuf::from(".rusty-commit.toml");
    if local_config.exists() {
        return Ok(local_config);
    }

    // Check global config
    if let Some(home_dir) = dirs::home_dir() {
        let global_config = home_dir.join(".config").join("rusty-commit").join("config.toml");
        return Ok(global_config);
    }

    // Fallback to local config
    Ok(local_config)
}

impl TryFrom<toml::Value> for Config {
    type Error = CommitError;

    fn try_from(value: toml::Value) -> Result<Self> {
        let table = value.as_table().ok_or_else(|| CommitError::ConfigError("Invalid config format".to_string()))?;

        Ok(Config {
            api_key: table.get("api_key").and_then(|v| v.as_str()).map(|s| s.to_string()),
            default_model: table.get("default_model").and_then(|v| v.as_str()).unwrap_or("openai/gpt-3.5-turbo").to_string(),
            max_diff_lines: table.get("max_diff_lines").and_then(|v| v.as_integer()).unwrap_or(1000) as usize,
            temperature: table.get("temperature").and_then(|v| v.as_float()).unwrap_or(0.7) as f32,
            max_tokens: table.get("max_tokens").and_then(|v| v.as_integer()).unwrap_or(150) as u32,
        })
    }
}