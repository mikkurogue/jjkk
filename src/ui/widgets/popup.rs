use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::app::App;

pub fn render_input_popup(f: &mut Frame, app: &App, title: &str, content: &str, area: Rect) {
    let popup_area = centered_rect(60, 40, area);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.lavender))
        .style(Style::default().bg(app.theme.surface0));

    let text = vec![
        Line::from(content),
        Line::from(""),
        Line::from(Span::styled(
            "Enter to confirm | Esc to cancel",
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
