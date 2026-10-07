# roadmap.md

## Now (Milestone 1: Core Note Engine & Native Windowing)
- [x] Rust Workspace setup (`sticky-note-core`, `sticky-note`, `sticky-note-settings`).
- [x] Slint UI setup for `NoteWindow` (two-tone header/body, text editor, hover-only `+` and `X`, curled/flat corner, 6-color palette).
- [x] Windows OS native integration: `HWND_BOTTOM` desktop widget layering, stealth `WS_EX_TOOLWINDOW` (no taskbar, no Alt+Tab), multi-monitor clamping.
- [x] Atomic local JSON storage: per-note files in `Sticky Note Notes/` (`Note 1.json`, etc.) + `Notes.json` index with self-healing reconciliation.
- [x] Window dragging, single-direction bottom-right resizing (min 180×120), and dynamic size inheritance (TASK-005).
- [x] Debounced auto-save on typing and geometry change (TASK-005).

## Next (Milestone 2: Interactions, Hotkeys, File Launching & Settings)
- [x] Title numbering logic (`Note 1`, `Note 2`, `Note 3`, ...) with fallback placeholder.
- [x] Double-click header inline rename with placeholder handling.
- [x] **Direct File Access:** Double-clicking individual note files in `Sticky Note Notes/` opens/unhides that note in `Sticky Note` (single-instance IPC).
- [x] Global shortcuts integration via `global-hotkey` (`Ctrl+Alt+N`, `Ctrl+Alt+S`).
- [x] `Sticky Note Settings` executable UI as a minimal matching pastel card.
- [x] Close button configurable behavior (Delete / Close / Ask prompt).

## Later (Milestone 3: Windows Packaging & Release)
- [x] Windows CI workflow (`.github/workflows/ci.yml`).
- [x] Windows release binary packaging (`Sticky Note.exe`, `Sticky Note Settings.exe`).
- [ ] System tray icon for discreet background management.

## Blocked / Deferred (Future Milestone)
- macOS native window integration (`kCGDesktopWindowLevel`, accessory mode) [TASK-009].
- Linux X11/Wayland desktop widget layer integration [TASK-009].
- Multi-OS cross-platform CI matrix [TASK-010].

