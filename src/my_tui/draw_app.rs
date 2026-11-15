//Logic of draw here;

use ratatui::{Frame, layout::{Constraint, Layout, Rect}, style::Color, widgets::{Block, Paragraph}
};

use crate::my_tui::App;

pub fn draw(frame: &mut Frame, _new_app: &mut App) {

    let [url, video, presets, formats, queue, help, status] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(4),
        Constraint::Length(1),
        Constraint::Min(6),
        Constraint::Length(5),
        Constraint::Length(1),
        Constraint::Length(1),
    ]).areas(frame.area());

    let about_url = Paragraph::new(" ")
        .block(Block::bordered().title(" type-dlp "));

    draw_url(frame, url, _new_app);
}

pub fn draw_url(frame: &mut Frame, area: Rect, app: &App) {
    let new_text = Paragraph::new(app.url_input.as_str())
        .block(Block::bordered().title(" type-dlp ").border_style(Color::Rgb(25, 130, 196)));

    frame.render_widget(new_text, area);
}