use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};

use tui_calc::{app::AppResult, App, Event, EventHandler, Tui};

fn main() -> AppResult<()> {
    if let Some(arg) = std::env::args().nth(1) {
        if ["--version", "-v"].contains(&arg.as_str()) {
            println!("{} v{}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
    }

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => app.handle_key_event(key_event)?, //.unwrap() tui_calc::handler::handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
