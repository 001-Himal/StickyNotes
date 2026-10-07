# FEAT-002 — Header Controls, Quick Add, Colors & Formatting

## Goal
Implement authentic physical sticky note micro-interactions: hover disclosure of quick-add (`+`) and close (`X`), double-click title renaming, right-click 6-color palette, and keyboard-only text formatting.

## Requirements
1. **Title Auto-Increment**:
   - New notes automatically receive the title `"Untitled Note 1"`.
   - If `"Untitled Note 1"` is already taken, increment sequentially (`"Untitled Note 2"`, etc.).
2. **Double-Click Rename**:
   - Double-clicking header text activates inline `TextInput`.
   - `Enter` commits title; `Escape` cancels; focus loss commits changes.
3. **Hover-Only Header Controls**:
   - Top-left `+` button: Revealed on header hover. Clicking immediately spawns and cascades a new note.
   - Top-right `X` button: Revealed on header hover.
4. **Configurable Close ('X') Behavior**:
   - Reads `close_action` from `config.json`:
     - `"delete"`: Permanently removes note from `Notes.json`.
     - `"close"`: Closes window while preserving note safely in `Notes.json`.
     - `"ask"`: Shows small themed prompt asking user: *"Delete note or just close?"*
5. **Right-Click 6-Color Palette**:
   - Right-clicking note canvas displays 6 classic pastel swatches (Yellow, Green, Blue, Purple, Pink, White).
   - Selected color updates the note theme immediately and persists in `Notes.json`.
6. **Keyboard-Only Formatting**:
   - Standard shortcuts without toolbars: `Ctrl+B` (Bold), `Ctrl+I` (Italic), `Ctrl+U` (Underline), `Ctrl+L` (Bullet list / alignment).

## Acceptance Criteria
- [ ] Hovering header reveals both `+` (top-left) and `X` (top-right).
- [ ] Clicking `+` creates a new note adjacent to the active note.
- [ ] Double-clicking title enables inline renaming; pressing Enter saves it.
- [ ] Right-clicking reveals 6 pastel color choices; choosing updates note color in `Notes.json`.
- [ ] Keyboard shortcuts `Ctrl+B`, `Ctrl+I`, `Ctrl+U`, `Ctrl+L` format text without visual toolbars.
- [ ] 'X' button obeys configured action (delete, close, or ask).
