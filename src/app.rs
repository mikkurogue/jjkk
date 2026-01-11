use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

use crate::config::{Settings, Theme};
use crate::jj::repo::{FileStatus, JjRepo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    WorkingCopy,
    Bookmarks,
    Log,
}

impl Tab {
    pub fn next(self) -> Self {
        match self {
            Tab::WorkingCopy => Tab::Bookmarks,
            Tab::Bookmarks => Tab::Log,
            Tab::Log => Tab::WorkingCopy,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Tab::WorkingCopy => Tab::Log,
            Tab::Bookmarks => Tab::WorkingCopy,
            Tab::Log => Tab::Bookmarks,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Tab::WorkingCopy),
            1 => Some(Tab::Bookmarks),
            2 => Some(Tab::Log),
            _ => None,
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Tab::WorkingCopy => "Working Copy",
            Tab::Bookmarks => "Bookmarks",
            Tab::Log => "Log",
        }
    }
}

#[derive(Debug, Clone)]
pub enum PopupState {
    None,
    Input {
        title: String,
        content: String,
        callback: PopupCallback,
    },
    Error {
        message: String,
    },
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupCallback {
    Describe,
    Commit,
    Rebase,
    Bookmark,
}

pub struct App {
    pub current_tab: Tab,
    pub settings: Settings,
    pub theme: Theme,
    pub should_quit: bool,
    pub popup_state: PopupState,
    pub status_message: Option<String>,
    pub selected_file_index: usize,
    pub selected_bookmark_index: usize,
    pub selected_log_index: usize,
    pub diff_scroll_offset: usize,
    pub scroll_offset: usize,
    pub repo: JjRepo,
    pub files: Vec<FileStatus>,
    pub current_diff: Option<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        let settings = Settings::load()?;
        let theme = Theme::catppuccin_mocha();
        let repo = JjRepo::open(None)?;

        Ok(Self {
            current_tab: Tab::WorkingCopy,
            settings,
            theme,
            should_quit: false,
            popup_state: PopupState::None,
            status_message: None,
            selected_file_index: 0,
            selected_bookmark_index: 0,
            selected_log_index: 0,
            diff_scroll_offset: 0,
            scroll_offset: 0,
            repo,
            files: Vec::new(),
            current_diff: None,
        })
    }

    pub fn refresh_status(&mut self) -> Result<()> {
        self.files = crate::jj::status::get_working_copy_status()?;
        self.selected_file_index = self
            .selected_file_index
            .min(self.files.len().saturating_sub(1));
        self.diff_scroll_offset = 0;
        self.update_diff()?;
        Ok(())
    }

    pub fn update_diff(&mut self) -> Result<()> {
        if let Some(file) = self.files.get(self.selected_file_index) {
            self.current_diff = Some(crate::jj::operations::get_file_diff(&file.path)?);
        } else {
            self.current_diff = None;
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        // Handle popup input first
        if let PopupState::Input {
            ref mut content,
            callback,
            ..
        } = self.popup_state
        {
            match key.code {
                KeyCode::Esc => {
                    self.popup_state = PopupState::None;
                }
                KeyCode::Enter => {
                    let text = content.clone();
                    let cb = callback.clone();
                    self.popup_state = PopupState::None;
                    self.execute_popup_callback(cb, text)?;
                }
                KeyCode::Char(c) => {
                    content.push(c);
                }
                KeyCode::Backspace => {
                    content.pop();
                }
                _ => {}
            }
            return Ok(());
        }

        // Handle error popup
        if let PopupState::Error { .. } = self.popup_state {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    self.popup_state = PopupState::None;
                }
                _ => {}
            }
            return Ok(());
        }

        // Handle help popup
        if let PopupState::Help = self.popup_state {
            match key.code {
                KeyCode::Char('?') | KeyCode::Esc | KeyCode::Char('q') => {
                    self.popup_state = PopupState::None;
                }
                _ => {}
            }
            return Ok(());
        }

