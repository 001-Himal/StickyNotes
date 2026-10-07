slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _window = SettingsWindow::new()?;
    Ok(())
}
