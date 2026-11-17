//Events on keyboard here;

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::my_tui::{App, app::{Focus, FormatRow, VideoInfo}};

///The main event loop for keyboard input control.
pub fn events_on_keyboard(new_app: &mut App) -> std::io::Result<()> {

    if let Event::Key(new_event) = event::read()? {
        match (new_app.focus, new_event.code) {
            (Focus::Url, KeyCode::Char(c)) => { new_app.url_input.push(c); },
            (Focus::Url, KeyCode::Backspace) => { new_app.url_input.pop(); },

            (Focus::Url, KeyCode::Enter) => { 
                if new_app.url_input.trim().is_empty() {
                    new_app.status = Some("Type URL before continue!".to_string());
                    
                } else if !new_app.has_ytdlp {
                    new_app.status = Some("yt-dlp has not found!".to_string());

                } else {
                    new_app.status = None;
                    new_app.video = Some(VideoInfo {
                        title: "Rust TUI".to_string(),
                        channel: "No boilepart".to_string(),
                        duration: "24:15".to_string(),
                    });

                    new_app.formats = vec![
                        FormatRow { id: "137+140".into(), kind: "v+a".into(), res: "1920x1080".into(), fps: "30".into(), codec: "avc1+aac".into(), size: "~210 MiB".into() },
                        FormatRow { id: "248+251".into(), kind: "v+a".into(), res: "1920x1080".into(), fps: "30".into(), codec: "vp9+opus".into(), size: "~160 MiB".into() },
                        FormatRow { id: "22".into(), kind: "v+a".into(), res: "1280x720".into(), fps: "30".into(), codec: "avc1+aac".into(), size: "~140 MiB".into() },
                        FormatRow { id: "251".into(), kind: "audio".into(), res: "—".into(), fps: "—".into(), codec: "opus".into(), size: "~14 MiB".into() },
                    ];
                    
                    new_app.formats_state.select(Some(0));
                }
            },

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

            (Focus::Formats, KeyCode::Down) => {
                new_app.formats_state.select_next();
            }

            (Focus::Formats, KeyCode::Up) => {
                new_app.formats_state.select_previous();
            }
            (_, KeyCode::Esc) => new_app.should_quit = true,
            _ => {}
        }
    }

    Ok(())
}  