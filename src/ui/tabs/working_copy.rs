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
    let lines: Vec<Line> = if let Some(ref diff) = app.current_diff {
        // Parse diff and colorize lines
        diff.lines()
            .map(|line| {
                let style = if line.starts_with('+') && !line.starts_with("+++") {
                    // Added line
                    Style::default().fg(app.theme.green)
                } else if line.starts_with('-') && !line.starts_with("---") {
                    // Removed line
                    Style::default().fg(app.theme.red)
                } else if line.starts_with("@@") {
                    // Hunk header
                    Style::default().fg(app.theme.blue)
                } else if line.starts_with("diff ") || line.starts_with("index ") {
                    // Diff header
                    Style::default().fg(app.theme.lavender)
                } else {
                    // Context line
                    Style::default().fg(app.theme.text)
                };
                Line::from(Span::styled(line, style))
            })
            .collect()
    } else if app.files.is_empty() {
        vec![Line::from("No changes in working copy")]
    } else {
        vec![Line::from("Select a file to view diff")]
    };

    // Calculate visible area height (subtract 2 for borders)
    let content_height = area.height.saturating_sub(2) as usize;

    // Calculate scroll offset bounds
    let max_scroll = lines.len().saturating_sub(content_height);
    let scroll_offset = app.diff_scroll_offset.min(max_scroll);

    // Slice lines based on scroll offset
    let visible_lines: Vec<Line> = lines
        .into_iter()
        .skip(scroll_offset)
        .take(content_height)
        .collect();

    let title = if app.current_diff.is_some() && max_scroll > 0 {
        format!(
            "Diff (Shift+J/K to scroll, {}/{})",
            scroll_offset, max_scroll
        )
    } else {
        "Diff".to_string()
    };

    let paragraph = Paragraph::new(visible_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(app.theme.surface1)),
        )
        .style(Style::default().bg(app.theme.base))
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}
