#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> iced::Result {
    pullryn::modules::downloader::presentation::app::run()
}
