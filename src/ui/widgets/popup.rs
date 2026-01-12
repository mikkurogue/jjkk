use ratatui::{
    Frame,
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{
        Modifier,
        Style,
    },
    text::{
        Line,
        Span,
    },
    widgets::{
        Block,
        Borders,
        Clear,
        List,
        ListItem,
        Paragraph,
        Wrap,
    },
};

use crate::app::App;

pub fn render_input_popup(
    f: &mut Frame,
    app: &App,
    title: &str,
    content: &str,
    cursor_position: usize,
    area: Rect,
) {
    let popup_area = centered_rect(60, 40, area);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.lavender))
        .style(Style::default().bg(app.theme.surface0));

    // Insert cursor character at cursor position
    let content_with_cursor = if content.is_empty() {
        "█".to_string()
    } else {
        let mut chars: Vec<char> = content.chars().collect();
        // Insert block cursor at position
        if cursor_position >= chars.len() {
            chars.push('█');
        } else {
            chars.insert(cursor_position, '█');
        }
        chars.into_iter().collect()
    };

    let text = vec![
        Line::from(content_with_cursor),
        Line::from(""),
        Line::from(Span::styled(
            "Enter to confirm | Shift+Enter for newline | Esc to cancel",
            Style::default().fg(app.theme.subtext0),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(app.theme.text));

    f.render_widget(Clear, popup_area);
    f.render_widget(paragraph, popup_area);
}

pub fn render_error_popup(f: &mut Frame, app: &App, message: &str, area: Rect) {
    let popup_area = centered_rect(60, 30, area);

    let block = Block::default()
        .title("Error")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.red))
        .style(Style::default().bg(app.theme.surface0));

    let text = vec![
        Line::from(Span::styled(message, Style::default().fg(app.theme.red))),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter or Esc to close",
            Style::default().fg(app.theme.subtext0),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);

    f.render_widget(Clear, popup_area);
    f.render_widget(paragraph, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn render_help_popup(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(80, 80, area);

    let block = Block::default()
        .title("Help - Keybindings")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.lavender))
        .style(Style::default().bg(app.theme.surface0));

    let help_text = vec![
        Line::from(Span::styled(
            "Navigation",
            Style::default()
                .fg(app.theme.blue)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  j/↓         Move down"),
        Line::from("  k/↑         Move up"),
        Line::from("  Shift+J     Scroll diff down"),
        Line::from("  Shift+K     Scroll diff up"),
        Line::from("  1/2/3       Switch to tab 1/2/3"),
        Line::from("  Tab         Next tab"),
        Line::from("  Shift+Tab   Previous tab"),
        Line::from("  Enter       Select/checkout item"),
        Line::from(""),
        Line::from(Span::styled(
            "Working Copy Operations",
            Style::default()
                .fg(app.theme.green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  d           Describe current change"),
        Line::from("  c           Commit working copy"),
        Line::from("  n           Create new commit"),
        Line::from("  R           Refresh status"),
        Line::from("  X           Restore working copy"),
        Line::from(""),
        Line::from(Span::styled(
            "Remote Operations",
            Style::default()
                .fg(app.theme.peach)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  f           Fetch from remote"),
        Line::from("  p           Push to remote"),
        Line::from(""),
        Line::from(Span::styled(
            "Branch/Bookmark Operations",
            Style::default()
                .fg(app.theme.mauve)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  b           Set bookmark"),
        Line::from("  r           Rebase to destination"),
        Line::from(""),
        Line::from(Span::styled(
            "Other",
            Style::default()
                .fg(app.theme.yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  ?           Show this help"),
        Line::from("  q           Quit (or close help)"),
        Line::from(""),
        Line::from(Span::styled(
            "Press '?' or 'q' or Esc to close",
            Style::default().fg(app.theme.subtext0),
        )),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(app.theme.text));

    f.render_widget(Clear, popup_area);
    f.render_widget(paragraph, popup_area);
}

pub fn render_bookmark_select_popup(
    f: &mut Frame,
    app: &App,
    content: &str,
    cursor_position: usize,
    available_bookmarks: &[String],
    selected_index: usize,
    area: Rect,
) {
    let popup_area = centered_rect(60, 60, area);

    let block = Block::default()
        .title("Set Bookmark")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.lavender))
        .style(Style::default().bg(app.theme.surface0));

    // Create layout: input field + suggestions list
    let inner_area = block.inner(popup_area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Input field
            Constraint::Min(5),    // Suggestions list
            Constraint::Length(2), // Help text
        ])
        .split(inner_area);

    // Render the input field with cursor
    let content_with_cursor = if content.is_empty() {
        "█".to_string()
    } else {
        let mut chars: Vec<char> = content.chars().collect();
        if cursor_position >= chars.len() {
            chars.push('█');
        } else {
            chars.insert(cursor_position, '█');
        }
        chars.into_iter().collect()
    };

    let input_text = vec![Line::from(content_with_cursor)];
    let input_paragraph = Paragraph::new(input_text)
        .style(Style::default().fg(app.theme.text))
        .wrap(Wrap { trim: false });

    // Filter bookmarks
    let filtered: Vec<&String> = if content.is_empty() {
        available_bookmarks.iter().collect()
    } else {
        available_bookmarks
            .iter()
            .filter(|b| b.to_lowercase().contains(&content.to_lowercase()))
            .collect()
    };

    // Render suggestions list
    let suggestions: Vec<ListItem> = filtered
        .iter()
        .enumerate()
        .map(|(i, bookmark)| {
            let style = if i == selected_index {
                Style::default()
                    .fg(app.theme.base)
                    .bg(app.theme.lavender)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };
            ListItem::new(format!("  {}", bookmark)).style(style)
        })
        .collect();

    let suggestions_list = List::new(suggestions).style(Style::default().fg(app.theme.text));

    // Help text
    let help = Paragraph::new(vec![Line::from(Span::styled(
        "↑↓/jk: navigate | Tab: autocomplete | Enter: confirm | Esc: cancel",
        Style::default().fg(app.theme.subtext0),
    ))])
    .alignment(Alignment::Center);

    // Render everything
    f.render_widget(Clear, popup_area);
    f.render_widget(block, popup_area);
    f.render_widget(input_paragraph, chunks[0]);
    f.render_widget(suggestions_list, chunks[1]);
    f.render_widget(help, chunks[2]);
}
