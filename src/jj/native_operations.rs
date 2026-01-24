use std::{
    collections::HashMap,
    sync::Arc,
};

use anyhow::{
    Ok,
    Result,
};
use jj_lib::{
    config::{
        ConfigSource,
        StackedConfig,
    },
    git::{
        GitFetch,
        GitImportOptions,
        GitSubprocessOptions,
        RemoteCallbacks,
        expand_default_fetch_refspecs,
        get_all_remote_names,
        get_git_repo,
    },
    object_id::ObjectId,
    ref_name::{
        RefName,
        RemoteName,
    },
    repo::{
        ReadonlyRepo,
        Repo,
        StoreFactories,
    },
    settings::UserSettings,
    workspace::{
        Workspace,
        default_working_copy_factories,
    },
};

pub struct Native {
    pub workspace:      Workspace,
    pub repo:           Arc<ReadonlyRepo>,
    #[allow(dead_code)] // - not actually dead code, just not yet used in a user facing way
    pub origin_names: Vec<String>,
    pub default_remote: String,
}

impl Native {
    /// Create a new native jj operation handler
    /// for now its empty
    pub fn new() -> Self {
        let workspace = detect_workspace().expect("Failed to detect workspace");
        let repo = workspace
            .repo_loader()
            .load_at_head()
            .expect("Failed to load repo head");

        let remote_names = get_all_remote_names(repo.store()).expect("Failed to get remotes");
        let remotes = remote_names
            .iter()
            .map(|re| re.as_str().to_owned())
            .collect();

        let default_remote = if remote_names.is_empty() {
            String::from("origin")
        } else {
            remote_names[0].as_str().to_owned()
        };

        Self {
            workspace,
            repo,
            origin_names: remotes,
            default_remote,
        }
    }

    /// Describe the current change with a message using jj-lib
    /// This is a native implementation using the jj-lib crate instead of CLI interop
    pub fn describe(&self, message: &str) -> Result<String> {
        // validate that there is at least some kind of message
        if message.trim().is_empty() {
            return Err(anyhow::anyhow!("Description message cannot be empty"));
        }

        // Start a transaction
        let mut tx = self.repo.start_transaction();

        // Get the working copy commit ID
        let wc_commit_id = tx
            .repo()
            .view()
            .get_wc_commit_id(self.workspace.workspace_name())
            .ok_or_else(|| anyhow::anyhow!("No working copy commit found"))?
            .clone();

        // Load the working copy commit
        let wc_commit = tx.repo().store().get_commit(&wc_commit_id)?;

        // Rewrite the commit with the new description
        tx.repo_mut()
            .rewrite_commit(&wc_commit)
            .set_description(message)
            .write()?;

        // Rebase any descendants
        tx.repo_mut().rebase_descendants()?;

        // Commit the transaction
        tx.commit("describe working copy")?;

        Ok(format!(
            "Working copy commit description updated to: {message}"
        ))
    }

    /// Commit the current change with a message and create a new empty working copy commit
    /// This is equivalent to `jj commit -m <message>`
    pub fn commit(&self, message: &str) -> Result<String> {
        // validate that there is at least some kind of message
        if message.trim().is_empty() {
            return Err(anyhow::anyhow!("Commit message cannot be empty"));
        }

        let mut tx = self.repo.start_transaction();

        let wc_commit_id = tx
            .repo()
            .view()
            .get_wc_commit_id(self.workspace.workspace_name())
            .ok_or_else(|| anyhow::anyhow!("No working copy commit found"))?
            .clone();

        let wc_commit = tx.repo().store().get_commit(&wc_commit_id)?;

        let committed = tx
            .repo_mut()
            .rewrite_commit(&wc_commit)
            .set_description(message)
            .write()?;

        // Create an empty tree for the new working copy commit
        let empty_tree = tx.repo().store().empty_merged_tree();

        // Create a new empty working copy commit as a child of the committed version
        let new_wc = tx
            .repo_mut()
            .new_commit(
                vec![committed.id().clone()], // Parent is the commit we just created
                empty_tree,                   // Empty tree for the new working copy
            )
            .write()?;

        // Update the working copy to point to the new empty commit
        tx.repo_mut().set_wc_commit(
            self.workspace.workspace_name().to_owned(),
            new_wc.id().clone(),
        )?;

        // Rebase any descendants
        tx.repo_mut().rebase_descendants()?;

        // Commit the transaction
        tx.commit("commit working copy")?;

        Ok(format!(
            "Created commit {} with description: {}",
            committed.id().hex(),
            message
        ))
    }

