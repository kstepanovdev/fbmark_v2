use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{ActiveWindow, App, Mode};

// TODO: unbloat this function
pub fn render_titles_search_panel(app: &App) -> Paragraph {
    let block = match &app.mode {
        Mode::Search(searhing_params) => {
            let text = searhing_params.title.to_string();
            Paragraph::new(text)
                .style(match searhing_params.active_window {
                    ActiveWindow::Title => Style::default().fg(Color::Yellow),
                    _ => Style::default(),
                })
                .block(Block::default().borders(Borders::ALL).title("Titles"))
        }
        _ => Paragraph::new("")
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Titles")),
    };

    block
}

pub fn render_links_search_panel(app: &App) -> Paragraph {
    let block = match &app.mode {
        Mode::Search(searhing_params) => {
            let text = searhing_params.link.to_string();
            Paragraph::new(text)
                .style(match searhing_params.active_window {
                    ActiveWindow::Link => Style::default().fg(Color::Yellow),
                    _ => Style::default(),
                })
                .block(Block::default().borders(Borders::ALL).title("Links"))
        }
        _ => Paragraph::new("")
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Links")),
    };

    block
}

pub fn collect_tags_items(app: &App) -> List<'static> {
    let mut tags = Vec::<ListItem>::new();

    for tag in &app.tags_items.items {
        tags.push(ListItem::new(Line::from(Span::styled(
            tag.name.to_string(),
            Style::default().fg(Color::Yellow),
        ))));
    }

    List::new(tags)
        .block(Block::default().title("Tags").borders(Borders::ALL))
        .style(if let Mode::Search(params) = &app.mode {
            match params.active_window {
                ActiveWindow::Tags => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            }
        } else {
            Style::default()
        })
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>")
}

pub fn collect_selected_tags_items(app: &App) -> List {
    let mut tags = Vec::<ListItem>::new();

    for tag in &app.tags_filter {
        tags.push(ListItem::new(Line::from(Span::styled(
            tag.name.to_string(),
            Style::default().fg(Color::Yellow),
        ))));
    }

    List::new(tags)
        .block(
            Block::default()
                .title("Selected Tags")
                .borders(Borders::ALL),
        )
        .style(Style::default())
}
