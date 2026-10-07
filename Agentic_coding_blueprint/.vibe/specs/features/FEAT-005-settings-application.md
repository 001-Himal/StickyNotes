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
    "font_family": "Segoe UI",
    "font_size": 14,
    "default_color": "#FFF8D6"
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
   - **Appearance**: Font family, font size, default color.
   - **Shortcuts**: Key combinations for New Note and Open Settings.
4. **Dynamic Size Inheritance**:
   - Whenever any note is resized by the user, `last_used_width` and `last_used_height` in `config.json` update automatically. New notes instantiate using these dimensions. No static presets dropdown.
5. **No Trash Recovery**:
   - Strictly minimalist; no trash or recovery database.

## Acceptance Criteria
- [ ] Settings window displays with the warm pastel paper sticky note theme.
- [ ] Close action options include: Delete, Close, and Ask prompt.
- [ ] Resizing any note dynamically updates default dimensions for future notes.
- [ ] Settings process exits cleanly with zero background residue.
