use git2::{ErrorClass, Repository};
use reqwest::Client;

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

    let response = Client::new()
        .post("https://api.openrouter.ai/v1/commit")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json!({"prompt": prompt}))
        .send()
        .await?;
}