        // Handle normal key events
        match key.code {
            KeyCode::Char('?') => {
                self.popup_state = PopupState::Help;
            }
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('1') => {
                self.current_tab = Tab::WorkingCopy;
            }
            KeyCode::Char('2') => {
                self.current_tab = Tab::Bookmarks;
            }
            KeyCode::Char('3') => {
                self.current_tab = Tab::Log;
            }
            KeyCode::Tab => {
                self.current_tab = self.current_tab.next();
            }
            KeyCode::BackTab => {
                self.current_tab = self.current_tab.prev();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                match self.current_tab {
                    Tab::WorkingCopy => {
                        if !self.files.is_empty() {
                            self.selected_file_index =
                                (self.selected_file_index + 1).min(self.files.len() - 1);
                            self.update_diff()?;
                            self.diff_scroll_offset = 0; // Reset scroll when changing files
                        }
                    }
                    Tab::Bookmarks => {
                        if let Ok(bookmarks) = crate::jj::operations::get_bookmarks() {
                            if !bookmarks.is_empty() {
                                self.selected_bookmark_index =
                                    (self.selected_bookmark_index + 1).min(bookmarks.len() - 1);
                            }
                        }
                    }
                    Tab::Log => {
                        if let Ok(commits) =
                            crate::jj::log::get_log(self.settings.ui.log_commits_count)
                        {
                            if !commits.is_empty() {
                                self.selected_log_index =
                                    (self.selected_log_index + 1).min(commits.len() - 1);
                            }
                        }
                    }
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                match self.current_tab {
                    Tab::WorkingCopy => {
                        self.selected_file_index = self.selected_file_index.saturating_sub(1);
                        self.update_diff()?;
                        self.diff_scroll_offset = 0; // Reset scroll when changing files
                    }
                    Tab::Bookmarks => {
                        self.selected_bookmark_index =
                            self.selected_bookmark_index.saturating_sub(1);
                    }
                    Tab::Log => {
                        self.selected_log_index = self.selected_log_index.saturating_sub(1);
                    }
                }
            }
            KeyCode::Char('J') => {
                // Shift+J for scrolling diff down
                if self.current_tab == Tab::WorkingCopy && self.current_diff.is_some() {
                    self.diff_scroll_offset += 1;
                }
            }
            KeyCode::Char('K') => {
                // Shift+K for scrolling diff up
                if self.current_tab == Tab::WorkingCopy {
                    self.diff_scroll_offset = self.diff_scroll_offset.saturating_sub(1);
                }
            }
            KeyCode::Enter => {
                match self.current_tab {
                    Tab::Bookmarks => {
                        self.handle_bookmark_checkout()?;
                    }
                    Tab::Log => {
                        // TODO: Show commit details
                    }
                    _ => {}
                }
            }
            KeyCode::Char('d') if self.current_tab == Tab::WorkingCopy => {
                self.show_describe_popup();
            }
            KeyCode::Char('c') if self.current_tab == Tab::WorkingCopy => {
                self.show_commit_popup();
            }
            KeyCode::Char('n') if self.current_tab == Tab::WorkingCopy => {
                self.handle_new_commit()?;
            }
            KeyCode::Char('f') => {
                self.handle_fetch()?;
            }
            KeyCode::Char('p') => {
                self.handle_push()?;
            }
            KeyCode::Char('r') => {
                self.show_rebase_popup();
            }
            KeyCode::Char('b') => {
                self.show_bookmark_popup();
            }
            KeyCode::Char('R') => {
                // Capital R to refresh status
                self.refresh_status()?;
                self.set_status_message("Refreshed".to_string());
            }
            _ => {}
        }

        Ok(())
    }

    fn show_describe_popup(&mut self) {
        self.popup_state = PopupState::Input {
            title: "Describe".to_string(),
            content: String::new(),
            callback: PopupCallback::Describe,
        };
    }

    fn show_commit_popup(&mut self) {
        self.popup_state = PopupState::Input {
            title: "Commit".to_string(),
            content: String::new(),
            callback: PopupCallback::Commit,
        };
    }

    fn show_rebase_popup(&mut self) {
        self.popup_state = PopupState::Input {
            title: "Rebase destination".to_string(),
            content: String::new(),
            callback: PopupCallback::Rebase,
        };
    }

