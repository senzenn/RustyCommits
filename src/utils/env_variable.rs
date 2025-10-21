use dialoguer::Input;
use std::io::{self, Write};
use which::which;

pub fn prompt_and_save_env_variable(key: &str, prompt_message: &str) -> io::Result<String> {
    let variable = Input::<String>::new()
        .with_prompt(prompt_message)
        .interact()?;

    let shell = get_shell();
    save_env_variable(&shell, key, &variable)?;

    Ok(variable)
}

fn get_shell() -> String {
    if which("zsh").is_ok() {
        "zsh".to_string()
    } else if which("bash").is_ok() {
        "bash".to_string()
    } else {
        panic!("Unsupported shell. Please use either Zsh or Bash.");
    }
}

fn save_env_variable(shell: &str, key: &str, value: &str) -> io::Result<()> {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let config_file_path = if shell == "zsh" {
        home_dir.join(".zshrc")
    } else {
        home_dir.join(".bashrc")
    };

    let mut config_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(config_file_path)?;

    writeln!(config_file, "{}={}", key, value)?;

    Ok(())
}
