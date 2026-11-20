use std::{io::{BufRead, BufReader}, path::Path, process::{Command, Stdio}, sync::mpsc::Sender};

use serde::Deserialize;

use crate::my_tui::app::{DownloadMsg, FormatRow, Preset};

#[derive(Deserialize)]
pub struct YtDlpOutput {
    pub title: String,
    pub uploader: Option<String>,
    pub duration: Option<f64>,
    pub formats: Vec<FormatJson>,
}

#[derive(Deserialize)]
pub struct FormatJson {
    pub format_id: String,
    pub resolution: Option<String>,
    pub fps: Option<f64>,
    pub vcodec: Option<String>,
    pub acodec: Option<String>,
    pub filesize: Option<u64>,
    pub filesize_approx: Option<u64>,
    pub format_note: Option<String>,
}

//Run the command ty-dlp -J and returns some information;
pub fn fetch_video_info(url: &str) -> Result<YtDlpOutput, String> {
    let output = Command::new("yt-dlp")
        .arg("-J")
        .arg(url)
        .output()
        .map_err(|e| format!("Fail to execute, error: {e}"))?;

    if !output.status.success() {
        return Err("yt-dlp return an error. verify the url".to_string());
    }

    serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Fail to review answer of yt-dlp: {e}"))
}

//Filters the popular formats from the list of formats.
pub fn filter_popular(formats: Vec<FormatJson>) -> Vec<FormatJson> {
    use std::collections::HashSet;
    let mut seen_resolutions = HashSet::new();
    let mut result = Vec::new();
    let mut picked_audio = false;

    for f in formats {
        let is_storyboard = f.vcodec.as_deref() == Some("none") && f.acodec.as_deref() == Some("none");

        if is_storyboard {
            continue;
        }

        let is_audio_only = f.vcodec.as_deref() == Some("none");

        if is_audio_only {
            if picked_audio {
                continue;
            }

            picked_audio = true;
            result.push(f);
            continue;
        }

        if let Some(res) = &f.resolution {
            if seen_resolutions.contains(res) {
                continue;
            }

            seen_resolutions.insert(res.clone());
        }

        result.push(f);
    }

    result
}

pub fn format_size(bytes: Option<u64>) -> String {
    match bytes {
        Some(b) => format!("~{:.0} Mib", b as f64 / 1024.0 / 1024.0),
        None => "-".to_string(),
    }
}

pub fn to_format_row(f: &FormatJson) -> FormatRow {
    let kind = match (f.vcodec.as_deref(), f.acodec.as_deref()) {
        (Some("none"), _) => "audio",
        (_, Some("none")) => "video",
        _ => "v + a",
    };

    FormatRow {
        id: f.format_id.clone(),
        kind: kind.to_string(),
        res: f.resolution.clone().unwrap_or_else(|| "-".to_string()),
        fps: f.fps.map(|v| format!("{v:.0}")).unwrap_or_else(|| "-".to_string()),
        codec: format!("{}/{}", f.vcodec.as_deref().unwrap_or("—"), f.acodec.as_deref().unwrap_or("—")),
        size: format_size(f.filesize.or(f.filesize_approx)),
    }
}

pub fn format_duration(secods: f64) -> String {
    let total = secods.round() as u64;
    format!("{:02}:{:02}", total / 60, total % 60)
}

pub fn preset_args(preset: Preset, custom_format: Option<&str>) -> Vec<String> {
    let v = |s: &[&str]| s.iter().map(|x| x.to_string()).collect();

    //Adjust presets to convert ALL FOR MP4;

    match preset {
        Preset::Best => v(&["-f", "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best"]),
        Preset::P1080 => v(&["-f", "bestvideo[height<=1080][ext=mp4]+bestaudio[ext=m4a]/best[height<=1080][ext=mp4]/best[height<=1080]"]),
        Preset::P720 => v(&["-f", "bestvideo[height<=720][ext=mp4]+bestaudio[ext=m4a]/best[height<=720][ext=mp4]/best[height<=720]"]),
        Preset::AudioMp3 => v(&["-x", "--audio-format", "mp3"]),
        Preset::Custom => v(&["-f", custom_format.unwrap_or("best")]),
    }
}

pub fn download_video(url: &str, args: &[String],dir: &Path,tx: &Sender<DownloadMsg>) -> Result<(), String> {
    let template = dir.join("%(title)s.%(ext)s");

    let mut child = Command::new("yt-dlp")
        .args(args)
        .arg("--newline")
        .arg("--progress-template")
        .arg("download:%(progress._percent_str)s|%(progress._speed_str)s|%(progress._eta_str)s")
        .arg("-o").arg(&template)
        .arg(url)
        .arg("--merge-output-format").arg("mp4")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Fail to execute, error: {e}"))?;

    
    let stderr = child.stderr.take().ok_or("Could not read stderr")?;
    
    let err_thread = std::thread::spawn(move || {
        BufReader::new(stderr)
            .lines()
            .map_while(Result::ok)
            .last()
            .unwrap_or_default()
    });

    let stdout = child.stdout.take().ok_or("Could not read stdout")?;

    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() != 3 {
            continue; // linhas que não são do template (ex.: [Merger])

        }

        let Ok(pct) = parts[0].trim().trim_end_matches('%').parse::<f32>() else {
            continue;
        };

        let _ = tx.send(DownloadMsg::Progress {
            pct,
            speed: parts[1].trim().to_string(),
            eta: parts[2].trim().to_string(),
        });
    }

    let status = child.wait().map_err(|e| format!("Wait failed: {e}"))?;
    let last_err = err_thread.join().unwrap_or_default();

    if status.success() {
        Ok(())

    } else if last_err.is_empty() {
        Err("yt-dlp failed".to_string())

    } else {
        Err(format!("yt-dlp failed: {last_err}"))
    }
}