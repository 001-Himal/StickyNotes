# TASK-004: Windows Desktop Layer and Stealth Window Integration

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 1

## Description
In `crates/sticky-note-core/src/platform/windows.rs`:
1. Use `windows-sys` FFI to obtain HWND from Slint window.
2. Apply `WS_EX_TOOLWINDOW` and strip `WS_EX_APPWINDOW` to remove window from Taskbar and Alt+Tab switcher.
3. Position window at `HWND_BOTTOM` (`SetWindowPos(hwnd, HWND_BOTTOM, ...)`).
4. Implement Z-order focus lifecycle:
   - When clicked/focused, allow note to accept keyboard input.
   - On deactivation/blur (`WM_KILLFOCUS` / `WM_ACTIVATE`), sink window immediately back to `HWND_BOTTOM` (`SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE`).
5. Hook `WM_DISPLAYCHANGE` to trigger display bounds re-validation across multi-monitor setups.
6. Verify that opening Chrome or File Explorer covers the note window.

## Acceptance Criteria
- [ ] Note is visible on the Windows desktop.
- [ ] Note is completely invisible in the Taskbar and Alt+Tab switcher.
- [ ] Clicking allows typing; switching to another app sinks note quietly behind foreground windows.
- [ ] Changing display resolution does not push notes off-screen.
