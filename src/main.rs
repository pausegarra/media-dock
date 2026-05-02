#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    pullyt::modules::downloader::presentation::tauri_app::run()
}
