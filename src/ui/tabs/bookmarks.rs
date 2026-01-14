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

pub fn render_bookmarks(f: &mut Frame, app: &mut App, area: Rect) {
    // Use cached bookmarks data
    let bookmarks = &app.bookmarks;

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
        .style(Style::default().bg(app.theme.base))
        .highlight_style(
            Style::default()
                .bg(app.theme.surface1)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut app.bookmark_list_state);
}
