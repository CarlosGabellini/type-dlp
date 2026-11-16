//Events on keyboard here;

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::my_tui::{App, app::{Focus, Preset}};

///The main event loop for keyboard input control.
pub fn events_on_keyboard(new_app: &mut App) -> std::io::Result<()> {

    if let Event::Key(new_event) = event::read()? {
        match new_event.code {
            KeyCode::Tab =>  {
                match new_app.focus {
                    Focus::Url => new_app.focus = Focus::Presets,
                    Focus::Presets => new_app.focus = Focus::Formats,
                    Focus::Formats => new_app.focus = Focus::Queue,
                    Focus::Queue => new_app.focus = Focus::Url,
                }
            }

            //Inverse for backTab;
            KeyCode::BackTab => {
                match new_app.focus {
                    Focus::Url => new_app.focus = Focus::Queue,
                    Focus::Presets => new_app.focus = Focus::Url,
                    Focus::Formats => new_app.focus = Focus::Presets,
                    Focus::Queue => new_app.focus = Focus::Formats,
                }
            }
            
            KeyCode::Char('q') => new_app.should_quit = true,
            _ => {}
        }
    }

    Ok(())
}   