use rfd::FileDialog;

use crate::modules::downloader::domain::entities::{DownloadMode, DownloadPreset};
use crate::modules::downloader::domain::ports::SaveDialogPort;

pub struct NativeSaveDialog;

impl SaveDialogPort for NativeSaveDialog {
    fn choose_output_file(
        &self,
        mode: DownloadMode,
        preset: DownloadPreset,
        title: &str,
    ) -> Option<String> {
        let mut dialog = FileDialog::new();

        let safe_title = sanitize_filename(title);
        let default_name = if safe_title.is_empty() {
            "media-dock".to_string()
        } else {
            safe_title
        };

        dialog = match mode {
            DownloadMode::AudioOnlyMp3 => {
                dialog
                    .add_filter("MP3 audio", &["mp3"])
                    .set_file_name(&format!("{}.mp3", default_name))
            }
            DownloadMode::VideoWithAudio => match preset {
                DownloadPreset::Compatibility => dialog
                    .add_filter("Video", &["mp4"])
                    .set_file_name(&format!("{}.mp4", default_name)),
                DownloadPreset::MaxQuality => dialog
                    .add_filter("Video", &["mkv", "webm", "mp4"])
                    .set_file_name(&format!("{}.mkv", default_name)),
            },
        };

        dialog.save_file().map(|p| p.display().to_string())
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}
