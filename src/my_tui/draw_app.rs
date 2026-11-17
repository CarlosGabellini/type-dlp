//Logic of draw here;

use ratatui::{
    Frame, 
    layout::{
        Constraint, Layout, Rect,
    }, 

    style::Color,
    widgets::{Block, Paragraph},
};

use crate::my_tui::{App, app::{Focus::{self}, Preset::{self}}};

/* //----------------------------------- Color scale for our map --------------------------------------------
        Our application will use a color scale to make it more customized; here is the code I should use 
    for the requested color scale:
    Vibrant_Coral = 255, 89, 94     (RGB)
    Golden_Pollen = 255, 202, 58
    Yellow_Green  = 138, 201, 38
    Steel_Blue    = 25, 130, 196
    Dusty_Grape   = 106, 76, 147

        These are the colors we will use for our terminal—not necessarily in this exact order across all 
    elements, but this will be the primary color scale!
    //------------------------------------------------------------------------------------------------------
 */

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

    draw_url(frame, url, _new_app);
    video_draw(frame, video, _new_app);
    draw_presets(frame, presets, _new_app);
    formats_draw(frame, formats, _new_app);
    queue_draw_information(frame, queue, _new_app);

    let status_line = match (_new_app.has_ytdlp, _new_app.has_ffmpeg) {
        (true, true) => "yt-dlp: [X] \tffmpeg: [X]",
        (true, false) => "yt-dlp: [X] \tffmpeg: [ ]",
        (false, true) => "yt-dlp: [ ] \tffmpeg: [X]",
        (false, false) => "yt-dlp: [ ] \tffmpeg: [ ]",
    };

    let status_text = Paragraph::new(status_line);

    frame.render_widget(status_text, status);
}

fn draw_url(frame: &mut Frame, area: Rect, app: &App) {
    
    //Using this if statement to determine the border color based on focus
    let border_color = if app.focus == Focus::Url {
        Color::Rgb(25, 130, 196)
    } else {
        Color::Rgb(106, 130, 196)
    };

    let new_text = Paragraph::new(app.url_input.as_str())
        .block(Block::bordered().title(" type-dlp ").border_style(border_color));

    frame.render_widget(new_text, area);

    if app.focus == Focus::Url {
        let x_cursor = area.x + 1 + app.url_input.chars().count() as u16;
        let y_cursor = area.y + 1;

        frame.set_cursor_position((x_cursor, y_cursor));
    }
}

fn draw_presets(frame: &mut Frame, area: Rect, app: &App) {
    let border_draw_presets = if app.focus == Focus::Presets {
        Color::Rgb(25, 130, 196)
    } else {
        Color::Rgb(106, 130, 196)
    };

    //We calling the auxiliar function for define the variables;
    let best = preset_label(app.preset, Preset::Best, "Best");
    let p1080 = preset_label(app.preset, Preset::P1080, "P1080");
    let p720 = preset_label(app.preset, Preset::P720, "P720");
    let audio_mp3 = preset_label(app.preset, Preset::AudioMp3, "Audio MP3");
    let custom = preset_label(app.preset, Preset::Custom, "Custom");

    //Here, the presets are together for only line;
    let line = format!("Preset {}  {}  {}  {}  {}", best, p1080, p720, audio_mp3, custom);

    let new_text_presets = Paragraph::new(line.as_str());
    
    frame.render_widget(new_text_presets, area);
}

fn preset_label(current: Preset, target: Preset, text: &str) -> String {
    match current == target {
        true => format!("[ {} ]", text),
        false => text.to_string(),
    }
}

fn video_draw(frame: &mut Frame, area: Rect, app: &App) {
    let video_border = Color::Rgb(138, 201, 38);
    
    let text_view = Paragraph::new(" New test from here! ")
        .block(Block::bordered().title(" video-information ").border_style(video_border));
    
    frame.render_widget(text_view, area);
}

fn formats_draw(frame: &mut Frame, area: Rect, app: &App) {
    let border_draw_formats = if app.focus == Focus::Formats {
        Color::Rgb(25, 130, 196)
    } else {
        Color::Rgb(106, 130, 196)
    };

    let text_view_formats = Paragraph::new(" Formats are here! ")
        .block(Block::bordered().title(" formats ").border_style(border_draw_formats));

    frame.render_widget(text_view_formats, area);
}

fn queue_draw_information(frame: &mut Frame, area: Rect, app: &App) {
    let borders_draw_queue = if app.focus == Focus::Queue {
        Color::Rgb(25, 130, 196)
    } else {
        Color::Rgb(106, 130, 196)
    };

    let text_view_queue = Paragraph::new(" Queue information here! ")
        .block(Block::bordered().title(" queue ").border_style(borders_draw_queue));

    frame.render_widget(text_view_queue, area);
}