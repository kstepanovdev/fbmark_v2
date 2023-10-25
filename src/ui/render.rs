use anyhow::Result;

use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::{ActiveWindow, App, Mode},
    tui::Frame,
    ui::components::{
        scrolling::collect_list_items,
        search::{
            collect_selected_tags_items, collect_tags_items, render_links_search_panel,
            render_titles_search_panel,
        },
    },
};

use super::{
    components::{create::render_creation_popup, help::render_help_popup},
    helpers::set_cursor,
};

pub fn render(app: &mut App, f: &mut Frame) -> Result<()> {
    let main_window_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(15),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(f.size());

    let results_panel_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(main_window_layout[0]);

    let search_panel_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_window_layout[1]);

    let title_and_link_panels = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(search_panel_layout[0]);

    let tags_panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(search_panel_layout[1]);

    let results_list = collect_list_items(app);
    for (i, list) in results_list.into_iter().enumerate() {
        f.render_stateful_widget(
            list,
            results_panel_layout[i],
            &mut app.bookmarks_items.state,
        );
    }

    let titles_search = render_titles_search_panel(app);
    f.render_widget(titles_search, title_and_link_panels[0]);

    let links_search = render_links_search_panel(app);
    f.render_widget(links_search, title_and_link_panels[1]);

    let tags_search = collect_tags_items(app);
    f.render_stateful_widget(tags_search, tags_panels[0], &mut app.tags_items.state);

    let selected_tags = collect_selected_tags_items(app);
    f.render_widget(selected_tags, tags_panels[1]);

    match &app.mode {
        Mode::Search(params) => match params.active_window {
            ActiveWindow::Link => {
                set_cursor(&params.link, &title_and_link_panels[1], f)?;
            }
            ActiveWindow::Title => {
                set_cursor(&params.title, &title_and_link_panels[0], f)?;
            }
            ActiveWindow::Tags => {}
        },
        Mode::Create(_) | Mode::Scrolling => {}
    }

    let mode_name = match app.mode {
        Mode::Search(_) => "Searching Mode",
        Mode::Create(_) => "Creation Mode",
        Mode::Scrolling => "Scrolling Mode",
    };
    let mode_footer = Paragraph::new(Line::from(Span::styled(
        mode_name,
        Style::default().fg(Color::Green),
    )))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(mode_footer, main_window_layout[2]);

    if let Mode::Create(params) = &app.mode {
        render_creation_popup(params, f)?;
    }

    if app.render_help {
        render_help_popup(f);
    }

    Ok(())
}
