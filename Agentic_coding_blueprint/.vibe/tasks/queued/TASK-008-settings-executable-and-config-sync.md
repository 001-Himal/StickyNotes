# TASK-008: Settings Executable and Sticky Note Themed Popup

- **Status:** Queued
- **Priority:** Medium
- **Owner:** Himal
- **Target:** Milestone 3

## Description
1. Create `crates/sticky-note-settings/` application.
2. Build `ui/settings_window.slint` as a compact (380×320px) popup matching the warm pastel paper sticky note theme.
3. Configure settings options:
   - Close button action: Delete / Close / Ask each time.
   - Start with Windows toggle.
   - Font family and size selector.
   - Default color choice.
   - Shortcut rebinders (`Ctrl+Alt+N`, `Ctrl+Alt+S`).
4. Exclude static size presets (new notes dynamically inherit last resized dimensions).
5. Read `config.json` on launch, write atomic update on user change.
6. Terminate immediately upon close.

## Acceptance Criteria
- [ ] `Sticky Note Settings.exe` opens as a minimal, pastel sticky-note-styled popup.
- [ ] Allows toggling Close action between Delete, Close, and Ask prompt.
- [ ] Closes cleanly with zero background residue.
