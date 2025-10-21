use dotenv::dotenv;
use git2::Repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // Option return :)

    let api_key = env::var("OPEN_ROUTER_API").unwrap_or_else(|| {
        prompt_and_save_env_var("OPEN_ROUTER_API", "Enter your OPEN Router API key: ").unwrap()
    }); // prompt for  open router api

    let repo = Repository::open(".")?;
    let statuses = repo.statuses(None)?;

    // collecting the modifications  across files

    let mut changed_files = Vec::new();
    for status in statuses.iter() {
        if status.status().is_wt_modified() || status.status().is_untracked() {
            changed_files.push(status.path()?.to_string());
        }
    }

    // BUG: file  is not opneing ????????? why
}
