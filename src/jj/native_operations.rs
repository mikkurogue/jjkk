use std::sync::Arc;

use anyhow::{
    Ok,
    Result,
};
use jj_lib::{
    config::StackedConfig,
    object_id::ObjectId,
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
    pub workspace: Workspace,
    pub repo:      Arc<ReadonlyRepo>,
}

impl Native {
    /// Create a new native jj operation handler
    /// for now its empty
    pub fn new() -> Self {
        let workspace = Self::detect_workspace().expect("Failed to detect workspace");
        let repo = workspace
            .repo_loader()
            .load_at_head()
            .expect("Failed to load repo head");

        Self { workspace, repo }
    }

    fn detect_workspace() -> Result<Workspace> {
        // Create user settings from default config
        let user_settings = Self::detect_user_settings()?;

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
        // Create user settings from default config
        let config = StackedConfig::with_defaults();
        Ok(config)
    }

    fn detect_user_settings() -> Result<UserSettings> {
        let config = Self::detect_config()?;
        let user_settings = UserSettings::from_config(config)?;
        Ok(user_settings)
    }

    /// Describe the current change with a message using jj-lib
    /// This is a native implementation using the jj-lib crate instead of CLI interop
    pub fn describe(&self, message: &str) -> Result<String> {
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
}
