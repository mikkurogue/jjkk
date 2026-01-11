use ratatui::{
    Frame,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{
        Color,
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
        Wrap,
    },
};
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    parsing::SyntaxSet,
};

use crate::{
    app::App,
    jj::repo::ChangeType,
};

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
    let lines: Vec<Line> = app.current_diff.as_ref().map_or_else(
        || {
            if app.files.is_empty() {
                vec![Line::from("No changes in working copy")]
            } else {
                vec![Line::from("Select a file to view diff")]
            }
        },
        |diff| {
            // Get file extension for syntax detection
            let file_path = app
                .files
                .get(app.selected_file_index)
                .map(|f| f.path.as_str());

            // Initialize syntect
            let ps = SyntaxSet::load_defaults_newlines();
            let ts = ThemeSet::load_defaults();
            let theme = &ts.themes["base16-ocean.dark"];

            // // Try to detect syntax from file path
            let syntax = file_path
                .and_then(|path| ps.find_syntax_for_file(path).ok().flatten())
                .or_else(|| Some(ps.find_syntax_plain_text()));

            // Parse diff and apply syntax highlighting
            diff.lines()
                .map(|line| {
                    // Check for diff-specific lines first
                    if line.starts_with("+++") || line.starts_with("---") {
                        // File headers
                        Line::from(Span::styled(line, Style::default().fg(app.theme.lavender)))
                    } else if line.starts_with("@@") {
                        // Hunk header
                        Line::from(Span::styled(
                            line,
                            Style::default()
                                .fg(app.theme.blue)
                                .add_modifier(Modifier::BOLD),
                        ))
                    } else if line.starts_with("diff ") || line.starts_with("index ") {
                        // Diff header
                        Line::from(Span::styled(line, Style::default().fg(app.theme.lavender)))
                    } else if let Some(content) = line.strip_prefix('+') {
                        // Added line - apply syntax highlighting to the content (skip the + prefix)
                        syntax.map_or_else(
                            || Line::from(Span::styled(line, Style::default().fg(app.theme.green))),
                            |syntax| {
                                let mut h = HighlightLines::new(syntax, theme);
                                let ranges = h.highlight_line(content, &ps).unwrap_or_default();
                                let spans: Vec<Span> = std::iter::once(Span::styled(
                                    "+",
                                    Style::default().fg(app.theme.green),
                                ))
                                .chain(ranges.into_iter().map(|(style, text)| {
                                    let color = syntect_to_ratatui_color(style.foreground);
                                    Span::styled(text, Style::default().fg(color))
                                }))
                                .collect();
                                Line::from(spans).style(Style::default().fg(app.theme.green))
                            },
                        )
                    } else if let Some(content) = line.strip_prefix('-') {
                        // Removed line - apply syntax highlighting to the content (skip the -
                        // prefix)

                        syntax.map_or_else(
                            || Line::from(Span::styled(line, Style::default().fg(app.theme.red))),
                            |syntax| {
                                let mut h = HighlightLines::new(syntax, theme);
                                let ranges = h.highlight_line(content, &ps).unwrap_or_default();
                                let spans: Vec<Span> = std::iter::once(Span::styled(
                                    "-",
                                    Style::default().fg(app.theme.red),
                                ))
                                .chain(ranges.into_iter().map(|(style, text)| {
                                    let color = syntect_to_ratatui_color(style.foreground);
                                    Span::styled(text, Style::default().fg(color))
                                }))
                                .collect();
                                Line::from(spans).style(Style::default().fg(app.theme.red))
                            },
                        )
                    } else {
                        // Context line - apply syntax highlighting
                        syntax.map_or_else(
                            || Line::from(Span::styled(line, Style::default().fg(app.theme.text))),
                            |syntax| {
                                let mut h = HighlightLines::new(syntax, theme);
                                let ranges = h.highlight_line(line, &ps).unwrap_or_default();
                                let spans: Vec<Span> = ranges
                                    .into_iter()
                                    .map(|(style, text)| {
                                        let color = syntect_to_ratatui_color(style.foreground);
                                        Span::styled(text, Style::default().fg(color))
                                    })
                                    .collect();
                                Line::from(spans)
                            },
                        )
                    }
                })
                .collect()
        },
    );

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
        format!("Diff (Shift+J/K to scroll, {scroll_offset}/{max_scroll})")
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

// Helper function to convert syntect color to ratatui color
const fn syntect_to_ratatui_color(color: syntect::highlighting::Color) -> Color {
    Color::Rgb(color.r, color.g, color.b)
}
