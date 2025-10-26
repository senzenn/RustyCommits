use dialoguer::{Input, Confirm, theme::ColorfulTheme};
use crate::errors::Result;

pub fn prompt_commit_message(current_message: &str) -> Result<String> {
    println!("Rusty  Generated message: {}", current_message);
    
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use this message?")
        .default(true)
        .interact()? 
    {
        return Ok(current_message.to_string());
    }
    
    // Let user edit the message
    let edited = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your commit message")
        .with_initial_text(current_message)
        .interact_text()?;
    
    Ok(edited)
}

pub fn confirm_commit(message: &str) -> Result<bool> {
    println!("📝 Ready to commit with message: \"{}\"", message);
    
    Ok(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit with this message?")
        .default(true)
        .interact()?)
}

pub fn prompt_api_key() -> Result<String> {
    let api_key = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your OpenRouter API key")
        .interact()?;
    
    Ok(api_key)
}
