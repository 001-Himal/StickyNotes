# TASK-006: Header Controls, Quick Add and Configurable Close Actions

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 2

## Description
1. Implement double-click event on header label to toggle into inline title editor.
2. Implement header hover revealing `+` (top-left) and `X` (top-right).
3. Connect `+` button click to spawn and cascade a new note immediately.
4. Wire up close button ('X') click event to execute action specified in `config.json`:
   - `delete`: Removes note permanently from `Notes.json`.
   - `close`: Marks note closed in `Notes.json` and dismisses window.
   - `ask`: Displays a small themed prompt dialog: *"Delete note or just close?"*

## Acceptance Criteria
- [ ] Double clicking header renames note; saved to `Notes.json`.
- [ ] Hovering header reveals `+` and `X`.
- [ ] Clicking `+` creates a new note adjacent to the active one.
- [ ] Close button action obeys user setting (delete, close, or ask prompt).
