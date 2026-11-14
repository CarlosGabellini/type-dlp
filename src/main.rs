// The our main for archives;
//Start and initialize;
mod my_tui;

use ratatui::{
    DefaultTerminal
};

use crate::my_tui::{App, draw, events_on_keyboard};

fn run(terminal: &mut DefaultTerminal, new_app: &mut App) -> std::io::Result<()> {
    while !new_app.should_quit {
        terminal.draw(|frame| draw(frame))?;
        events_on_keyboard(new_app)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let mut new_app = App {
        should_quit: false,
    };

    let result = run(&mut terminal, &mut new_app);

    ratatui::restore();
    result
}