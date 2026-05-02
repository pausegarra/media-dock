fn main() {
    tauri_build::build();

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icons/windows/app.ico");
        res.compile().expect("failed to embed Windows resource");
    }
}
