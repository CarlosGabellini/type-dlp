#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Focus { Url, Presets, Formats, Queue }

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Preset { Best, P1080, P720, AudioMp3, Custom }

pub struct VideoInfo { 
    title: String,
    channel: String,
    duration: String,
}

pub struct FormatRow { 
    id: String,
    kind: String,
    res: String,
    fps: String,
    codec: String,
    size: String,
}

pub enum DownloadStatus {
    Waiting,
    Downloading,
    Done,
    Failed,
}

pub struct DownloadItem {
    title: String,
    status: DownloadStatus,
    progress: f32,
    speed: String,
    eta: String,
}

pub struct App {
    pub should_quit: bool,
    pub focus: Focus,
    pub url_input: String,
    
    pub video: Option<VideoInfo>,
    pub preset: Preset,
    pub formats: Vec<FormatRow>,

    pub formats_state: ratatui::widgets::TableState,
    pub queue: Vec<DownloadItem>,
    pub download_dir: String,
    pub has_ytdlp: bool,
    pub has_ffmpeg: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            focus: Focus::Url,
            url_input: String::new(),
            video: None,
            preset: Preset::Best,
            formats: Vec::new(),
            formats_state: ratatui::widgets::TableState::default(),
            queue: Vec::new(),
            download_dir: String::from("~/Videos"),
            has_ytdlp: false,
            has_ffmpeg: false,
        }
    }
}