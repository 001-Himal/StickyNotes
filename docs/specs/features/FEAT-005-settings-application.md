# FEAT-005 — Settings Application

## Goal
Provide a minimal, on-demand configuration executable (`Sticky Note Settings`) that edits `config.json` without cluttering the main note interface, styled in the exact same warm paper pastel theme as the sticky notes.

## Schema: `config.json`
```json
{
  "version": 1,
  "general": {
    "start_with_os": false
  },
  "note_appearance": {
    "last_used_width": 300,
    "last_used_height": 200,
    "corner_style": "curled",
    "font_family": "Segoe UI",
    "font_size": 14,
    "default_color": "yellow"
  },
  "behavior": {
    "close_action": "ask",
    "autosave_debounce_ms": 300
  },
  "shortcuts": {
    "new_note": "Ctrl+Alt+N",
    "open_settings": "Ctrl+Alt+S"
  }
}
```

## Requirements
1. **Independent Binary & Sticky Note Aesthetic**:
   - Compiled as `sticky-note-settings` (`Sticky Note Settings.exe`).
   - Renders as a compact (380×320px) pastel card matching the warm paper sticky note theme, not an OS control panel.
2. **On-Demand Lifecycle**:
   - Launched via `Ctrl+Alt+S` or Start Menu.
   - Exits immediately when closed, freeing all resources.
3. **Preferences Managed**:
   - **Close Button Action**: Radio options (`delete` permanently, `close` and keep saved, `ask` each time).
   - **General**: Launch on OS startup toggle.
   - **Appearance**:
     - **Corner Style:** Curled / Bended corner (Classic) vs. Flat corner (Modern).
     - Font family, font size, default color.
   - **Shortcuts**: Key combinations for New Note and Open Settings.
4. **Dynamic Size Inheritance**:
   - Whenever any note is resized by the user, `last_used_width` and `last_used_height` in `config.json` update automatically. New notes instantiate using these dimensions. No static presets dropdown.
5. **Real-Time Config Synchronization via IPC**:
   - On saving changes to `config.json`, `Sticky Note Settings` connects to the local IPC named pipe (`\\.\pipe\StickyNote_IPC`) and transmits `RELOAD_CONFIG`.
   - Running `Sticky Note.exe` updates active notes and settings in-memory immediately without busy polling or requiring restart.
6. **No Trash Recovery**:
   - Strictly minimalist; no trash or recovery database.

## Acceptance Criteria
- [ ] Settings window displays with the warm pastel paper sticky note theme.
- [ ] Close action options include: Delete, Close, and Ask prompt.
- [ ] Saving settings notifies running `Sticky Note` process via IPC to reload configuration.
- [ ] Resizing any note dynamically updates default dimensions for future notes.
- [ ] Settings process exits cleanly with zero background residue.
