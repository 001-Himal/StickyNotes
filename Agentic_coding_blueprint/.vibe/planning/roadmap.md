# roadmap.md

## Now (Milestone 1: Core Note Engine & Native Windowing)
- Rust Workspace setup (`sticky-note-core`, `sticky-note`, `sticky-note-settings`).
- Slint UI setup for `NoteWindow` (header, text editor, hover-only `+` and `X`, hover-only resize grip).
- Single-direction bottom-right resize handling (bottom and right expansion, min-clamp 180×120).
- Windows OS native integration: `HWND_BOTTOM` / desktop widget layering, frameless window, omit from taskbar and Alt+Tab.
- Atomic local JSON storage: per-note files in `Sticky Note Notes/` (`Note 1.json`, etc.) + `Notes.json` index.
- Debounced auto-save on typing.

## Next (Milestone 2: Interactions, Hotkeys, File Launching & Settings)
- Title numbering logic (`Note 1`, `Note 2`, `Note 3`, ...) with fallback placeholder.
- Double-click header inline rename with placeholder handling.
- **Direct File Access:** Double-clicking individual note files in `Sticky Note Notes/` opens/unhides that note in `Sticky Note` (single-instance IPC).
- Global shortcuts integration via `global-hotkey` (`Ctrl+Alt+N`, `Ctrl+Alt+S`).
- `Sticky Note Settings` executable UI as a minimal matching pastel card.
- Close button configurable behavior (Delete / Close / Ask prompt).

## Later (Milestone 3: Cross-Platform & Packaging)
- macOS native window integration (`kCGDesktopWindowLevel`, accessory mode).
- Linux X11/Wayland desktop widget layer integration.
- GitHub Actions CI matrix to compile native binaries for Windows (.exe), macOS, and Linux.
- System tray icon for discreet background management.