    /// Fetch changes from the remote git repository using native jj-lib
    /// This is a native implementation using the jj-lib crate instead of CLI interop
    pub fn git_fetch(&self, remote: Option<&str>) -> Result<String> {
        let remote = remote.map_or_else(
            || self.default_remote.clone(),
            std::borrow::ToOwned::to_owned,
        );

        // Start a transaction
        let mut tx = self.repo.start_transaction();

        // Get user settings for subprocess options
        let user_settings = detect_user_settings()?;

        // Create subprocess options from settings
        let subprocess_options = GitSubprocessOptions::from_settings(&user_settings)?;

        // Create import options with defaults
        // These control how Git refs are imported into jj
        let import_options = GitImportOptions {
            auto_local_bookmark:         false, // Don't auto-create local bookmarks
            abandon_unreachable_commits: true,  // Clean up unreachable commits
            remote_auto_track_bookmarks: HashMap::new(), // Use default tracking config
        };

        // Get the underlying git repository before creating GitFetch
        // We need this to expand refspecs
        let git_repo = get_git_repo(tx.repo().store())?;

        let remote_name = RemoteName::new(&remote);

        // Expand the default fetch refspecs for the remote
        // This determines what refs to fetch (typically refs/heads/*)
        let (_ignored_refspecs, refspecs) = expand_default_fetch_refspecs(remote_name, &git_repo)?;

        // Create GitFetch handler (after we're done with the immutable borrow above)
        let mut git_fetch = GitFetch::new(tx.repo_mut(), subprocess_options, &import_options)?;

        // Set up callbacks for progress reporting (currently no-op)
        // You can extend this to provide progress updates
        let callbacks = RemoteCallbacks::default();

        // Perform the actual fetch operation
        // Parameters:
        // - remote_name: "origin"
        // - refspecs: what to fetch
        // - callbacks: progress reporting
        // - depth: None for full history (could use Some(n) for shallow fetch)
        // - fetch_tags_override: None to use git config default
        git_fetch.fetch(remote_name, refspecs, callbacks, None, None)?;

        // Import the fetched refs into jj's view
        let stats = git_fetch.import_refs()?;

        // Commit the transaction
        tx.commit("fetch from git remote")?;

        // Return a summary of what was fetched
        Ok(format!(
            "Fetched from origin\n\
             {} remote bookmarks imported",
            stats.changed_remote_bookmarks.len()
        ))
    }

    pub fn track(&self, bookmark_name: &str, remote: Option<&str>) -> Result<String> {
        let remote = remote.map_or_else(
            || self.default_remote.clone(),
            std::borrow::ToOwned::to_owned,
        );

        let mut tx = self.repo.start_transaction();

        let remote_name = RemoteName::new(&remote);
        let ref_name = RefName::new(bookmark_name);
        let symbol = ref_name.to_remote_symbol(remote_name);

        let remote_ref = tx.repo().view().get_remote_bookmark(symbol);

        if remote_ref.is_tracked() {
            return Ok(format!(
                "Remote bookmark already tracked: {bookmark_name}@{remote}"
            ));
        }

        tx.repo_mut().track_remote_bookmark(symbol)?;

        let local_target = tx.repo().view().get_local_bookmark(&ref_name);
        let has_conflict = local_target.has_conflict();

        tx.commit(&format!("track remote bookmark {bookmark_name}@{remote}"))?;

        let mut message = String::from("Started tracking 1 remote bookmarks.");

        if has_conflict {
            message.push_str(&format!(
                "\nWarning: Tracking created conflicts in local bookmark '{bookmark_name}'.\n\
                 Run 'jj log' or 'jj st' to see the conflicted state."
            ));
        }

        Ok(message)
    }
}

fn detect_workspace() -> Result<Workspace> {
    // Create user settings from default config
    let user_settings = detect_user_settings()?;

    // Load the workspace
    let workspace = Workspace::load(
        &user_settings,
        std::path::Path::new("."),
        &StoreFactories::default(),
        &default_working_copy_factories(),
    )?;

    Ok(workspace)
}

fn detect_config() -> Result<StackedConfig> {
    // Create stacked config with defaults
    let mut config = StackedConfig::with_defaults();

    // Load user config from standard location (~/.config/jj/config.toml)
    if let Some(config_dir) = dirs::config_dir() {
        let user_config_path = config_dir.join("jj").join("config.toml");
        if user_config_path.exists() {
            config.load_file(ConfigSource::User, user_config_path)?;
        }
    }

    // Load repo config from .jj/repo/config.toml if it exists
    let repo_config_path = std::path::Path::new(".jj/repo/config.toml");
    if repo_config_path.exists() {
        config.load_file(ConfigSource::Repo, repo_config_path)?;
    }

    Ok(config)
}

fn detect_user_settings() -> Result<UserSettings> {
    let config = detect_config()?;
    let user_settings = UserSettings::from_config(config)?;
    Ok(user_settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Only run manually in a jj repo
    fn test_describe_jj() {
        let native = Native::new();

        let result = native.describe("Test description from jj-lib");
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore] // Only run manually in a jj repo
    fn test_commit_jj() {
        let native = Native::new();

        // First set up a working copy with some description
        let describe_result = native.describe("Setting up test commit");
        assert!(describe_result.is_ok());

        // Now commit it
        let commit_result = native.commit("Test commit from jj-lib");
        println!("{:?}", commit_result);
        assert!(commit_result.is_ok());
    }

    #[test]
    #[ignore] // Only run manually in a jj repo with a git remote configured
    fn test_git_fetch_jj() {
        let native = Native::new();

        let result = native.git_fetch(None);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
