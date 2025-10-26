mod cli;
mod api;
mod config;
mod errors;
mod utils;
mod interactive;

use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rusty_commit::api::openrouter::generate_commit_message_openrouter;
use rusty_commit::utils::git::{get_git_changes, filter_diff_content, perform_git_commit, generate_fallback_message};
use rusty_commit::config::{load_config, save_config};
use rusty_commit::interactive::{prompt_commit_message, confirm_commit, prompt_api_key};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    
    // Load configuration
    let mut config = load_config(cli.config.as_deref())?;
    
    // Override config with CLI args
    config.default_model = cli.model;
    if let Some(api_key) = &cli.api_key {
        config.api_key = Some(api_key.clone());
    }
    
    // Get API key
    let api_key = if let Some(key) = config.api_key.clone() {
        key
    } else {
        println!("🔑 API key not found in config.");
        let key = prompt_api_key()?;
        
        // Save to config
        config.api_key = Some(key.clone());
        if let Err(e) = save_config(&config, cli.config.as_deref()) {
            println!("⚠️  Warning: Could not save API key to config: {}", e);
        }
        
        key
    };
    
    // Open git repository
    let repo = git2::Repository::open(".")?;
    
    // Get git changes
    if cli.verbose {
        println!("🔍 Analyzing git changes...");
    }
    
    let changes = get_git_changes(&repo)?;
    
    if changes.staged_files.is_empty() && changes.unstaged_files.is_empty() {
        println!("{}", "📭 No changes to commit.".yellow());
        return Ok(());
    }
    
    // Determine which diff to use
    let (diff_content, files) = if !changes.staged_diff.is_empty() {
        (changes.staged_diff, changes.staged_files)
    } else {
        (changes.unstaged_diff, changes.unstaged_files)
    };
    
    let filtered_diff = filter_diff_content(&diff_content, config.max_diff_lines);
    
    if cli.verbose {
        println!("📁 Files changed: {}", files.join(", "));
        println!("📊 Diff size: {} lines (filtered to {})", 
                diff_content.lines().count(), 
                filtered_diff.lines().count());
    }
    
    // Handle different commands
    match &cli.command {
        Some(cli::Commands::Generate) | None if cli.dry_run => {
            // Generate message only
            let pb = ProgressBar::new_spinner();
            pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}").unwrap());
            pb.set_message("🤖 Generating commit message...");
            pb.enable_steady_tick(std::time::Duration::from_millis(100));
            
            let commit_message = match generate_commit_message_openrouter(
                &repo, 
                &files, 
                &filtered_diff, 
                &api_key, 
                &config.default_model
            ).await {
                Ok(message) => message,
                Err(e) => {
                    pb.finish_with_message("❌ API failed, using fallback");
                    println!("⚠️  API failed: {}. Using intelligent fallback...", e);
                    generate_fallback_message(&files, &filtered_diff)
                }
            };
            
            pb.finish_with_message("✅ Message generated");
            println!("📝 Generated message: {}", commit_message.green());
        }
        
        Some(cli::Commands::Commit { message }) => {
            let commit_message = if let Some(msg) = message {
                msg.clone()
            } else {
                // Generate message
                let pb = ProgressBar::new_spinner();
                pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}").unwrap());
                pb.set_message("🤖 Generating commit message...");
                pb.enable_steady_tick(std::time::Duration::from_millis(100));
                
                let msg = match generate_commit_message_openrouter(
                    &repo, 
                    &files, 
                    &filtered_diff, 
                    &api_key, 
                    &config.default_model
                ).await {
                    Ok(message) => message,
                    Err(e) => {
                        pb.finish_with_message("❌ API failed, using fallback");
                        println!("⚠️  API failed: {}. Using intelligent fallback...", e);
                        generate_fallback_message(&files, &filtered_diff)
                    }
                };
                pb.finish_with_message("✅ Message generated");
                msg
            };
            
            // Interactive mode
            let final_message = if cli.interactive {
                prompt_commit_message(&commit_message)?
            } else {
                commit_message
            };
            
            // Confirm before committing
            if !cli.force && !confirm_commit(&final_message)? {
                println!("❌ Commit cancelled.");
                return Ok(());
            }
            
            // Perform commit
            let pb = ProgressBar::new_spinner();
            pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}").unwrap());
            pb.set_message("💾 Committing changes...");
            pb.enable_steady_tick(std::time::Duration::from_millis(100));
            
            match perform_git_commit(&repo, &final_message) {
                Ok(_) => {
                    pb.finish_with_message("✅ Commit successful");
                    println!("🎉 Committed with message: {}", final_message.green());
                }
                Err(e) => {
                    pb.finish_with_message("❌ Commit failed");
                    return Err(e.into());
                }
            }
        }
        
        Some(cli::Commands::Config { config_command }) => {
            match config_command {
                cli::ConfigCommands::SetApiKey => {
                    let key = prompt_api_key()?;
                    config.api_key = Some(key);
                    save_config(&config, cli.config.as_deref())?;
                    println!("✅ API key saved to config");
                }
                cli::ConfigCommands::SetModel { model } => {
                    config.default_model = model.clone();
                    save_config(&config, cli.config.as_deref())?;
                    println!("✅ Default model set to: {}", model);
                }
                cli::ConfigCommands::Show => {
                    println!("📋 Current configuration:");
                    println!("  API Key: {}", if config.api_key.is_some() { "Set" } else { "Not set" });
                    println!("  Default Model: {}", config.default_model);
                    println!("  Max Diff Lines: {}", config.max_diff_lines);
                    println!("  Temperature: {}", config.temperature);
                    println!("  Max Tokens: {}", config.max_tokens);
                }
            }
        }
        
        Some(cli::Commands::InstallHook) => {
            println!("🚧 Hook installation not implemented yet");
            // TODO: Implement pre-commit hook installation
        }
        
        _ => {
            println!("Use --help for usage information");
        }
    }
    
    Ok(())
}
