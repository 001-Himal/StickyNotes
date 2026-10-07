# COMP-004: Corner Resize Handle & Bended Paper Look (`ResizeHandle.slint`)

## Purpose
The bottom-right corner component providing both the visual paper aesthetic and the interactive drag-to-resize mechanism.

## Visual Modes (Configurable in Settings)

### 1. Curled / Bended Look (Classic Post-It Peel Aesthetic)
- **Geometry:** 22 × 22 px triangular corner curl in the bottom-right.
- **Flap Rendering:**
  - Backside of paper flap rendered in a slightly lighter or inverted gradient tone.
  - Soft curved shadow cast beneath the curl (`offset-y: 2px`, `blur: 4px`, `alpha: 0.18`), giving the tangible illusion of a sticky note peeling off the screen.
- **Hover Reaction:** On hover, the curl gently elevates by 1px and cursor switches to `nwse-resize`.

### 2. Normal / Flat Look (Modern Minimalist)
- **Geometry:** Flat flush corner matching the window's 4px border radius.
- **Indicator:** Three subtle diagonal grip lines (`#A8A38B`) that smoothly transition from `opacity: 0.0` to `opacity: 1.0` when cursor hovers within the 20×20px corner zone.

## Interaction & Resizing Logic
- Hit target: Bottom-right 22 × 22 px region.
- Cursor: `nwse-resize` across both visual modes.
- Dragging: Adjusts `width` and `height` of the note window in real time.
- Min Clamping: Enforces minimum width 180px, minimum height 120px.
- Persistence: On mouse release, note geometry is persisted in `Notes.json` and default new note size is updated in `config.json`.
