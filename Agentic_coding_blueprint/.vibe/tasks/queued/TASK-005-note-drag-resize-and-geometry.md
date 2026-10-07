# TASK-005: Window Dragging, Resizing and Geometry Persistence

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 2

## Description
1. Handle header mouse drag events to update window position in real time.
2. Handle bottom-right corner mouse drag events to update window dimensions (minimum 180x120).
3. Connect geometry changes to debounced save queue so position and size persist in `Notes.json`.

## Acceptance Criteria
- [ ] Dragging header moves the window smoothly across monitors.
- [ ] Dragging corner resizes cleanly with correct min-width/height clamping.
- [ ] Position and size persist across process restarts.
