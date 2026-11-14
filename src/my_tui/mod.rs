//Here is the state of our mod.rs, thats controling the tui;
pub struct App {
    pub should_quit: bool,
}

use ratatui::{
    Frame, crossterm::event::{self, Event, KeyCode}, layout::{Constraint, Layout}, widgets::{Block, Paragraph}
};

pub fn draw(frame: &mut Frame) {

    let [topo, saudacao] = Layout::vertical([
        Constraint::Length(10),
        Constraint::Min(5),
    ]).areas(frame.area());

    let about_app = Paragraph::new(" Aqui podemos comecar a ver o ratatui! ")
        .block(Block::bordered().title(" type-dlp "));

    let other_thing = Paragraph::new(" Its just work? ")
        .block(Block::bordered());

    frame.render_widget(about_app, topo);
    frame.render_widget(other_thing, saudacao);
}

pub fn events_on_keyboard(new_app: &mut App) -> std::io::Result<()> {

    if let Event::Key(new_event) = event::read()? {
        match new_event.code {
            KeyCode::Char('q') => new_app.should_quit = true,
            _ => {}
        }
    }

    Ok(())
}