use colored::Colorize;
use git2::Repository;
use rusty_commit::api::openrouter::generate_commit_message_openrouter;
use rusty_commit::utils::env_variable::prompt_and_save_env_variable;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // Option return :)

    let api_key = env::var("OPEN_ROUTER_API").unwrap_or_else(|_| {
        prompt_and_save_env_variable("OPEN_ROUTER_API", "Enter your OPEN Router API key: ").unwrap()
    }); // prompt for  open router api

    let repo = Repository::open(".")?;
    let statuses = repo.statuses(None)?;

    // collecting the modifications  across files

    let mut changed_files = Vec::new();
    for status in statuses.iter() {
        if status.status().is_wt_modified() || status.status().is_wt_new() {
            if let Some(path) = status.path() {
                changed_files.push(path.to_string());
            }
        }
    }

    // BUG: file  is not opneing ?????????
    // FIXME: try changing diff instead of git2 or manual crate  >>>

    if changed_files.is_empty() {
        println!("NO changes to commit . Exiting ");
        return Ok(());
    }

    let commit_message =
    // functionn call  goes here  -> 
        match generate_commit_message_openrouter(&repo, &changed_files, &api_key).await {
            Ok(message) => message,
            Err(_) => {
                println!("{}", "Open Router API failed using fallback commit message✌️😏".blue());
                //FIXME: use algo to get the fallback message and change this static message into meaning mearningful
                "Fallback commit message".to_string()
            }
        };
    // final message

    println!("⚡️Generated commit message : {}", commit_message.green());
    Ok(())
}
