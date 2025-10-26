use thiserror::Error;
use reqwest::StatusCode;

#[derive(Debug, Error)]
pub enum CommitError {
    #[error("OpenRouter API failed with status {status}: {message}")]
    OpenRouterApiFail {
        status: StatusCode,
        message: String,
    },
    
    #[error("Invalid response from OpenRouter API")]
    InvalidResponse,
    
    #[error("Git repository error: {0}")]
    GitError(#[from] git2::Error),
    
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("No changes to commit")]
    NoChanges,
    
    #[error("Git operation failed: {0}")]
    GitOperationError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Config file not found: {0}")]
    ConfigNotFound(String),
    
    #[error("Interactive prompt error: {0}")]
    DialoguerError(#[from] dialoguer::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
}

pub type Result<T> = std::result::Result<T, CommitError>;