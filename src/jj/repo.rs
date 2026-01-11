use anyhow::Result;
use std::path::PathBuf;

// Placeholder for now - will implement with jj-lib once we figure out the API
pub struct JjRepo {
    _workspace_root: PathBuf,
}

impl JjRepo {
    pub fn open(path: Option<PathBuf>) -> Result<Self> {
        let cwd = path.unwrap_or_else(|| std::env::current_dir().expect("Failed to get cwd"));

        // TODO: Open workspace with jj-lib
        Ok(Self {
            _workspace_root: cwd,
        })
    }

    pub fn get_status(&self) -> Result<Vec<FileStatus>> {
        // TODO: Implement with jj-lib
        // For now, use jj status command output
        Ok(vec![])
    }
}

#[derive(Debug, Clone)]
pub struct FileStatus {
    pub path: String,
    pub status: ChangeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
}

impl ChangeType {
    pub fn symbol(&self) -> &str {
        match self {
            ChangeType::Added => "A",
            ChangeType::Modified => "M",
            ChangeType::Deleted => "D",
        }
    }
}
