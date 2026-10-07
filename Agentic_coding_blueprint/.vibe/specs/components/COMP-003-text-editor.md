# COMP-003: Text Editor Area (`TextEditor.slint`)

## Purpose
The primary content area of the sticky note offering zero-clutter text entry.

## Properties & Specifications
- Background: `#FFF8D6` (Warm canary yellow, customizable per note or config).
- Padding: `8px 12px 12px 12px`.
- Text Wrapping: `WordWrap` enabled.
- Font Family: System native sans-serif (`Segoe UI`, `SF Pro`, `Roboto`).
- Font Size: 14px default (scalable via settings or shortcuts).
- Line Height: 1.45 for optimal legibility.
- Auto-Save Trigger: On text modification, dispatch debounced change event (300ms idle threshold).
