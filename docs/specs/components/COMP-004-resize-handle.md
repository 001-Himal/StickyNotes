# COMP-004: Corner Resize Handle & Bended Paper Look (`ResizeHandle.slint`)

## Purpose
The bottom-right corner component providing both the visual paper aesthetic and the interactive drag-to-resize mechanism.

## Single-Direction Resizing Rule
- Resizing operates strictly in a **single outward direction**: pulling to the right increases width; pulling downward increases height.
- The top-left corner of the note window remains firmly anchored in place on the desktop wallpaper.
- Minimum size clamp: 180px width, 120px height.

## Hover-Only Disclosure
- The bottom-right corner resize indicator / grip is **100% invisible** during normal ambient desktop display.
- Only when the cursor hovers directly within the 22×22px bottom-right corner region does the resize indicator smoothly fade in, switching cursor to `nwse-resize`.

## Visual Modes (Configurable in Settings)

### 1. Curled / Bended Look (Classic Post-It Peel Aesthetic)
- **Geometry:** 22 × 22 px triangular corner curl in the bottom-right.
- **Flap Rendering:**
  - Backside of paper flap rendered in a slightly lighter or inverted gradient tone.
  - Soft curved shadow cast beneath the curl (`offset-y: 2px`, `blur: 4px`, `alpha: 0.18`), giving the tangible illusion of a sticky note peeling off the desktop wallpaper.
- **Hover Reaction:** On hover, the curl gently elevates by 1px and cursor switches to `nwse-resize`.

### 2. Normal / Flat Look (Modern Minimalist)
- **Geometry:** Flat flush corner matching the window's 4px border radius.
- **Indicator:** Three subtle diagonal grip lines (`#A8A38B`) that smoothly transition from `opacity: 0.0` to `opacity: 1.0` when cursor hovers within the corner zone.

## Interaction & State Sync
- Dragging: Adjusts `width` and `height` in real time.
- Mouse Release: Note dimensions saved to `Notes.json` and default new note size updated in `config.json`.
