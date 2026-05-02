use pullyt::modules::downloader::infrastructure::yt_dlp::parse_progress;

#[test]
fn parses_percentage_line() {
    let p = parse_progress("[download]  42.4% of 12.01MiB at 2.11MiB/s ETA 00:04").unwrap();
    assert!(p > 0.42 && p < 0.43);
}
