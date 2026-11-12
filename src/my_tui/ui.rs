use ratatui::{Frame, widgets::Block};
use crate::my_tui::app::App;


pub fn draw(frame: &mut Frame, _app: &App) {
    let block = Block::bordered().title(" type-dlp ");
    frame.render_widget(block, frame.area());
}