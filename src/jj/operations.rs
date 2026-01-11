use std::process::Command;

use anyhow::{
    Context,
    Result,
};

/// Restore the working copy of a jj repository
/// Executes `jj restore` command
pub fn restore_working_copy() -> Result<String> {
    let output = Command::new("jj")
        .args(["restore"])
        .output()
        .context("Failed to run jj restore")?;

    if !output.status.success() {
        anyhow::bail!(
            "jj restore failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Get the diff of a file from the working copy
/// Executes `jj diff --no-pager <file_path>` command
pub fn get_file_diff(file_path: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(["diff", "--no-pager", file_path])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Describe the current change with a message
/// Executes `jj describe -m <message>` command
pub fn describe(message: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(["describe", "-m", message])
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

/// Commit a change and create a new empty commit on the working copy.
/// Executes `jj commit -m <message>` command
pub fn commit(message: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(["commit", "-m", message])
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

/// Create a new empty commit on the working copy.
/// Executes `jj new` command
pub fn new_commit() -> Result<String> {
    let output = Command::new("jj")
        .args(["new"])
        .output()
        .context("Failed to run jj new")?;

    if !output.status.success() {
        anyhow::bail!("jj new failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Fetch changes from the remote git repository
/// Executes `jj git fetch` command
pub fn git_fetch() -> Result<String> {
    let output = Command::new("jj")
        .args(["git", "fetch"])
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

/// Push changes to the remote git repository
/// If a bookmark is provided, push that bookmark
/// Otherwise, push the current change
/// Executes `jj git push -b <bookmark>` or `jj git push --change @` command
pub fn git_push(bookmark: Option<&str>) -> Result<String> {
    let mut args = vec!["git", "push"];

    if let Some(bookmark_name) = bookmark {
        args.push("-b");
        args.push(bookmark_name);
    } else {
        // If no bookmark, push the current change
        args.push("--change");
        args.push("@");
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

/// Rebase the current change onto the specified destination
/// Executes `jj rebase -d <destination>` command
pub fn rebase(destination: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(["rebase", "-d", destination])
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

/// Set a bookmark at the current change
/// Executes `jj bookmark set <name>` command
pub fn set_bookmark(name: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(["bookmark", "set", name])
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

/// Get the name of the current bookmark, if any
/// Executes `jj log -r @ --no-graph -T bookmarks` command
pub fn get_current_bookmark() -> Result<Option<String>> {
    let output = Command::new("jj")
        .args(["log", "-r", "@", "--no-graph", "-T", "bookmarks"])
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

/// Check if the working copy is empty (no uncommitted changes or no changes)
/// Executes `jj status` command
pub fn is_working_copy_empty() -> Result<bool> {
    let output = Command::new("jj")
        .args(["status"])
        .output()
        .context("Failed to check working copy status")?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check if output contains "Working copy changes:" followed by nothing
    // or if it says "The working copy is clean"
    let is_empty = stdout.contains("The working copy is clean")
        || stdout
            .lines()
            .skip_while(|line| !line.contains("Working copy changes:"))
            .nth(1)
            .is_none();

    Ok(is_empty)
}

#[derive(Debug, Clone)]
pub struct BookmarkInfo {
    pub name:       String,
    pub is_current: bool,
}

/// Get the list of bookmarks in the repository
/// Executes `jj bookmark list` command
pub fn get_bookmarks() -> Result<Vec<BookmarkInfo>> {
    let output = Command::new("jj")
        .args(["bookmark", "list"])
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
                let is_current = current_bookmark.as_ref().is_some_and(|b| b == &name);
                bookmarks.push(BookmarkInfo { name, is_current });
            }
        }
    }

    Ok(bookmarks)
}

/// Start work on a bookmark by creating a new change at that bookmark
/// Executes `jj new <name>` command
pub fn checkout_bookmark(name: &str) -> Result<String> {
    let output = Command::new("jj")
        .args(["new", name])
        .output()
        .context("Failed to checkout bookmark")?;

    if !output.status.success() {
        anyhow::bail!("jj new failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
