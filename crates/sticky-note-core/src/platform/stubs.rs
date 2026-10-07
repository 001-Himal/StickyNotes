//! Cross-platform stubs for non-Windows systems (macOS, Linux).

/// Sinks window to desktop layer (stub).
pub fn sink_to_desktop_layer<T>(_handle: T) {}

/// Applies stealth window styles (stub).
pub fn apply_stealth_window_styles<T>(_handle: T) {}

/// Clamps coordinates to monitor bounds (stub).
pub fn clamp_to_monitor_bounds(x: i32, y: i32, _width: u32, _height: u32) -> (i32, i32) {
    (x, y)
}

/// Sets autostart registry entry (stub).
pub fn set_autostart_registry(_enable: bool, _exe_path: Option<&str>) -> std::io::Result<()> {
    Ok(())
}

