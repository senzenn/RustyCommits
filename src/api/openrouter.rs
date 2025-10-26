

use git2::Repository;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::errors::{CommitError, Result};

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

pub async fn generate_commit_message_openrouter(
    _repo: &Repository,
    files: &[String],
    diff_content: &str,
    api_key: &str,
    model: &str,
) -> Result<String> {
    let prompt = format_commit_prompt(files, diff_content);
    
    let request = ChatRequest {
        model: model.to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        max_tokens: 150,
        temperature: 0.7,
    };

    let response = Client::new()
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let message = response.text().await.unwrap_or_default();
        return Err(CommitError::OpenRouterApiFail { status, message });
    }

    let response_data: ChatResponse = response.json().await?;
    
    response_data.choices
        .first()
        .and_then(|choice| Some(choice.message.content.trim().to_string()))
        .ok_or(CommitError::InvalidResponse)
}

fn format_commit_prompt(files: &[String], diff_content: &str) -> String {
    format!(
        r#"Generate a concise, meaningful commit message for the following changes:

Files changed: {}

Diff:
{}

Please provide only the commit message, no explanations or quotes. Follow conventional commit format if applicable."#,
        files.join(", "),
        diff_content
    )
}
