use crossterm::event::{Event, KeyCode, KeyModifiers};

pub struct App {
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                _ => {}
            }
        }
    }
}
