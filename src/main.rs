use slint::ToSharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app_window = AppWindow::new()?;
    app_window.invoke_set_font_size(14.0);
    app_window.invoke_set_font_family("comic sans".to_shared_string());
    app_window.run()
}
