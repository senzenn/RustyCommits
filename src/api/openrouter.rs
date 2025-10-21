use git2::{ErrorClass, Repository};
use reqwest::Client;
use serde_json::Value;

use crate::errors;

pub async fn generate_commit_message_openrouter(
    repo: &Repository,
    files: &[String],
    api_key: &str,
) -> Result<String, errors::CommitError> {
    // ctx for the api request for commit message
    let prompt = format!(
        "Generate  a  commit message for changes in files:  {} ",
        files.join(", ")
    );
    // api response capture

    let response = Client::new()
        .post("https://api.openrouter.ai/v1/commit")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json!({"prompt": prompt}))
        .send()
        .await?;
    // error handler
    if !response.status().is_success() {
        return Err(errors::CommitError::OpenRouterApiFail);
    }

    // parse messages 

    let response_json : Value  = response.json().await()?; // value parse any valid data into json
    // data should be valid json type

    let commit_message = response_json["messages"].as_str().ok_or(errors::CommitError::InvalidResponse)?;
    Ok(commit_message.to_string())
}
