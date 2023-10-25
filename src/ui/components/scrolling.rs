use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;

pub fn collect_list_items(app: &App) -> Vec<List<'static>> {
    let bookmarks = app.bookmarks_items.items.clone();
    let titles_header = format!(
        "Titles: {}/{}",
        app.bookmarks_items.items.len(),
        app.bookmarks.len()
    );

    let mut titles = Vec::<ListItem>::new();
    let mut links = Vec::<ListItem>::new();
    let mut tags = Vec::<ListItem>::new();

    for bookmark in bookmarks {
        // titles
        let title = bookmark.title.unwrap_or(String::new());
        titles.push(ListItem::new(Line::from(Span::styled(
            title.to_string(),
            Style::default().fg(Color::Yellow),
        ))));

        // links
        let url: String = bookmark.url.into();
        links.push(ListItem::new(Line::from(Span::styled(
            url.to_string(),
            Style::default().fg(Color::Yellow),
        ))));

        // tags
        let parsed_tags = bookmark
            .tags
            .unwrap_or(vec![])
            .iter()
            .map(|tag| tag.name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        tags.push(ListItem::new(Line::from(Span::styled(
            parsed_tags.to_string(),
            Style::default().fg(Color::Yellow),
        ))));
    }

    vec![
        List::new(titles)
            .block(Block::default().title(titles_header).borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">>"),
        List::new(links)
            .block(Block::default().title("Links").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">>"),
        List::new(tags)
            .block(Block::default().title("Tags").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">>"),
    ]
}
