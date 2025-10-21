use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommitError {
    #[error("Failed to generate commit message using OpenRouter API")]
    OpenRouterApiFail,
    #[error("Invalid response from OpenRouter API")]
    InvalidResponse,
}
