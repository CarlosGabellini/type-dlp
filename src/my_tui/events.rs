//Events on keyboard here;

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::my_tui::{App, app::{Focus}};

///The main event loop for keyboard input control.
pub fn events_on_keyboard(new_app: &mut App) -> std::io::Result<()> {

    if let Event::Key(new_event) = event::read()? {
        match (new_app.focus, new_event.code) {
            (Focus::Url, KeyCode::Char(c)) => { new_app.url_input.push(c); },
            (Focus::Url, KeyCode::Backspace) => { new_app.url_input.pop(); },

            (_, KeyCode::Tab) =>  {
                match new_app.focus {
                    Focus::Url => new_app.focus = Focus::Presets,
                    Focus::Presets => new_app.focus = Focus::Formats,
                    Focus::Formats => new_app.focus = Focus::Queue,
                    Focus::Queue => new_app.focus = Focus::Url,
                }
            }

            //Inverse for backTab;
            (_, KeyCode::BackTab) => {
                match new_app.focus {
                    Focus::Url => new_app.focus = Focus::Queue,
                    Focus::Presets => new_app.focus = Focus::Url,
                    Focus::Formats => new_app.focus = Focus::Presets,
                    Focus::Queue => new_app.focus = Focus::Formats,
                }
            }

            (Focus::Presets, KeyCode::Right) => new_app.next_preset(),
            (Focus::Presets, KeyCode::Left) => new_app.previous_preset(),

            (_, KeyCode::Esc) => new_app.should_quit = true,
            _ => {}
        }
    }

    Ok(())
}  