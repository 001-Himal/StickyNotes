# TASK-009: Cross-Platform macOS and Linux Windowing Abstractions

- **Status:** Blocked
- **Priority:** Low
- **Owner:** Himal
- **Target:** Milestone 3
- **Block Reason:** Deferred per user directive to focus exclusively on Windows native experience first. Cross-platform implementation will be revisited after Windows feature-complete release.

## Description
1. Create `crates/sticky-note-core/src/platform/macos.rs` using Cocoa runtime:
   - Set `NSWindow.level` above desktop icons (`CGWindowLevelForKey(.desktopWindow) + 1`).
   - Set `NSApplicationActivationPolicyAccessory` (suppress Dock and Cmd+Tab switcher).
   - Set `NSWindowCollectionBehaviorCanJoinAllSpaces | NSWindowCollectionBehaviorStationary`.
2. Create `crates/sticky-note-core/src/platform/linux.rs` using X11 / Wayland window type hints:
   - X11: Set `_NET_WM_WINDOW_TYPE_UTILITY`, `_NET_WM_STATE_BELOW`, `_NET_WM_STATE_STICKY`, `_NET_WM_STATE_SKIP_TASKBAR`, `_NET_WM_STATE_SKIP_PAGER`.
   - Wayland: Apply frameless utility window hints (and `wlr-layer-shell` layer bottom where supported).
3. Ensure conditional compilation (`#[cfg(target_os = ...)]`) compiles smoothly on all platforms.

## Acceptance Criteria
- [ ] Code compiles without warnings on Windows, macOS, and Linux.
- [ ] macOS window stays above desktop icons, below regular applications, and hidden from Dock.
- [ ] Linux X11 window stays on desktop layer without capturing root wallpaper events.
