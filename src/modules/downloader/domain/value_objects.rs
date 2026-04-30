use super::errors::DownloaderError;

#[derive(Debug, Clone)]
pub struct YoutubeUrl(String);

impl YoutubeUrl {
    pub fn parse(raw: &str) -> Result<Self, DownloaderError> {
        let normalized = raw.trim();
        let is_valid = normalized.contains("youtube.com/watch") || normalized.contains("youtu.be/");
        if is_valid {
            Ok(Self(normalized.to_string()))
        } else {
            Err(DownloaderError::InvalidUrl)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
