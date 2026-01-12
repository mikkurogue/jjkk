use std::process::Command;

use anyhow::Result;

use super::repo::{
    ChangeType,
    FileStatus,
};

pub fn get_working_copy_status() -> Result<Vec<FileStatus>> {
    let output = Command::new("jj").args(["status", "--no-pager"]).output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut files = Vec::new();

    for line in stdout.lines() {
        if let Some(file_status) = parse_status_line(line) {
            files.push(file_status);
        }
    }

    Ok(files)
}

fn parse_status_line(line: &str) -> Option<FileStatus> {
    let line = line.trim();

    // Parse "A file.txt" or "M file.txt" or "D file.txt" format
    if line.len() < 3 {
        return None;
    }

    let status_char = line.chars().next()?;
    let change_type = match status_char {
        'A' => ChangeType::Added,
        'M' => ChangeType::Modified,
        'D' => ChangeType::Deleted,
        _ => return None,
    };

    let path = line[1..].trim().to_string();

    Some(FileStatus {
        path,
        status: change_type,
    })
}
