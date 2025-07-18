use slint::ToSharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app_window = AppWindow::new()?;

    let ui_settings = UISettings {
        fontsize: 14.0,
        fontfamily: "FreeSans".to_shared_string(),
    };

    app_window.invoke_apply_ui_settings(ui_settings);

    app_window.run()
}
