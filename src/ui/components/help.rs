use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{tui::Frame, ui::helpers::centered_rect};

pub fn render_help_popup(f: &mut Frame) {
    let popup_block = Block::default()
        .title("Help panel")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(90, 50, f.size());
    // Clear underlaying layer
    f.render_widget(Clear, area);
    f.render_widget(popup_block, area);

    let v_popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    let h_popup_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(v_popup_chunks[0]);

    // render help for Search block
    let search_block = Block::default()
        .title("Searching mode")
        .borders(Borders::ALL);
    let search_text = vec![
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" Tab ", Style::new().yellow().italic()),
            Span::raw("to change active window"),
        ]),
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" Enter ", Style::new().yellow().italic()),
            Span::raw("in Tags window to filter bookmarks by the tag"),
        ]),
    ];
    let search_panel = Paragraph::new(search_text)
        .block(search_block)
        .style(Style::default());
    f.render_widget(search_panel, h_popup_chunks[0]);

    // render help for Scrolling block
    let scrolling_block = Block::default()
        .title("Scrolling mode")
        .borders(Borders::ALL);
    let scrolling_text = vec![
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" Up/Down ", Style::new().yellow().italic()),
            Span::raw("to navigate"),
        ]),
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" Enter ", Style::new().yellow().italic()),
            Span::raw("to open a bookmark in xdg-selectd browser"),
        ]),
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" Del ", Style::new().yellow().italic()),
            Span::raw("to delete highlighted bookmark"),
        ]),
    ];
    let scrolling_panel = Paragraph::new(scrolling_text)
        .block(scrolling_block)
        .style(Style::default());
    f.render_widget(scrolling_panel, h_popup_chunks[1]);

    // render help for Creation block
    let creation_block = Block::default()
        .title("Creation mode")
        .borders(Borders::ALL);
    let creation_text = vec![
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" Tab ", Style::new().yellow().italic()),
            Span::raw("to change active window"),
        ]),
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" Enter ", Style::new().yellow().italic()),
            Span::raw("to create a bookmark"),
        ]),
    ];
    let creation_panel = Paragraph::new(creation_text)
        .block(creation_block)
        .style(Style::default());
    f.render_widget(creation_panel, h_popup_chunks[2]);

    // general help
    let general_block = Block::default()
        .title("General controls")
        .borders(Borders::ALL);
    let general_text = vec![
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" Backquote (`) ", Style::new().yellow().italic()),
            Span::raw("to change active mode"),
        ]),
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" Escape ", Style::new().yellow().italic()),
            Span::raw("to close the app"),
        ]),
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" F5 ", Style::new().yellow().italic()),
            Span::raw("to sync bookmarks"),
        ]),
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" F1 ", Style::new().yellow().italic()),
            Span::raw("to close/reopen this window"),
        ]),
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" F3 ", Style::new().yellow().italic()),
            Span::raw("to switch to creation mode"),
        ]),
        Line::from(vec![
            Span::raw("Press"),
            Span::styled(" F12 ", Style::new().yellow().italic()),
            Span::raw("to reset searching state"),
        ]),
    ];
    let general_panel = Paragraph::new(general_text)
        .block(general_block)
        .style(Style::default());
    f.render_widget(general_panel, v_popup_chunks[1]);
}
