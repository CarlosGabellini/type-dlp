//Events on keyboard here;

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::my_tui::App;

pub fn events_on_keyboard(new_app: &mut App) -> std::io::Result<()> {

    if let Event::Key(new_event) = event::read()? {
        match new_event.code {
            KeyCode::Char('q') => new_app.should_quit = true,
            _ => {}
        }
    }
    Ok(())
}