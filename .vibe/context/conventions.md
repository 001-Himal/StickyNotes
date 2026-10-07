# conventions.md

## Naming
- Files/folders: `snake_case` for Rust files and modules (`note_manager.rs`, `platform/windows.rs`), `kebab-case` for crates and docs.
- Components: `PascalCase` for Slint UI elements (`NoteWindow`, `HeaderBar`, `SettingsPanel`).
- Functions: `snake_case` for Rust functions and methods (`create_note`, `save_notes_atomic`).
- Constants: `SCREAMING_SNAKE_CASE` (`DEFAULT_NOTE_WIDTH`, `AUTOSAVE_DEBOUNCE_MS`).

## Formatting
- Rust formatter: `cargo fmt` (standard 4-space indentation, 100-character line width).
- Linter: `cargo clippy -- -D warnings` (strict zero warnings).
- Slint UI: Standard indentation and modular `.slint` component files.

## Patterns to follow
- **Atomic File Writing**: Always write to a `.tmp` file in the same directory before performing an atomic rename, preventing file corruption.
- **Debounced Persistence**: Batch and debounce text input changes by 300ms before persisting to disk.
- **Zero-Polling Idle**: Rely on OS window event callbacks and message loops; do not run busy wait loops or unnecessary timers.
- **Graceful Fallbacks**: If `config.json` or `Notes.json` is missing, initialize sane defaults without crashing.

## Patterns to avoid
- No heavy async runtimes (avoid heavy Tokio overhead; rely on Slint's native event loop or minimal std threads).
- No WebViews, DOMs, or browser runtimes.
- No blocking I/O on the main GUI thread.
- No external network requests, auto-updaters, or telemetry calls.
