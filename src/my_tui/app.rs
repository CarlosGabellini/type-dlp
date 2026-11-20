use ratatui::widgets::{ListState, TableState};
use std::{path::PathBuf, sync::mpsc::Receiver};

use crate::ytdlp;

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
    pub title: String,
    pub channel: String,
    pub duration: String,
}

pub struct FormatRow {
    pub id: String,
    pub kind: String,
    pub res: String,
    pub fps: String,
    pub codec: String,
    pub size: String,
}

pub struct DownloadItem {
    pub title: String,
    pub url: String,
    pub args: Vec<String>,
    pub status: DownloadStatus,
    pub progress: f32,
    pub speed: String,
    pub eta: String,
}

pub struct App {
    pub should_quit: bool,
    pub focus: Focus,                   //The main enum for our menu;
    pub url_input: String,

    pub video: Option<VideoInfo>,
    pub preset: Preset,
    pub formats: Vec<FormatRow>,
    pub formats_state: TableState,
    pub video_url: Option<String>,

    pub queue: Vec<DownloadItem>,
    pub queue_state: ListState,

    pub download_dir: PathBuf,
    pub has_ytdlp: bool,
    pub has_ffmpeg: bool,

    pub status: Option<String>,
    pub loading: bool,
    pub video_result: Option<Receiver<Result<ytdlp::YtDlpOutput, String>>>,

    pub download_rx: Option<Receiver<Result<(), String>>>,
    pub current_download: Option<usize>,    
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
            video_result: None,
            video_url: None,
            current_download: None,
            download_rx: None,
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

    pub fn start_fetch(&mut self, url: String) {
        use std::sync::mpsc::channel;
        use std::thread;
    
        let (tx, rx) = channel();
        self.video_url = Some(url.clone());
    
        thread::spawn(move || {
            let result = ytdlp::fetch_video_info(&url);
            let _ = tx.send(result);
        });
    
        self.video_result = Some(rx);
        self.loading = true;
        self.status = None;
    }

    pub fn pool_fetch_result(&mut self) {
        use std::sync::mpsc::TryRecvError;

        let Some(rx) = &self.video_result else {
            return;
        };

        match rx.try_recv() {
            Ok(Ok(info)) => {
                self.video = Some(VideoInfo {
                    title: info.title.clone(),
                    channel: info.uploader.clone().unwrap_or_default(),
                    duration: ytdlp::format_duration(info.duration.unwrap_or(0.0)),
                });

                let filtered = ytdlp::filter_popular(info.formats);
                self.formats = filtered.iter().map(ytdlp::to_format_row).collect();
                self.formats_state.select(Some(0));

                self.loading = false;
                self.video_result = None;
            },

            Ok(Err(e)) => {
                self.status = Some(e);
                self.loading = false;
                self.video_result = None;
            }

            Err(TryRecvError::Empty) => {
                //Still not finish, keep waiting the next frame;
            }

            Err(TryRecvError::Disconnected) => {
                self.status = Some("Unexpected error: channel disconnected".to_string());
                self.loading = false;
                self.video_result = None;
            }
        }
    }

    pub fn enqueue_selected(&mut self) {
        let Some(url) = self.video_url.clone() else {
            self.status = Some("Load a video first!".to_string());
            return;
        };
    
        // Enter em Formats sempre significa "usar a linha escolhida", então vira Custom
        let preset = if self.focus == Focus::Formats { Preset::Custom } else { self.preset };
    
        let custom_format = self.formats_state.selected()
            .and_then(|i| self.formats.get(i))
            .map(|row| if row.kind == "video" { format!("{}+bestaudio", row.id) } else { row.id.clone() });
    
        if preset == Preset::Custom && custom_format.is_none() {
            return;
        }
        if preset == Preset::AudioMp3 && !self.has_ffmpeg {
            self.status = Some("ffmpeg is required for MP3!".to_string());
            return;
        }
    
        let args = ytdlp::preset_args(preset, custom_format.as_deref());
        let title = self.video.as_ref().map(|v| v.title.clone()).unwrap_or_default();
    
        self.queue.push(DownloadItem {
            title, url, args,
            status: DownloadStatus::Waiting,
            progress: 0.0,
            speed: String::new(),
            eta: String::new(),
        });
        // ...select e status iguais ao que você já tem
    }

    pub fn poll_downloads(&mut self) {
        use std::sync::mpsc::{channel, TryRecvError};
        use std::thread;
    
        if let (Some(rx), Some(idx)) = (&self.download_rx, self.current_download) {
            let result = match rx.try_recv() {
                Ok(r) => r,
                Err(TryRecvError::Empty) => return,
                Err(TryRecvError::Disconnected) => Err("Download thread died".to_string()),
            };
    
            self.queue[idx].status = match result {
                Ok(()) => DownloadStatus::Done,
                Err(e) => DownloadStatus::Failed(e),
            };
            self.queue[idx].progress = if matches!(self.queue[idx].status, DownloadStatus::Done) { 100.0 } else { 0.0 };
            self.download_rx = None;
            self.current_download = None;
        }
        
        let Some(idx) = self.queue.iter().position(|i| matches!(i.status, DownloadStatus::Waiting)) else {
            return;
        };
    
        self.queue[idx].status = DownloadStatus::Downloading;
    
        let url = self.queue[idx].url.clone();
        let args = self.queue[idx].args.clone();
        let dir = self.download_dir.clone();
    
        let (tx, rx) = channel();
        thread::spawn(move || {
            let _ = tx.send(ytdlp::download_video(&url, &args, &dir));
        });
    
        self.download_rx = Some(rx);
        self.current_download = Some(idx);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}