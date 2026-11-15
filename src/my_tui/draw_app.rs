//Logic of draw here;

use ratatui::{Frame, 
    layout::{Constraint, Layout}, 
    widgets::Paragraph
};

use crate::my_tui::App;

pub fn draw(frame: &mut Frame, new_app: &mut App) {

    let [url, video, presets, formats, queue, help, status] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(4),
        Constraint::Length(1),
        Constraint::Min(6),
        Constraint::Length(5),
        Constraint::Length(1),
        Constraint::Length(1),
    ]).areas(frame.area());

    frame.render_widget(Paragraph::new(""), url);
}