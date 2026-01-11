use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
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
            "Ctrl+Enter to confirm | Esc to cancel",
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
