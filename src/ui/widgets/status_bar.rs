use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::Paragraph,
};

use crate::app::App;

pub fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status_text = if let Some(loading_msg) = &app.loading_message {
        // Show loading spinner with message
        format!("{} {}", app.get_spinner_char(), loading_msg)
    } else if let Some(msg) = &app.status_message {
        // Show success message
        format!("✓ {msg}")
    } else {
        // Show default keybinds
        "q: quit | hjkl: navigate | f: fetch | p: push | r: rebase  | d: describe | b: bookmark | X: restore | R: refresh".to_string()
    };

    let style = if app.loading_message.is_some() {
        Style::default().fg(app.theme.yellow).bg(app.theme.base)
    } else if app.status_message.is_some() {
        Style::default().fg(app.theme.green).bg(app.theme.base)
    } else {
        Style::default().fg(app.theme.subtext0).bg(app.theme.base)
    };

    let status = Paragraph::new(status_text).style(style);

    f.render_widget(status, area);
}
