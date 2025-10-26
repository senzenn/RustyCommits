use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rusty-commit")]
#[command(about = "AI-powered commit message generation")]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Generate commit message without committing
    #[arg(short, long)]
    pub dry_run: bool,

    /// Use interactive mode for editing messages
    #[arg(short, long)]
    pub interactive: bool,

    /// Force commit without confirmation
    #[arg(short, long)]
    pub force: bool,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// AI model to use (default: gpt-3.5-turbo)
    #[arg(long, default_value = "openai/gpt-3.5-turbo")]
    pub model: String,

    /// API key (overrides config file)
    #[arg(long)]
    pub api_key: Option<String>,

    /// Config file path
    #[arg(long)]
    pub config: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate and commit with AI-generated message
    Commit {
        /// Custom commit message (overrides AI generation)
        message: Option<String>,
    },
    /// Generate commit message only
    Generate,
    /// Install as git pre-commit hook
    InstallHook,
    /// Configure API keys and settings
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set API key
    SetApiKey,
    /// Set default model
    SetModel {
        model: String,
    },
    /// Show current configuration
    Show,
}
