# FEAT-005 — Settings Application

## Goal
Provide a decoupled, on-demand configuration executable (`Sticky Note Settings`) that edits `config.json` without cluttering the main note interface.

## Schema: `config.json`
```json
{
  "version": 1,
  "general": {
    "start_with_os": false,
    "confirm_before_delete": true
  },
  "note_appearance": {
    "default_width": 300,
    "default_height": 200,
    "font_family": "Segoe UI",
    "font_size": 14,
    "default_color": "#FFF8D6"
  },
  "behavior": {
    "close_action": "delete",
    "autosave_debounce_ms": 300
  },
  "shortcuts": {
    "new_note": "Ctrl+Alt+N",
    "open_settings": "Ctrl+Alt+S"
  }
}
```

## Requirements
1. **Independent Binary**: Compiled as `sticky-note-settings` (`Sticky Note Settings.exe`).
2. **On-Demand Lifecycle**: Launched only when the user requests it. Closes immediately when dismissed, releasing all resources.
3. **Sections**:
   - General (Startup, confirmation prompt).
   - Note Appearance (Default size, font, color palette).
   - Behavior (Close button action: `delete`, `close`, `hide`).
   - Shortcuts (Rebinding keys).
4. **Synchronization**: On save, write atomically to `config.json`. The running `sticky-note` process watches or detects configuration changes and reloads dynamically.

## Acceptance Criteria
- [ ] Changing Close action to "Delete" causes the note to be deleted upon clicking 'X'.
- [ ] Changing Close action to "Close/Hide" keeps the note saved in `Notes.json` marked closed.
- [ ] Settings executable terminates cleanly on exit with 0 lingering processes.
