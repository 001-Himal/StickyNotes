# roadmap.md

## Now (Milestone 1: Core Note Engine & Native Windowing)
- Rust Workspace setup (`sticky-note-core`, `sticky-note`, `sticky-note-settings`).
- Slint UI setup for `NoteWindow` (header, text editor, hover close, hover resize).
- Windows OS native integration: `HWND_BOTTOM` / desktop widget layering, frameless window, omit from taskbar and Alt+Tab.
- Atomic local JSON storage in `Sticky Note Notes/Notes.json`.
- Debounced auto-save on typing.

## Next (Milestone 2: Interactions, Hotkeys & Settings)
- Title auto-increment logic ("Untitled Note 1", "Untitled Note 2", ...).
- Double-click header inline rename.
- Global shortcuts integration via `global-hotkey` (`Ctrl+Alt+N`, `Ctrl+Alt+S`).
- `Sticky Note Settings` executable UI and `config.json` schema.
- Cross button configurable behavior (Delete / Close / Hide).

## Later (Milestone 3: Cross-Platform & Packaging)
- macOS native window integration (`kCGDesktopWindowLevel`, accessory mode).
- Linux X11/Wayland desktop widget layer integration.
- GitHub Actions CI matrix to compile native binaries for Windows (.exe), macOS, and Linux.
- System tray icon for discreet background management.
