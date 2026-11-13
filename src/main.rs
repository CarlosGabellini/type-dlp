// The our main for archives;
//Start and initialize;
mod my_tui;

use ratatui::{
    DefaultTerminal, 
    crossterm::event::{self, Event::{self, Key}, 
    KeyEventKind}
};

use crate::my_tui::{
    app::App, 
    ui
};

fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();

    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.on_key(key);
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);

    ratatui::restore();
    result
}