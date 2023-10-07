use anyhow::Result;

use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::{
    app::{ActiveWindow, CreationParams},
    tui::Frame,
    ui::helpers::{centered_rect, set_cursor},
};

pub fn collect_creation_tags_items(params: &CreationParams) -> List<'static> {
    let mut tags = Vec::<ListItem>::new();

    for tag in &params.tags_items.items {
        tags.push(ListItem::new(Line::from(Span::styled(
            tag.name.to_string(),
            Style::default().fg(Color::Yellow),
        ))));
    }

    List::new(tags)
        .block(Block::default().title("Tags").borders(Borders::ALL))
        .style(match params.active_window {
            ActiveWindow::Tags => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>")
}

pub fn collect_creation_selected_tags(params: &CreationParams) -> List {
    let mut tags = Vec::<ListItem>::new();

    for tag in &params.selected_tags {
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

pub fn render_creation_popup(params: &CreationParams, f: &mut Frame) -> Result<()> {
    let popup_block = Block::default()
        .title("Create a brand-new bookmark")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(60, 35, f.size());
    // clear underlaying layer first
    f.render_widget(Clear, area);
    f.render_widget(popup_block.clone(), area);

    let h_popup_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let v_inputs_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(5),
        ])
        .split(h_popup_chunks[0]);

    let h_tags_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(h_popup_chunks[1]);

    let title_text = params.title.to_string();
    let title_block = Paragraph::new(title_text)
        .style(match params.active_window {
            ActiveWindow::Title => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Title"));
    f.render_widget(title_block, v_inputs_chunks[0]);

    let url_text = params.link.to_string();
    let url_block = Paragraph::new(url_text)
        .style(match params.active_window {
            ActiveWindow::Link => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Link"));
    f.render_widget(url_block, v_inputs_chunks[1]);

    match params.active_window {
        ActiveWindow::Link => {
            set_cursor(&params.link, &v_inputs_chunks[1], f)?;
        }
        ActiveWindow::Title => {
            set_cursor(&params.title, &v_inputs_chunks[0], f)?;
        }
        ActiveWindow::Tags => {}
    }

    let status_text =
        Paragraph::new(String::new()).block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status_text, v_inputs_chunks[2]);

    let tags_selection = collect_creation_tags_items(params);
    f.render_stateful_widget(
        tags_selection,
        h_tags_chunks[0],
        &mut params.tags_items.state.clone(),
    );

    let selected_tags = collect_creation_selected_tags(params);
    f.render_widget(selected_tags, h_tags_chunks[1]);

    Ok(())
}
