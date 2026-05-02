use pullryn::modules::downloader::domain::errors::DownloaderError;
use pullryn::modules::downloader::domain::value_objects::YoutubeUrl;

#[test]
fn accepts_youtube_watch_url() {
    let parsed = YoutubeUrl::parse("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    assert!(parsed.is_ok());
}

#[test]
fn accepts_youtu_be_url() {
    let parsed = YoutubeUrl::parse("https://youtu.be/dQw4w9WgXcQ");
    assert!(parsed.is_ok());
}

#[test]
fn rejects_non_youtube_url() {
    let parsed = YoutubeUrl::parse("https://example.org/video");
    assert!(matches!(parsed, Err(DownloaderError::InvalidUrl)));
}
