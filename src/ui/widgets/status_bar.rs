use ratatui::{Frame, layout::Rect, style::Style, widgets::Paragraph};

use crate::app::App;

pub fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status_text = if let Some(ref msg) = app.status_message {
        format!("✓ {}", msg)
    } else {
        "q: quit | hjkl: navigate | d: describe | c: commit | n: new | b: bookmark | f: fetch | p: push | r: rebase | R: refresh".to_string()
    };

    let style = if app.status_message.is_some() {
        Style::default().fg(app.theme.green).bg(app.theme.base)
    } else {
        Style::default().fg(app.theme.subtext0).bg(app.theme.base)
    };

    let status = Paragraph::new(status_text).style(style);

    f.render_widget(status, area);
}
