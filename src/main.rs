// The our main for archives;
//Start and initialize;
mod my_tui;
mod ytdlp;

use ratatui::{
    DefaultTerminal
};

use crate::my_tui::{App, draw, events_on_keyboard};

fn run(terminal: &mut DefaultTerminal, new_app: &mut App) -> std::io::Result<()> {
    while !new_app.should_quit {
        terminal.draw(|frame| draw(frame, new_app))?;
        events_on_keyboard(new_app)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut my_app = App::new();

    //Context and verify yt-dlp and ffmpeg has been installed
    let context = std::process::Command::new("yt-dlp")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    let context_ffmpeg = std::process::Command::new("ffmpeg")
        .arg("-version")            //ffmpeg needs only '-' and not '--'
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
    
    my_app.has_ytdlp = context;
    my_app.has_ffmpeg = context_ffmpeg;
    
    let result = run(&mut terminal, &mut my_app);

    ratatui::restore();
    result
}