use std::time::Instant;

use anyhow::Result;
use crossterm::event::{
    KeyCode,
    KeyEvent,
    KeyModifiers,
};

use crate::{
    config::{
        Settings,
        Theme,
    },
    jj::{
        native_operations::Native,
        repo::{
            FileStatus,
            JjRepo,
        },
    },
};

/// Each tab of the ui that can be selected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    /// Current working copy
    WorkingCopy,
    /// Known Bookmarks tab
    Bookmarks,
    /// Log tab
    Log,
}

impl Tab {
    pub const fn next(self) -> Self {
        match self {
            Self::WorkingCopy => Self::Bookmarks,
            Self::Bookmarks => Self::Log,
            Self::Log => Self::WorkingCopy,
        }
    }

    pub const fn prev(self) -> Self {
        match self {
            Self::WorkingCopy => Self::Log,
            Self::Bookmarks => Self::WorkingCopy,
            Self::Log => Self::Bookmarks,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PopupState {
    None,
    Input {
        title:           String,
        content:         String,
        cursor_position: usize,
        callback:        PopupCallback,
    },
    BookmarkSelect {
        content: String,
        cursor_position: usize,
        available_bookmarks: Vec<String>,
        selected_index: usize,
    },
    Error {
        message: String,
    },
    Warning {
        message: String,
    },
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupCallback {
    Describe,
    Commit,
    Rebase,
}

pub struct App {
    pub current_tab: Tab,
    pub settings: Settings,
    pub theme: Theme,
    pub should_quit: bool,
    pub popup_state: PopupState,
    pub status_message: Option<String>,
    pub status_message_timestamp: Option<Instant>,
    pub loading_message: Option<String>,
    pub loading_start: Option<Instant>,
    pub selected_file_index: usize,
    pub selected_bookmark_index: usize,
    pub selected_log_index: usize,
    pub diff_scroll_offset: usize,
    /// Marked with underscore to indicate it's currently unused
    _scroll_offset: usize,
    /// Marked with underscore to indicate it's currently unused
    _repo: JjRepo,
    pub files: Vec<FileStatus>,
    pub current_diff: Option<String>,

    pub native_ops: Native,
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
            status_message_timestamp: None,
            loading_message: None,
            loading_start: None,
            selected_file_index: 0,
            selected_bookmark_index: 0,
            selected_log_index: 0,
            diff_scroll_offset: 0,
            _scroll_offset: 0,
            _repo: repo,
            files: Vec::new(),
            current_diff: None,
            native_ops: Native::new(),
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
            ref mut cursor_position,
            callback,
            ..
        } = self.popup_state
        {
            // Helper to get byte position from character position
            let char_to_byte = |s: &str, char_pos: usize| -> usize {
                s.char_indices()
                    .nth(char_pos)
                    .map_or(s.len(), |(byte_pos, _)| byte_pos)
            };

            match key.code {
                KeyCode::Esc => {
                    self.popup_state = PopupState::None;
                }
                KeyCode::Enter => {
                    // Shift+Enter inserts a newline
                    if key.modifiers.contains(KeyModifiers::SHIFT) {
                        let byte_pos = char_to_byte(content, *cursor_position);
                        content.insert(byte_pos, '\n');
                        *cursor_position += 1;
                    } else {
                        // Regular Enter submits the form
                        let text = content.clone();
                        let cb = callback;
                        self.popup_state = PopupState::None;
                        self.execute_popup_callback(cb, &text)?;
                    }
                }
                KeyCode::Char(c) => {
                    let byte_pos = char_to_byte(content, *cursor_position);
                    content.insert(byte_pos, c);
                    *cursor_position += 1;
                }
                KeyCode::Backspace => {
                    if *cursor_position > 0 {
                        *cursor_position -= 1;
                        let byte_pos = char_to_byte(content, *cursor_position);
                        content.remove(byte_pos);
                    }
                }
                KeyCode::Left => {
                    *cursor_position = cursor_position.saturating_sub(1);
                }
                KeyCode::Right => {
                    let char_len = content.chars().count();
                    *cursor_position = (*cursor_position + 1).min(char_len);
                }
                KeyCode::Home => {
                    *cursor_position = 0;
                }
                KeyCode::End => {
                    *cursor_position = content.chars().count();
                }
                _ => {}
            }
            return Ok(());
        }

        // Handle bookmark selection popup
        if let PopupState::BookmarkSelect {
            ref mut content,
            ref mut cursor_position,
            ref available_bookmarks,
            ref mut selected_index,
        } = self.popup_state
        {
            // Helper to get byte position from character position
            let char_to_byte = |s: &str, char_pos: usize| -> usize {
                s.char_indices()
                    .nth(char_pos)
                    .map_or(s.len(), |(byte_pos, _)| byte_pos)
            };

            // Filter bookmarks based on current content
            let filtered: Vec<&String> = if content.is_empty() {
                available_bookmarks.iter().collect()
            } else {
                available_bookmarks
                    .iter()
                    .filter(|b| b.to_lowercase().contains(&content.to_lowercase()))
                    .collect()
            };

            match key.code {
                KeyCode::Esc => {
                    self.popup_state = PopupState::None;
                }
                KeyCode::Enter => {
                    // If there's filtered content and user selected from list, use that
                    let bookmark_name = if !filtered.is_empty() && *selected_index < filtered.len()
                    {
                        filtered[*selected_index].clone()
                    } else if !content.is_empty() {
                        // Otherwise use the typed content as new bookmark name
                        content.clone()
                    } else {
                        // Empty input, do nothing
                        self.popup_state = PopupState::None;
                        return Ok(());
                    };

                    self.popup_state = PopupState::None;
                    match crate::jj::operations::set_bookmark(&bookmark_name) {
                        Ok(_) => {
                            self.set_status_message(format!("Set bookmark: {bookmark_name}"));
                            self.refresh_status()?;
                        }
                        Err(e) => {
                            self.show_error(format!("Failed to set bookmark: {e}"));
                        }
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if !filtered.is_empty() {
                        *selected_index = selected_index.saturating_sub(1);
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !filtered.is_empty() {
                        *selected_index = (*selected_index + 1).min(filtered.len() - 1);
                    }
                }
                KeyCode::Tab => {
                    // Autocomplete with selected bookmark
                    if !filtered.is_empty() && *selected_index < filtered.len() {
                        *content = filtered[*selected_index].clone();
                        *cursor_position = content.chars().count();
                    }
                }
                KeyCode::Char(c) => {
                    let byte_pos = char_to_byte(content, *cursor_position);
                    content.insert(byte_pos, c);
                    *cursor_position += 1;
                    *selected_index = 0; // Reset selection when typing
                }
                KeyCode::Backspace => {
                    if *cursor_position > 0 {
                        *cursor_position -= 1;
                        let byte_pos = char_to_byte(content, *cursor_position);
                        content.remove(byte_pos);
                        *selected_index = 0; // Reset selection when deleting
                    }
                }
                KeyCode::Left => {
                    *cursor_position = cursor_position.saturating_sub(1);
                }
                KeyCode::Right => {
                    let char_len = content.chars().count();
                    *cursor_position = (*cursor_position + 1).min(char_len);
                }
                KeyCode::Home => {
                    *cursor_position = 0;
                }
                KeyCode::End => {
                    *cursor_position = content.chars().count();
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
        if matches!(self.popup_state, PopupState::Help) {
            match key.code {
                KeyCode::Char('?' | 'q') | KeyCode::Esc => {
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
                        if let Ok(bookmarks) = crate::jj::operations::get_bookmarks()
                            && !bookmarks.is_empty()
                        {
                            self.selected_bookmark_index =
                                (self.selected_bookmark_index + 1).min(bookmarks.len() - 1);
                        }
                    }
                    Tab::Log => {
                        if let Ok(commits) =
                            crate::jj::log::get_log(self.settings.ui.log_commits_count)
                            && !commits.is_empty()
                        {
                            self.selected_log_index =
                                (self.selected_log_index + 1).min(commits.len() - 1);
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
                    Tab::Log | Tab::WorkingCopy => {
                        // TODO: Show commit details
                    }
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
            KeyCode::Char('t') => {
                self.track_current_bookmark();
            }
            KeyCode::Char('R') => {
                // Capital R to refresh status
                self.refresh_status()?;
                self.set_status_message("Refreshed".to_string());
            }
            KeyCode::Char('X') => {
                // Capital X to restore the working copy (aka discard changes)
                self.restore_working_copy()?;
                self.set_status_message("Restored working copy".to_owned());
            }
            _ => {}
        }

        Ok(())
    }

    fn track_current_bookmark(&mut self) {
        match crate::jj::operations::track_current_bookmark() {
            Ok(v) => {
                self.set_status_message(v);
            }
            Err(e) => {
                self.show_error(format!("Failed to track bookmark: {e}"));
            }
        }
    }

    fn restore_working_copy(&mut self) -> Result<()> {
        match crate::jj::operations::restore_working_copy() {
            Ok(_) => {
                self.refresh_status()?;
            }
            Err(e) => {
                self.show_error(format!("Failed to restore working copy: {e}"));
            }
        }
        Ok(())
    }

    fn show_describe_popup(&mut self) {
        self.popup_state = PopupState::Input {
            title:           "Describe".to_string(),
            content:         String::new(),
            cursor_position: 0,
            callback:        PopupCallback::Describe,
        };
    }

    fn show_commit_popup(&mut self) {
        self.popup_state = PopupState::Input {
            title:           "Commit".to_string(),
            content:         String::new(),
            cursor_position: 0,
            callback:        PopupCallback::Commit,
        };
    }

    fn show_rebase_popup(&mut self) {
        self.popup_state = PopupState::Input {
            title:           "Rebase destination".to_string(),
            content:         String::new(),
            cursor_position: 0,
            callback:        PopupCallback::Rebase,
        };
    }

    fn show_bookmark_popup(&mut self) {
        // Fetch available bookmarks
        let bookmarks = crate::jj::operations::get_bookmarks().map_or_else(
            |_| Vec::new(),
            |bookmarks| bookmarks.into_iter().map(|b| b.name).collect(),
        );

        self.popup_state = PopupState::BookmarkSelect {
            content: String::new(),
            cursor_position: 0,
            available_bookmarks: bookmarks,
            selected_index: 0,
        };
    }

    fn execute_popup_callback(&mut self, callback: PopupCallback, text: &str) -> Result<()> {
        match callback {
            PopupCallback::Describe => match self.native_ops.describe(text) {
                Ok(_) => {
                    self.set_status_message("Description updated".to_string());
                    self.refresh_status()?;
                }
                Err(e) => {
                    self.show_error(format!("Failed to describe: {e}"));
                }
            },
            PopupCallback::Commit => match crate::jj::operations::commit(text) {
                Ok(_) => {
                    self.set_status_message("Committed successfully".to_string());
                    self.refresh_status()?;
                }
                Err(e) => {
                    self.show_error(format!("Failed to commit: {e}"));
                }
            },
            PopupCallback::Rebase => match crate::jj::operations::rebase(text) {
                Ok(_) => {
                    self.set_status_message(format!("Rebased to {text}"));
                    self.refresh_status()?;
                }
                Err(e) => {
                    self.show_error(format!("Failed to rebase: {e}"));
                }
            },
        }
        Ok(())
    }

    fn handle_new_commit(&mut self) -> Result<()> {
        // Check if working copy is already empty
        match crate::jj::operations::is_working_copy_empty() {
            Ok(true) => {
                self.show_warning("Already on an empty commit. Add changes first.".to_string());
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
                        self.show_error(format!("Failed to create new commit: {e}"));
                    }
                }
            }
            Err(e) => {
                self.show_error(format!("Failed to check working copy: {e}"));
            }
        }
        Ok(())
    }

    fn handle_fetch(&mut self) -> Result<()> {
        self.show_loading("Fetching from remote".to_string());
        match crate::jj::operations::git_fetch() {
            Ok(_) => {
                self.clear_loading();
                self.set_status_message("Fetched from remote".to_string());
                self.refresh_status()?;
            }
            Err(e) => {
                self.clear_loading();
                self.show_error(format!("Failed to fetch: {e}"));
            }
        }
        Ok(())
    }

    fn handle_push(&mut self) -> Result<()> {
        self.show_loading("Pushing to remote".to_string());
        let bookmark = crate::jj::operations::get_current_bookmark().ok().flatten();
        match crate::jj::operations::git_push(bookmark.as_deref()) {
            Ok(_) => {
                self.clear_loading();
                let msg = bookmark.map_or_else(
                    || "Pushed current change (created temporary bookmark)".to_string(),
                    |b| format!("Pushed bookmark: {b}"),
                );
                self.set_status_message(msg);
                self.refresh_status()?;
            }
            Err(e) => {
                self.clear_loading();
                self.show_error(format!("Failed to push: {e}"));
            }
        }
        Ok(())
    }

    pub fn set_status_message(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_message_timestamp = Some(Instant::now());
    }

    pub fn clear_status_message(&mut self) {
        self.status_message = None;
        self.status_message_timestamp = None;
    }

    pub fn update_status_message_timeout(&mut self) {
        if let Some(timestamp) = self.status_message_timestamp
            && timestamp.elapsed().as_secs() >= 2
        {
            self.clear_status_message();
        }
    }

    pub fn show_error(&mut self, message: String) {
        self.popup_state = PopupState::Error { message };
    }

    pub fn show_warning(&mut self, message: String) {
        self.popup_state = PopupState::Warning { message };
    }

    pub fn show_loading(&mut self, message: String) {
        self.loading_message = Some(message);
        self.loading_start = Some(Instant::now());
    }

    pub fn clear_loading(&mut self) {
        self.loading_message = None;
        self.loading_start = None;
    }

    pub fn get_spinner_char(&self) -> char {
        self.loading_start.map_or(' ', |start| {
            let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let elapsed = start.elapsed().as_millis();
            let frame_index = (elapsed / 80) as usize % frames.len();
            frames[frame_index]
        })
    }

    fn handle_bookmark_checkout(&mut self) -> Result<()> {
        let bookmarks = crate::jj::operations::get_bookmarks()?;
        if let Some(bookmark) = bookmarks.get(self.selected_bookmark_index) {
            match crate::jj::operations::checkout_bookmark(&bookmark.name) {
                Ok(_) => {
                    self.set_status_message(format!("Checked out bookmark: {}", bookmark.name));
                    // auto track the bookmark
                    crate::jj::operations::auto_track_bookmark(&bookmark.name).ok();
                    self.refresh_status()?;
                }
                Err(e) => {
                    self.show_error(format!("Failed to checkout bookmark: {e}"));
                }
            }
        }
        Ok(())
    }
}
