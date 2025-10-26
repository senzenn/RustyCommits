use git2::{DiffOptions, Repository};
use crate::errors::Result;

#[derive(Debug)]
pub struct GitChanges {
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
    pub staged_diff: String,
    pub unstaged_diff: String,
}

pub fn get_git_changes(repo: &Repository) -> Result<GitChanges> {
    let mut staged_files = Vec::new();
    let mut unstaged_files = Vec::new();

    // Get status to identify changed files
    let statuses = repo.statuses(None)?;

    for entry in statuses.iter() {
        let path = entry.path().unwrap_or("").to_string();

        if entry.status().is_index_new() || entry.status().is_index_modified() || entry.status().is_index_deleted() {
            staged_files.push(path.clone());
        }

        if entry.status().is_wt_new() || entry.status().is_wt_modified() || entry.status().is_wt_deleted() {
            unstaged_files.push(path);
        }
    }

    // Get diffs
    let staged_diff = get_staged_diff(repo)?;
    let unstaged_diff = get_unstaged_diff(repo)?;

    Ok(GitChanges {
        staged_files,
        unstaged_files,
        staged_diff,
        unstaged_diff,
    })
}

fn get_staged_diff(repo: &Repository) -> Result<String> {
    let head = repo.head()?;
    let head_tree = head.peel_to_tree()?;
    let mut index = repo.index()?;
    let index_tree = repo.find_tree(index.write_tree()?)?;

    let diff = repo.diff_tree_to_tree(Some(&head_tree), Some(&index_tree), None)?;
    let mut diff_text = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        diff_text.extend_from_slice(line.content());
        true
    })?;

    Ok(String::from_utf8_lossy(&diff_text).to_string())
}

fn get_unstaged_diff(repo: &Repository) -> Result<String> {
    let mut opts = DiffOptions::new();
    let diff = repo.diff_index_to_workdir(None, Some(&mut opts))?;

    let mut diff_text = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        diff_text.extend_from_slice(line.content());
        true
    })?;

    Ok(String::from_utf8_lossy(&diff_text).to_string())
}

pub fn filter_diff_content(diff: &str, max_lines: usize) -> String {
    let lines: Vec<&str> = diff.lines().collect();

    if lines.len() <= max_lines {
        return diff.to_string();
    }

    // Keep header and truncate middle
    let header_lines = 10;
    let footer_lines = 10;
    let mut result = Vec::new();

    // Add header
    result.extend_from_slice(&lines[..header_lines.min(lines.len())]);
    result.push("... [diff truncated due to size] ...");

    // Add footer if there's space
    if lines.len() > header_lines + footer_lines {
        let start = lines.len().saturating_sub(footer_lines);
        result.extend_from_slice(&lines[start..]);
    }

    result.join("\n")
}

pub fn perform_git_commit(repo: &Repository, message: &str) -> Result<()> {
    // Stage all changes first
    let mut index = repo.index()?;
    index.add_all(["."], git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Create commit
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;

    let tree = repo.find_tree(index.write_tree()?)?;
    let signature = repo.signature()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&head_commit],
    )?;

    Ok(())
}

pub fn generate_fallback_message(files: &[String], diff: &str) -> String {

    if diff.is_empty() {
        return format!("Update {}", files.join(", "));
    }

    let lines: Vec<&str> = diff.lines().collect();
    let mut added_lines = 0;
    let mut deleted_lines = 0;

    for line in &lines {
        if line.starts_with('+') && !line.starts_with("+++") {
            added_lines += 1;
        } else if line.starts_with('-') && !line.starts_with("---") {
            deleted_lines += 1;
        }
    }

    // Detect common patterns
    let diff_lower = diff.to_lowercase();
    if diff_lower.contains("test") || diff_lower.contains("spec") {
        "Add/update tests".to_string()
    } else if diff_lower.contains("readme") || diff_lower.contains("doc") {
        "Update documentation".to_string()
    } else if diff_lower.contains("fix") || diff_lower.contains("bug") {
        "Fix bug".to_string()
    } else if added_lines > deleted_lines * 2 {
        format!("Add new functionality to {}", files.join(", "))
    } else if deleted_lines > added_lines * 2 {
        format!("Remove/cleanup code from {}", files.join(", "))
    } else {
        format!("Update {}", files.join(", "))
    }
}