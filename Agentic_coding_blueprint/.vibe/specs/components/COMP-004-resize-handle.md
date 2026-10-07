# COMP-004: Hover Resize Handle (`ResizeHandle.slint`)

## Purpose
The bottom-right corner grip for intuitive note resizing.

## Specifications
- Location: Bottom-right corner of `NoteWindow`.
- Hit Target Dimensions: 20 × 20 px.
- Visual Appearance: Three subtle diagonal grip lines (`#A8A38B`).
- Visibility: `opacity: 0.0` default; `opacity: 1.0` when cursor hovers within the 20×20px region.
- Cursor: Changes to `nwse-resize` on hover and during active resize drag.
- Clamping: Minimum window constraint enforced at 180 × 120 px.