    fn show_bookmark_popup(&mut self) {
        self.popup_state = PopupState::Input {
            title: "Set bookmark".to_string(),
            content: String::new(),
            callback: PopupCallback::Bookmark,
        };
    }

    fn execute_popup_callback(&mut self, callback: PopupCallback, text: String) -> Result<()> {
        match callback {
            PopupCallback::Describe => match crate::jj::operations::describe(&text) {
                Ok(_) => {
                    self.set_status_message("Description updated".to_string());
                    self.refresh_status()?;
                }
                Err(e) => {
                    self.show_error(format!("Failed to describe: {}", e));
                }
            },
            PopupCallback::Commit => match crate::jj::operations::commit(&text) {
                Ok(_) => {
                    self.set_status_message("Committed successfully".to_string());
                    self.refresh_status()?;
                }
                Err(e) => {
                    self.show_error(format!("Failed to commit: {}", e));
                }
            },
            PopupCallback::Rebase => match crate::jj::operations::rebase(&text) {
                Ok(_) => {
                    self.set_status_message(format!("Rebased to {}", text));
                    self.refresh_status()?;
                }
                Err(e) => {
                    self.show_error(format!("Failed to rebase: {}", e));
                }
            },
            PopupCallback::Bookmark => match crate::jj::operations::set_bookmark(&text) {
                Ok(_) => {
                    self.set_status_message(format!("Set bookmark: {}", text));
                }
                Err(e) => {
                    self.show_error(format!("Failed to set bookmark: {}", e));
                }
            },
        }
        Ok(())
    }

    fn handle_new_commit(&mut self) -> Result<()> {
        // Check if working copy is already empty
        match crate::jj::operations::is_working_copy_empty() {
            Ok(true) => {
                self.show_error("Already on an empty commit. Add changes first.".to_string());
                return Ok(());
            }
            Ok(false) => {
                // Working copy has changes, proceed with new commit
                match crate::jj::operations::new_commit() {
                    Ok(_) => {
                        self.set_status_message("Created new commit".to_string());
                        self.refresh_status()?;
                    }
                    Err(e) => {
                        self.show_error(format!("Failed to create new commit: {}", e));
                    }
                }
            }
            Err(e) => {
                self.show_error(format!("Failed to check working copy: {}", e));
            }
        }
        Ok(())
    }

    fn handle_fetch(&mut self) -> Result<()> {
        match crate::jj::operations::git_fetch() {
            Ok(_) => {
                self.set_status_message("Fetched from remote".to_string());
                self.refresh_status()?;
            }
            Err(e) => {
                self.show_error(format!("Failed to fetch: {}", e));
            }
        }
        Ok(())
    }

    fn handle_push(&mut self) -> Result<()> {
        let bookmark = crate::jj::operations::get_current_bookmark().ok().flatten();
        match crate::jj::operations::git_push(bookmark.as_deref()) {
            Ok(_) => {
                let msg = if let Some(b) = bookmark {
                    format!("Pushed bookmark: {}", b)
                } else {
                    "Pushed current change (created temporary bookmark)".to_string()
                };
                self.set_status_message(msg);
            }
            Err(e) => {
                self.show_error(format!("Failed to push: {}", e));
            }
        }
        Ok(())
    }

    pub fn set_status_message(&mut self, message: String) {
        self.status_message = Some(message);
    }

    pub fn show_error(&mut self, message: String) {
        self.popup_state = PopupState::Error { message };
    }

    fn handle_bookmark_checkout(&mut self) -> Result<()> {
        let bookmarks = crate::jj::operations::get_bookmarks()?;
        if let Some(bookmark) = bookmarks.get(self.selected_bookmark_index) {
            match crate::jj::operations::checkout_bookmark(&bookmark.name) {
                Ok(_) => {
                    self.set_status_message(format!("Checked out bookmark: {}", bookmark.name));
                    self.refresh_status()?;
                }
                Err(e) => {
                    self.show_error(format!("Failed to checkout bookmark: {}", e));
                }
            }
        }
        Ok(())
    }
}
