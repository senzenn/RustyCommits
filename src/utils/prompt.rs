use dialoguer::Input;
use std::io;

pub fn prompt_user(prompt_message: &str) -> io::Result<String> {
    Input::<String>::new()
        .with_prompt(prompt_message)
        .interact_text() // or interact() for other types of inputs since without feature it's not
        // working idk why?
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
