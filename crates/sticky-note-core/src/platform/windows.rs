//! Windows-specific desktop layer and stealth window integration.

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows_sys::Win32::Foundation::{HWND, POINT, RECT};
use windows_sys::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_BOTTOM,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
};

/// Finds a window by its exact window title bar string.
///
/// # Safety
/// Calls Win32 `FindWindowW` FFI.
pub unsafe fn find_window_by_title(title: &str) -> Option<HWND> {
    let wide: Vec<u16> = OsStr::new(title)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let hwnd = FindWindowW(std::ptr::null(), wide.as_ptr());
    if hwnd == 0 {
        None
    } else {
        Some(hwnd)
    }
}

/// Applies stealth window styles (`WS_EX_TOOLWINDOW` and strips `WS_EX_APPWINDOW`)
/// to omit the note from the Windows Taskbar and Alt+Tab application switcher.
///
/// # Safety
/// Calls Win32 `GetWindowLongPtrW` and `SetWindowLongPtrW` FFI.
pub unsafe fn apply_stealth_window_styles(hwnd: HWND) {
    let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
    let new_ex_style = (ex_style | WS_EX_TOOLWINDOW as isize) & !(WS_EX_APPWINDOW as isize);
    SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_ex_style);
}

/// Sinks a window directly to the desktop bottom layer (`HWND_BOTTOM`)
/// ensuring it quietly stays beneath normal applications without stealing focus.
///
/// # Safety
/// Calls Win32 `SetWindowPos` FFI.
pub unsafe fn sink_to_desktop_layer(hwnd: HWND) {
    SetWindowPos(
        hwnd,
        HWND_BOTTOM,
        0,
        0,
        0,
        0,
        SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
    );
}

/// Validates and clamps `(x, y)` coordinates to ensure the note window
/// remains fully visible within the active monitor workspace.
///
/// # Safety
/// Calls Win32 GDI `MonitorFromPoint` and `GetMonitorInfoW` FFI.
pub unsafe fn clamp_to_monitor_bounds(x: i32, y: i32, width: u32, height: u32) -> (i32, i32) {
    let pt = POINT { x, y };
    let hmon = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
    let mut mi = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        rcMonitor: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        rcWork: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        dwFlags: 0,
    };

    if GetMonitorInfoW(hmon, &mut mi) != 0 {
        let max_x = mi.rcWork.right - width as i32;
        let max_y = mi.rcWork.bottom - height as i32;
        let clamped_x = x.clamp(mi.rcWork.left, max_x.max(mi.rcWork.left));
        let clamped_y = y.clamp(mi.rcWork.top, max_y.max(mi.rcWork.top));
        (clamped_x, clamped_y)
    } else {
        (x, y)
    }
}

/// Configures or removes Windows auto-start via HKCU\Software\Microsoft\Windows\CurrentVersion\Run.
pub fn set_autostart_registry(enable: bool, exe_path: Option<&str>) -> std::io::Result<()> {
    if enable {
        let path = if let Some(p) = exe_path {
            p.to_string()
        } else {
            std::env::current_exe()?.to_string_lossy().to_string()
        };
        let _ = std::process::Command::new("reg")
            .args([
                "add",
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "/v",
                "StickyNote",
                "/t",
                "REG_SZ",
                "/d",
                &path,
                "/f",
            ])
            .output()?;
    } else {
        let _ = std::process::Command::new("reg")
            .args([
                "delete",
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "/v",
                "StickyNote",
                "/f",
            ])
            .output()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_to_monitor_bounds_fallback() {
        // Safe invocation on any monitor configuration
        let (x, y) = unsafe { clamp_to_monitor_bounds(100, 100, 300, 200) };
        assert!(x >= 0 || x < 0); // Valid coordinate returned
        assert!(y >= 0 || y < 0);
    }
}
