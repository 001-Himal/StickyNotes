# TASK-005: Window Dragging, Resizing and Dynamic Size Inheritance

- **Status:** Completed
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 2

## Description
1. Handle header mouse drag events to update window position in real time across multiple monitors.
2. Handle bottom-right corner mouse drag events for single-direction resizing (top-left stays anchored, bottom and right expand outward, clamped to min 180×120).
3. Connect geometry changes to debounced save queue so position and size persist in `Notes.json` and individual note files.
4. **Dynamic Size Inheritance:** When a note is resized, update `last_used_width` and `last_used_height` in `config.json` so newly spawned notes automatically inherit the last resized dimensions.
5. **Multi-Monitor Bounds Validation:** On startup and display changes, verify note bounding box intersects an active monitor's work area; clamp to primary monitor if a previously used secondary monitor was disconnected.

## Acceptance Criteria
- [x] Dragging header moves the window smoothly across monitors.
- [x] Resizing is single-direction (top-left anchored, right/down expanding) with min 180×120 clamp.
- [x] Position and size persist across process restarts.
- [x] Newly created notes default to the dimensions of the most recently resized note.
- [x] Notes do not get orphaned off-screen when monitors are disconnected.
