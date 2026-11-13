//Logic of our archive, here is the draw;

use ratatui::{
    Frame, 
    layout::{Constraint, Layout}, 
    widgets::{Block, Paragraph}
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