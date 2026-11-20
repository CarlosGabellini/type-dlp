use std::process::Command;

use serde::Deserialize;

use crate::my_tui::app::FormatRow;

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
            if !picked_audio {
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