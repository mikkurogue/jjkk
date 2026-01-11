use std::process::Command;

use anyhow::{
    Context,
    Result,
};

#[derive(Debug)]
pub struct CommitInfo {
    pub change_id:   String,
    /// Short commit id, currently unused it seems
    _commit_id:      String,
    pub description: String,
    pub author:      String,
}

pub fn get_log(limit: usize) -> Result<Vec<CommitInfo>> {
    let output = Command::new("jj")
        .args([
            "log",
            "--limit",
            &limit.to_string(),
            "--no-graph",
            "-T",
            r#"change_id.short() ++ " " ++ commit_id.short() ++ " " ++ description.first_line() ++ " <" ++ author.email() ++ ">\n""#,
        ])
        .output()
        .context("Failed to get log")?;

    if !output.status.success() {
        anyhow::bail!("jj log failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut commits = Vec::new();

    for line in stdout.lines() {
        // Parse format: "change_id commit_id description <email>"
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() >= 3 {
            let change_id = parts[0].to_string();
            let commit_id = parts[1].to_string();

            // Split description and author
            let rest = parts[2];
            if let Some(author_start) = rest.rfind('<') {
                let description = rest[..author_start].trim().to_string();
                let author = rest[author_start..].to_string();

                commits.push(CommitInfo {
                    change_id,
                    _commit_id: commit_id,
                    description,
                    author,
                });
            }
        }
    }

    Ok(commits)
}
