use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Tabs},
};

use crate::app::{App, PopupState, Tab};
use crate::ui::tabs::bookmarks::render_bookmarks;
use crate::ui::tabs::log::render_log;
use crate::ui::tabs::working_copy::render_working_copy;
use crate::ui::widgets::popup::{render_error_popup, render_input_popup};
use crate::ui::widgets::status_bar::render_status_bar;

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
        PopupState::Input { title, content, .. } => {
            render_input_popup(f, app, title, content, size);
        }
        PopupState::Error { message } => {
            render_error_popup(f, app, message, size);
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

fn render_placeholder(f: &mut Frame, app: &App, text: &str, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.surface1))
        .style(Style::default().bg(app.theme.base));

    let text_widget = ratatui::widgets::Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(app.theme.text));

    f.render_widget(text_widget, area);
}
