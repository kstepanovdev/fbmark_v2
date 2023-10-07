use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{ActiveWindow, App, Mode};

pub async fn update(app: &mut App, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => app.quit(),
        KeyCode::F(1) => {
            app.toggle_help_render();
        }
        KeyCode::F(3) => {
            app.toggle_creation_mode();
        }
        KeyCode::F(5) => {
            app.sync_bmarks().await?;
        }
        KeyCode::F(12) => {
            app.reset().await?;
        }
        KeyCode::Left => match &mut app.mode {
            Mode::Search(params) => {
                if let ActiveWindow::Tags = params.active_window {
                    app.tags_items.unselect();
                } else {
                    app.add_char(key_event)?;
                };
            }
            Mode::Scrolling => app.bookmarks_items.unselect(),
            Mode::Create(_) => {}
        },
        KeyCode::Down => match &mut app.mode {
            Mode::Search(params) => {
                if let ActiveWindow::Tags = params.active_window {
                    app.tags_items.next();
                };
            }
            Mode::Scrolling => app.bookmarks_items.next(),
            Mode::Create(params) => {
                if let ActiveWindow::Tags = params.active_window {
                    params.tags_items.next();
                };
            }
        },
        KeyCode::Up => match &mut app.mode {
            Mode::Search(params) => {
                if let ActiveWindow::Tags = params.active_window {
                    app.tags_items.previous();
                };
            }
            Mode::Scrolling => app.bookmarks_items.previous(),
            Mode::Create(params) => {
                if let ActiveWindow::Tags = params.active_window {
                    params.tags_items.previous();
                };
            }
        },
        KeyCode::Enter => app.resolve_enter().await?,
        KeyCode::Delete => app.on_delete().await?,
        KeyCode::Char('`') => app.toggle_mode(),
        KeyCode::Tab => app.change_active_window(),
        _ => app.add_char(key_event)?,
    };
    Ok(())
}
