# FEAT-004 — Global Shortcuts

## Goal
Allow users to create new sticky notes or open settings instantly from anywhere in the OS without switching to the desktop first.

## Requirements
1. **Shortcut Registrations**:
   - `Ctrl + Alt + N` (macOS: `Cmd + Alt + N`): Spawns a new sticky note centered or cascaded on the active screen.
   - `Ctrl + Alt + S` (macOS: `Cmd + Alt + S`): Launches or brings forward `Sticky Note Settings`.
2. **Implementation**:
   - Utilize the `global-hotkey` crate on a lightweight dedicated thread or within the event loop.
   - If hotkeys conflict with another app, log a warning gracefully without failing startup.
3. **Configurability**:
   - Key combinations stored in and read from `config.json`.

## Acceptance Criteria
- [ ] Pressing `Ctrl+Alt+N` while inside Chrome or VS Code immediately creates a new note on the desktop.
- [ ] Pressing `Ctrl+Alt+S` opens the Settings dialog window.
- [ ] Background CPU usage while waiting for hotkeys is 0.0%.
