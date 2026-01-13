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

use crate::{
    app::App,
    jj::log,
};

pub fn render_log(f: &mut Frame, app: &mut App, area: Rect) {
    // Get log with configured limit
    let limit = app.settings.ui.log_commits_count;

    let commits = match log::get_log(limit) {
        Ok(c) => c,
        Err(e) => {
            let error_text = format!("Failed to get log: {e}");
            let paragraph = Paragraph::new(error_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Log")
                        .border_style(Style::default().fg(app.theme.surface1)),
                )
                .style(Style::default().fg(app.theme.red).bg(app.theme.base));
            f.render_widget(paragraph, area);
            return;
        }
    };

    if commits.is_empty() {
        let paragraph = Paragraph::new("No commits found.")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Log")
                    .border_style(Style::default().fg(app.theme.surface1)),
            )
            .style(Style::default().fg(app.theme.subtext0).bg(app.theme.base));
        f.render_widget(paragraph, area);
        return;
    }

    // Create list items
    let items: Vec<ListItem> = commits
        .iter()
        .enumerate()
        .map(|(i, commit)| {
            let is_selected = i == app.selected_log_index;

            let change_style = if is_selected {
                Style::default()
                    .fg(app.theme.blue)
                    .bg(app.theme.surface1)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.blue)
            };

            let desc_style = if is_selected {
                Style::default()
                    .fg(app.theme.text)
                    .bg(app.theme.surface1)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.text)
            };

            let author_style = if is_selected {
                Style::default()
                    .fg(app.theme.subtext0)
                    .bg(app.theme.surface1)
            } else {
                Style::default().fg(app.theme.subtext0)
            };

            let content = vec![
                Span::styled(&commit.change_id, change_style),
                Span::raw(" "),
                Span::styled(&commit.description, desc_style),
                Span::raw(" "),
                Span::styled(&commit.author, author_style),
            ];

            ListItem::new(Line::from(content))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Log (last {limit} commits, j/k to navigate)"))
                .border_style(Style::default().fg(app.theme.surface1)),
        )
        .style(Style::default().bg(app.theme.base))
        .highlight_style(
            Style::default()
                .bg(app.theme.surface1)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut app.log_list_state);
}
