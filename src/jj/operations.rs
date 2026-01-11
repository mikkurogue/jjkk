use anyhow::{Context, Result};
use std::process::Command;

pub fn get_file_diff(file_path: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(&["diff", "--no-pager", file_path])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn describe(message: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(&["describe", "-m", message])
        .output()
        .context("Failed to run jj describe")?;

    if !output.status.success() {
        anyhow::bail!(
            "jj describe failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn commit(message: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(&["commit", "-m", message])
        .output()
        .context("Failed to run jj commit")?;

    if !output.status.success() {
        anyhow::bail!(
            "jj commit failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn new_commit() -> Result<String> {
    let output = Command::new("jj")
        .args(&["new"])
        .output()
        .context("Failed to run jj new")?;

    if !output.status.success() {
        anyhow::bail!("jj new failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn git_fetch() -> Result<String> {
    let output = Command::new("jj")
        .args(&["git", "fetch"])
        .output()
        .context("Failed to run jj git fetch")?;

    if !output.status.success() {
        anyhow::bail!(
            "jj git fetch failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn git_push(bookmark: Option<&str>) -> Result<String> {
    let mut args = vec!["git", "push"];

    if let Some(bookmark_name) = bookmark {
        args.push("-b");
        args.push(bookmark_name);
    }

    let output = Command::new("jj")
        .args(&args)
        .output()
        .context("Failed to run jj git push")?;

    if !output.status.success() {
        anyhow::bail!(
            "jj git push failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn rebase(destination: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(&["rebase", "-d", destination])
        .output()
        .context("Failed to run jj rebase")?;

    if !output.status.success() {
        anyhow::bail!(
            "jj rebase failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn set_bookmark(name: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(&["bookmark", "set", name])
        .output()
        .context("Failed to run jj bookmark set")?;

    if !output.status.success() {
        anyhow::bail!(
            "jj bookmark set failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn get_current_bookmark() -> Result<Option<String>> {
    let output = Command::new("jj")
        .args(&["log", "-r", "@", "--no-graph", "-T", "bookmarks"])
        .output()
        .context("Failed to get current bookmark")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let bookmark = stdout.trim();

    if bookmark.is_empty() {
        Ok(None)
    } else {
        Ok(Some(bookmark.to_string()))
    }
}

pub fn is_working_copy_empty() -> Result<bool> {
    let output = Command::new("jj")
        .args(&["status"])
        .output()
        .context("Failed to check working copy status")?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check if output contains "Working copy changes:" followed by nothing
    // or if it says "The working copy is clean"
    let is_empty = stdout.contains("The working copy is clean")
        || stdout
            .lines()
            .skip_while(|line| !line.contains("Working copy changes:"))
            .skip(1) // Skip the "Working copy changes:" line itself
            .next()
            .is_none();

    Ok(is_empty)
}

#[derive(Debug, Clone)]
pub struct BookmarkInfo {
    pub name: String,
    pub is_current: bool,
}

pub fn get_bookmarks() -> Result<Vec<BookmarkInfo>> {
    let output = Command::new("jj")
        .args(&["bookmark", "list"])
        .output()
        .context("Failed to get bookmarks")?;

    if !output.status.success() {
        anyhow::bail!(
            "jj bookmark list failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let current_bookmark = get_current_bookmark().ok().flatten();

    let mut bookmarks = Vec::new();
    for line in stdout.lines() {
        // Parse bookmark lines like "main: abc123 description"
        if let Some(name) = line.split(':').next() {
            let name = name.trim().to_string();
            if !name.is_empty() {
                let is_current = current_bookmark
                    .as_ref()
                    .map(|b| b == &name)
                    .unwrap_or(false);
                bookmarks.push(BookmarkInfo { name, is_current });
            }
        }
    }

    Ok(bookmarks)
}

pub fn checkout_bookmark(name: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(&["new", name])
        .output()
        .context("Failed to checkout bookmark")?;

    if !output.status.success() {
        anyhow::bail!("jj new failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
