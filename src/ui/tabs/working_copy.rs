use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;
use crate::jj::repo::ChangeType;

pub fn render_working_copy(f: &mut Frame, app: &App, area: Rect) {
    // Split into left (file list) and right (diff view)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // File list
            Constraint::Percentage(70), // Diff view
        ])
        .split(area);

    render_file_list(f, app, chunks[0]);
    render_diff_view(f, app, chunks[1]);
}

fn render_file_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let symbol = file.status.symbol();
            let color = match file.status {
                ChangeType::Added => app.theme.green,
                ChangeType::Modified => app.theme.blue,
                ChangeType::Deleted => app.theme.red,
            };

            let style = if i == app.selected_file_index {
                Style::default()
                    .fg(app.theme.text)
                    .bg(app.theme.surface1)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };

            ListItem::new(Line::from(vec![
                Span::styled(symbol, Style::default().fg(color)),
                Span::raw(" "),
                Span::styled(&file.path, style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Files")
                .border_style(Style::default().fg(app.theme.surface1)),
        )
        .style(Style::default().bg(app.theme.base));

    f.render_widget(list, area);
}

fn render_diff_view(f: &mut Frame, app: &App, area: Rect) {
    let diff_text = if let Some(ref diff) = app.current_diff {
        diff.clone()
    } else if app.files.is_empty() {
        "No changes in working copy".to_string()
    } else {
        "Select a file to view diff".to_string()
    };

    let paragraph = Paragraph::new(diff_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Diff")
                .border_style(Style::default().fg(app.theme.surface1)),
        )
        .style(Style::default().fg(app.theme.text).bg(app.theme.base))
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}
