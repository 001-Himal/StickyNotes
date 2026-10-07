# TASK-004: Windows Desktop Layer and Stealth Window Integration

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 1

## Description
In `crates/sticky-note-core/src/platform/windows.rs`:
1. Use `windows-sys` FFI to obtain HWND from Slint window.
2. Apply `WS_EX_TOOLWINDOW` and strip `WS_EX_APPWINDOW` to remove window from Taskbar and Alt+Tab switcher.
3. Position window at `HWND_BOTTOM` (or attach behind top-level applications, above Progman / WorkerW).
4. Verify that opening Chrome or File Explorer covers the note window.

## Acceptance Criteria
- [ ] Note is visible on the Windows desktop.
- [ ] Note is completely invisible in the Taskbar and Alt+Tab switcher.
- [ ] Note stays behind all foreground applications.
