use std::path::PathBuf;

use anyhow::Result;

// Placeholder for now - will implement with jj-lib once we figure out the API
pub struct JjRepo {
    _workspace_root: PathBuf,
}

impl JjRepo {
    pub fn open(path: Option<PathBuf>) -> Result<Self> {
        let cwd = path.unwrap_or_else(|| std::env::current_dir().expect("Failed to get cwd"));

        // TODO : make this a bit nicer
        if !cwd.join(".jj").is_dir() {
            anyhow::bail!("No jj repository found at {}", cwd.display());
        }

        // TODO: Open workspace with jj-lib
        Ok(Self {
            _workspace_root: cwd,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FileStatus {
    pub path:   String,
    pub status: ChangeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
}

impl ChangeType {
    pub const fn symbol(&self) -> &str {
        match self {
            Self::Added => "A",
            Self::Modified => "M",
            Self::Deleted => "D",
        }
    }
}
