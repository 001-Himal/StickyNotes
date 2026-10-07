# COMP-002: Header Bar (`HeaderBar.slint`)

## Purpose
The draggable title bar of the note providing window movement, inline renaming, quick-add (`+`), and close (`X`) disclosure.

## Structure
- Height: Fixed 28px.
- Left edge: Hover-revealed `+` button (`18×18px`, top-left corner).
- Center: Title text label (`12px semi-bold`, charcoal `#2B2B2B`).
- Right edge: Hover-revealed `X` button (`18×18px`, top-right corner).
- Background: Matches the note's header accent tone (e.g. `#F5E8A9` for yellow note).

## Interactions
1. **Window Drag:** Mouse down on header area (excluding buttons) initiates smooth OS window dragging.
2. **Top-Left `+` Quick Add Button:**
   - Hidden by default; fades into view when mouse cursor enters the header.
   - Clicking `+` immediately spawns a new note adjacent to the active note.
3. **Double-Click Title Rename:**
   - Double-clicking header text activates inline `TextInput`.
   - `Enter` commits title; `Escape` cancels; focus lost commits changes.
4. **Top-Right `X` Close Button:**
   - Hidden by default; fades into view on header hover.
   - Click action follows `config.json` preference:
     - `delete`: Removes note from `Notes.json`.
     - `close`: Hides note window while preserving text in `Notes.json`.
     - `ask`: Opens small themed prompt: *"Delete note or just close?"*
