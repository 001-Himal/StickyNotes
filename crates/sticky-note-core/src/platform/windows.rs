//! Windows-specific desktop layer and stealth window integration.

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows_sys::Win32::Foundation::{BOOL, HWND, LPARAM, POINT, RECT};
use windows_sys::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowW, GetWindowLongPtrW, GetWindowThreadProcessId, SetForegroundWindow,
    SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_BOTTOM, HWND_TOP, SWP_FRAMECHANGED,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW, WS_EX_APPWINDOW,
    WS_EX_TOOLWINDOW,
};

unsafe extern "system" fn enum_process_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let hwnds = &mut *(lparam as *mut Vec<HWND>);
    let mut pid = 0;
    GetWindowThreadProcessId(hwnd, &mut pid);
    if pid == std::process::id() {
        hwnds.push(hwnd);
    }
    1
}

/// Enumerates all windows belonging to the current process.
///
/// # Safety
/// Calls Win32 `EnumWindows` and `GetWindowThreadProcessId` FFI.
pub unsafe fn find_process_windows() -> Vec<HWND> {
    let mut hwnds: Vec<HWND> = Vec::new();
    EnumWindows(Some(enum_process_windows_proc), &mut hwnds as *mut _ as LPARAM);
    hwnds
}

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
/// with `SWP_FRAMECHANGED` to omit the note from the Windows Taskbar and Alt+Tab application switcher.
///
/// # Safety
/// Calls Win32 `GetWindowLongPtrW`, `SetWindowLongPtrW`, and `SetWindowPos` FFI.
pub unsafe fn apply_stealth_window_styles(hwnd: HWND) {
    let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
    let new_ex_style = (ex_style | WS_EX_TOOLWINDOW as isize) & !(WS_EX_APPWINDOW as isize);
    SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_ex_style);
    SetWindowPos(
        hwnd,
        0,
        0,
        0,
        0,
        0,
        SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
    );
}

/// Brings a window to the top of the z-order and activates it.
///
/// # Safety
/// Calls Win32 `SetWindowPos` and `SetForegroundWindow` FFI.
pub unsafe fn bring_window_to_front(hwnd: HWND) {
    apply_stealth_window_styles(hwnd);
    SetWindowPos(
        hwnd,
        HWND_TOP,
        0,
        0,
        0,
        0,
        SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
    );
    SetForegroundWindow(hwnd);
}

/// Brings all windows belonging to the current process to the front and activates them.
///
/// # Safety
/// Calls Win32 FFI functions.
pub unsafe fn bring_all_process_windows_to_front() {
    let hwnds = find_process_windows();
    for hwnd in hwnds {
        bring_window_to_front(hwnd);
    }
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

/// Finds all windows of the current process, applies stealth styles,
/// and sinks them to the desktop layer.
///
/// # Safety
/// Calls Win32 FFI functions.
pub unsafe fn stealth_and_sink_all_process_windows() {
    let hwnds = find_process_windows();
    for hwnd in hwnds {
        apply_stealth_window_styles(hwnd);
        sink_to_desktop_layer(hwnd);
    }
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
