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
