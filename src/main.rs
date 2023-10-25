pub mod adapters;
pub mod app;
pub mod event;
pub mod models;
pub mod settings;
pub mod tui;
pub mod ui;
pub mod update;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

#[tokio::main]
async fn main() -> Result<()> {
    initialize_panic_handler();
    let mut app = App::new().await?;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while !app.is_should_quit() {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick | Event::Mouse(_) | Event::Resize(_, _) => {}
            Event::Key(key_event) => update(&mut app, key_event).await?,
        };
    }

    tui.exit()?;
    Ok(())
}

pub fn initialize_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        better_panic::Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .verbosity(better_panic::Verbosity::Full)
            .create_panic_handler()(panic_info);
        std::process::exit(libc::EXIT_FAILURE);
    }));
}
