use ratatui::widgets::{ListState, TableState};
use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Focus { Url, Presets, Formats, Queue }

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Preset { Best, P1080, P720, AudioMp3, Custom }

pub enum DownloadStatus {
    Waiting,
    Downloading,
    Done,
    Failed(String), // guarda o motivo do erro
}

pub struct VideoInfo {
    pub(super) title: String,
    pub(super) channel: String,
    pub(super) duration: String,
}

pub struct FormatRow {
    pub(super) id: String,
    pub(super) kind: String,
    pub(super) res: String,
    pub(super) fps: String,
    pub(super) codec: String,
    pub(super) size: String,
}

pub struct DownloadItem {
    pub(super) title: String,
    pub(super) status: DownloadStatus,
    pub(super) progress: f32,
    pub(super) speed: String,
    pub(super) eta: String,
}

pub struct App {
    pub should_quit: bool,
    pub focus: Focus,                   //The main enum for our menu;
    pub url_input: String,

    pub video: Option<VideoInfo>,
    pub preset: Preset,
    pub formats: Vec<FormatRow>,
    pub formats_state: TableState,

    pub queue: Vec<DownloadItem>,
    pub queue_state: ListState,

    pub download_dir: PathBuf,
    pub has_ytdlp: bool,
    pub has_ffmpeg: bool,

    pub status: Option<String>, // mensagens e erros no rodapé
    pub loading: bool,          // spinner enquanto busca o -J
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
            formats_state: TableState::default(),
            queue: Vec::new(),
            queue_state: ListState::default(),
            download_dir: dirs::video_dir().unwrap_or_else(|| PathBuf::from(".")),
            has_ytdlp: false,
            has_ffmpeg: false,
            status: None,
            loading: false,
        }
    }

    pub fn next_preset(&mut self) {
        self.preset = match self.preset {
            Preset::Best => Preset::P1080,
            Preset::P1080 => Preset::P720,
            Preset::P720 => Preset::AudioMp3,
            Preset::AudioMp3 => Preset::Custom,
            Preset::Custom => Preset::Best,
        };
    }

    pub fn previous_preset(&mut self) {
        self.preset = match self.preset {
            Preset::Best => Preset::Custom,
            Preset::Custom => Preset::AudioMp3,
            Preset::AudioMp3 => Preset::P720,
            Preset::P720 => Preset::P1080,
            Preset::P1080 => Preset::Best,
        };
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}