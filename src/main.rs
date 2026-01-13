//! jjkk - A terminal UI for the jj version control system

mod app;
mod config;
mod jj;
mod ui;

use std::io;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
    },
    execute,
    terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen,
        disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{
    Terminal,
    backend::{
        Backend,
        CrosstermBackend,
    },
};
use ui::layout::render_ui;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new()?;

    // Load initial status
    app.refresh_status()?;

    // Run the application
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()>
where
    <B as Backend>::Error: Send + Sync + 'static,
{
    loop {
        app.update_status_message_timeout();

        // Only draw if needed or when loading spinner is active
        if app.needs_redraw || app.loading_message.is_some() {
            terminal.draw(|f| render_ui(f, app))?;
            app.needs_redraw = false;
        }

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key_event(key)?;
                app.needs_redraw = true; // Mark for redraw after handling input
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
