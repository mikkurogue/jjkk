use ratatui::{
    Frame,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{
        Modifier,
        Style,
    },
    widgets::{
        Block,
        Borders,
        Tabs,
    },
};

use crate::{
    app::{
        App,
        PopupState,
        Tab,
    },
    ui::{
        tabs::{
            bookmarks::render_bookmarks,
            log::render_log,
            working_copy::render_working_copy,
        },
        widgets::{
            popup::{
                render_bookmark_select_popup,
                render_error_popup,
                render_help_popup,
                render_input_popup,
            },
            status_bar::render_status_bar,
        },
    },
};

/// Render the main ui of the application
/// Initial state should show the working copy tab
pub fn render_ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab bar
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(size);

    // Render tab bar
    render_tab_bar(f, app, chunks[0]);

    // Render current tab content
    render_tab_content(f, app, chunks[1]);

    // Render status bar
    render_status_bar(f, app, chunks[2]);

    // Render popups on top
    match &app.popup_state {
        PopupState::Input {
            title,
            content,
            cursor_position,
            ..
        } => {
            render_input_popup(f, app, title, content, *cursor_position, size);
        }
        PopupState::BookmarkSelect {
            content,
            cursor_position,
            available_bookmarks,
            selected_index,
        } => {
            render_bookmark_select_popup(
                f,
                app,
                content,
                *cursor_position,
                available_bookmarks,
                *selected_index,
                size,
            );
        }
        PopupState::Error { message } => {
            render_error_popup(f, app, message, size);
        }
        PopupState::Help => {
            render_help_popup(f, app, size);
        }
        PopupState::None => {}
    }
}

fn render_tab_bar(f: &mut Frame, app: &App, area: Rect) {
    let tab_titles = vec!["1: Working Copy", "2: Bookmarks", "3: Log"];
    let selected_index = match app.current_tab {
        Tab::WorkingCopy => 0,
        Tab::Bookmarks => 1,
        Tab::Log => 2,
    };

    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("jjkk"))
        .select(selected_index)
        .style(Style::default().fg(app.theme.text))
        .highlight_style(
            Style::default()
                .fg(app.theme.lavender)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn render_tab_content(f: &mut Frame, app: &App, area: Rect) {
    match app.current_tab {
        Tab::WorkingCopy => {
            render_working_copy(f, app, area);
        }
        Tab::Bookmarks => {
            render_bookmarks(f, app, area);
        }
        Tab::Log => {
            render_log(f, app, area);
        }
    }
}
