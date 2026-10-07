# TASK-006: Header Controls, Quick Add, Note 1/2 Titles & Close Actions

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 2

## Description
1. Implement double-click event on header label to toggle into inline title editor.
2. Title naming defaults to sequential `Note 1`, `Note 2`, `Note 3`, etc.
3. If header text is empty or cleared with whitespace, display placeholder `"Write title here..."` or revert to `"Note N"`.
4. Implement hover-only disclosure for `+` (top-left) and `X` (top-right) (both are 100% invisible when not hovered).
5. Connect `+` button click to spawn and cascade a new note immediately.
6. Wire up close button ('X') click event to execute action specified in `config.json`:
   - `delete`: Removes note permanently from `Notes.json` and disk.
   - `close`: Marks note closed in `Notes.json` and dismisses window.
   - `ask`: Displays a small themed prompt dialog: *"Delete note or just close?"*

## Acceptance Criteria
- [ ] New notes automatically named `Note 1`, `Note 2`, etc.
- [ ] Double clicking header renames note; blank input retains placeholder or falls back to `Note N`.
- [ ] Hovering header reveals `+` and `X`; hidden when not hovered.
- [ ] Clicking `+` creates a new note adjacent to the active one.
- [ ] Close button action obeys user setting (delete, close, or ask prompt).
