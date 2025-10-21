use dialoguer::Input;

pub fn prompt_user(prompt_message: &str) -> String {
    Input::<String>::new()
        .with_prompt(prompt_message)
        .interact_text()
}
