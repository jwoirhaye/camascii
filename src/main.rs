mod app;
mod camera;
mod codec;
mod ui;

use app::App;
use camera::Camera;
use crossterm::{
    cursor, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{io, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera = Camera::open()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    while !app.should_quit {
        let frame = camera.capture()?;
        terminal.draw(|f| ui::draw(f, &frame[..]))?;

        if crossterm::event::poll(Duration::from_millis(1))? {
            app.handle_event(crossterm::event::read()?);
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show)?;

    Ok(())
}
