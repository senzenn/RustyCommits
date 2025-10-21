use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommitError {
    #[error("Failed to generate commit message using OpenRouter API")]
    OpenRouterApiFail,
    #[error("Invalid response from OpenRouter API")]
    InvalidResponse,
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
}
