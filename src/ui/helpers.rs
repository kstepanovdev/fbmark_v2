use anyhow::Result;
use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use tui_input::Input;

use crate::tui::Frame;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn set_cursor(input: &Input, place_to_render: &Rect, f: &mut Frame) -> Result<()> {
    f.set_cursor(
        // Put cursor past the end of the input text
        place_to_render.x + u16::try_from(input.visual_cursor())? + 1,
        // Move one line down, from the border to the input line
        place_to_render.y + 1,
    );
    Ok(())
}
