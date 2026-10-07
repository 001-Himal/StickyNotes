# TASK-008: Settings Executable and Sticky Note Themed Popup

- **Status:** Completed
- **Priority:** Medium
- **Owner:** Himal
- **Target:** Milestone 2

## Description
1. Create `crates/sticky-note-settings/` application.
2. Build `ui/settings_window.slint` as a compact (400×490px) popup matching the warm pastel paper sticky note theme (`#FDF1B0` body, `#F6E077` header).
3. Configure settings options:
   - Close button action: Delete / Close / Ask each time.
   - Start with Windows toggle (Registry `Run` key on Windows).
   - Corner Style toggle: Curled / Bended dog-ear vs. Flat modern corner.
   - Font family and size selector.
   - Default color choice (6 pastel swatches).
   - Shortcut rebinders (`Ctrl+Alt+N`, `Ctrl+Alt+S`).
4. Read `config.json` on launch, write atomic update on user change.
5. Transmit `RELOAD_CONFIG` command via local IPC pipe (`\\.\pipe\StickyNote_IPC`) so running `Sticky Note.exe` reloads preferences in real time.
6. Terminate immediately upon close, releasing all memory.

## Acceptance Criteria
- [x] `Sticky Note Settings.exe` opens as a minimal, pastel sticky-note-styled popup.
- [x] Allows toggling Close action, Corner Style (Curled/Flat), and Startup.
- [x] Modifying settings notifies running `Sticky Note.exe` via IPC to reload configuration.
- [x] Closes cleanly with zero background residue.
