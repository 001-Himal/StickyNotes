# TASK-005: Window Dragging, Resizing and Dynamic Size Inheritance

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 2

## Description
1. Handle header mouse drag events to update window position in real time.
2. Handle bottom-right corner mouse drag events to update window dimensions (minimum 180×120).
3. Connect geometry changes to debounced save queue so position and size persist in `Notes.json`.
4. **Dynamic Size Inheritance:** When a note is resized, update `last_used_width` and `last_used_height` in `config.json` so newly spawned notes automatically inherit the last resized dimensions.

## Acceptance Criteria
- [ ] Dragging header moves the window smoothly across monitors.
- [ ] Dragging corner resizes cleanly with correct min-width/height clamping.
- [ ] Position and size persist across process restarts.
- [ ] Newly created notes default to the dimensions of the most recently resized note.
