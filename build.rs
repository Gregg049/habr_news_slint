fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
    if cfg!(target_os = "windows") {
        winres::WindowsResource::new()
            .set_icon("ui/icons/ico_habr.ico")
            .compile()
            .unwrap();
    }
}
