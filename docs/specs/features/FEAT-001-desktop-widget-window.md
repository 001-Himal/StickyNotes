# FEAT-001 — Desktop Widget Windowing

## Goal
Render borderless, lightweight note windows directly on the OS desktop layer, ensuring they stay behind active user applications (Chrome, VS Code, games) and never appear in the taskbar or Alt+Tab switcher.

## Requirements
1. **Window Layering**: Window must sit directly above desktop wallpaper / icons, beneath all normal applications.
   - Windows: Call Win32 `SetWindowLongPtr` with `WS_EX_TOOLWINDOW` (removes from taskbar and Alt+Tab) and position with `SetWindowPos(hwnd, HWND_BOTTOM, ...)`. Hook desktop worker window if needed.
   - macOS: Configure `NSWindowCollectionBehaviorCanJoinAllSpaces` and window level `kCGDesktopWindowLevel`.
   - Linux: Set `_NET_WM_WINDOW_TYPE_DESKTOP` or utility window hints.
2. **Frameless Shell**: Borderless window rendered via Slint without default OS title bar.
3. **Smooth Dragging**: Dragging the top header area moves the window position smoothly.
4. **Smooth Resizing**: Dragging the bottom-right grip resizes the window with minimum clamp at 180×120.

## Acceptance Criteria
- [ ] Note appears on the desktop.
- [ ] When Chrome/Notepad is clicked, the note disappears behind it (does not stay on top).
- [ ] No icon in the Windows Taskbar, macOS Dock, or Alt+Tab switcher.
- [ ] Header dragging moves the window accurately across single and multi-monitor setups.
- [ ] Memory footprint remains <30 MB.
