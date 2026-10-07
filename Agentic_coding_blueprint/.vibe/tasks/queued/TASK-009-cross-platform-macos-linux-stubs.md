# TASK-009: Cross-Platform macOS and Linux Windowing Abstractions

- **Status:** Queued
- **Priority:** Low
- **Owner:** Himal
- **Target:** Milestone 4

## Description
1. Create `crates/sticky-note-core/src/platform/macos.rs` using Cocoa / Objective-C runtime for desktop level and dock suppression.
2. Create `crates/sticky-note-core/src/platform/linux.rs` using X11 / Wayland window type hints (`_NET_WM_WINDOW_TYPE_DESKTOP`).
3. Ensure conditional compilation (`#[cfg(target_os = ...)]`) compiles smoothly on all platforms.

## Acceptance Criteria
- [ ] Code compiles without warnings on Windows, macOS, and Linux.
- [ ] Desktop layering and taskbar/dock hiding works on each platform.
