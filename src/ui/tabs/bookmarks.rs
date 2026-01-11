use ratatui::{
    Frame,
    layout::Rect,
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
        List,
        ListItem,
        Paragraph,
    },
};

use crate::app::App;

pub fn render_bookmarks(f: &mut Frame, app: &App, area: Rect) {
    // Get bookmarks
    let bookmarks = match crate::jj::operations::get_bookmarks() {
        Ok(b) => b,
        Err(e) => {
            let error_text = format!("Failed to get bookmarks: {e}");
            let paragraph = Paragraph::new(error_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Bookmarks")
                        .border_style(Style::default().fg(app.theme.surface1)),
                )
                .style(Style::default().fg(app.theme.red).bg(app.theme.base));
            f.render_widget(paragraph, area);
            return;
        }
    };

    if bookmarks.is_empty() {
        let paragraph = Paragraph::new("No bookmarks found.\nPress 'b' to create one.")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Bookmarks")
                    .border_style(Style::default().fg(app.theme.surface1)),
            )
            .style(Style::default().fg(app.theme.subtext0).bg(app.theme.base));
        f.render_widget(paragraph, area);
        return;
    }

    // Create list items
    let items: Vec<ListItem> = bookmarks
        .iter()
        .enumerate()
        .map(|(i, bookmark)| {
            let is_selected = i == app.selected_bookmark_index;
            let style = if is_selected {
                Style::default()
                    .fg(app.theme.text)
                    .bg(app.theme.surface1)
                    .add_modifier(Modifier::BOLD)
            } else if bookmark.is_current {
                Style::default()
                    .fg(app.theme.lavender)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };

            let prefix = if bookmark.is_current { "* " } else { "  " };
            let content = format!("{}{}", prefix, bookmark.name);

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Bookmarks (* = current, j/k to navigate, Enter to checkout)")
                .border_style(Style::default().fg(app.theme.surface1)),
        )
        .style(Style::default().bg(app.theme.base));

    f.render_widget(list, area);
}
