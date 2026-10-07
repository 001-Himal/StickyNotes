# FEAT-002 — Header Rename & Hover Controls

## Goal
Implement physical sticky note micro-interactions: clean title display with double-click inline renaming and hover-only disclosure of the close button and resize handle.

## Requirements
1. **Title Auto-Increment**:
   - New notes automatically receive the title `"Untitled Note 1"`.
   - If `"Untitled Note 1"` is already taken, increment to `"Untitled Note 2"`, etc.
2. **Double-Click Rename**:
   - Double-clicking the header text swaps it to a native inline input field.
   - Pressing `Enter` or losing focus commits the new title string.
   - Title is updated in memory and serialized to `Notes.json`.
3. **Hover-Only Close Button ('X')**:
   - Default state: `opacity: 0` (completely invisible).
   - Mouse enters header zone: smooth transition to visible in top-right corner.
   - Clicking 'X' triggers configured close action (Delete / Close / Hide).
4. **Hover-Only Resize Handle**:
   - Default state: invisible.
   - Mouse enters bottom-right 20×20 area: diagonal grip icon fades in.

## Acceptance Criteria
- [ ] First note created is titled "Untitled Note 1"; second note is "Untitled Note 2".
- [ ] Double-clicking title enables inline renaming; pressing Enter saves it.
- [ ] Close button ('X') is completely invisible until cursor enters header.
- [ ] Bottom-right resize handle is completely invisible until hovered.
