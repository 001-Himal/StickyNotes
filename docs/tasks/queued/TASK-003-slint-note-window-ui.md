# TASK-003: Design and Implement Slint Note Window UI

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 1

## Description
Develop `ui/note_window.slint` with the physical sticky note design system:
1. Warm paper background with subtle ambient shadow and canonical 6 two-tone pastel palettes from `design-system.md` (Yellow, Blue, Green, Pink, Purple, White).
2. Header component (28px) with title label, double-click inline editor, hover-only `+` quick-add button, and hover-only `X` close button (both 100% invisible when not hovered).
3. Bottom-right corner component supporting both authentic Curled / Bended dog-ear corner (22×22px with subtle under-curl shadow) and Flat modern corner with subtle hover grip lines.
4. Multi-line plain text editing area with smooth word wrapping, right-click 6-color palette context menu, and keyboard formatting (`Ctrl+B`, `Ctrl+I`, `Ctrl+U`, `Ctrl+L`).

## Acceptance Criteria
- [ ] Slint UI renders cleanly with authentic two-tone pastel paper aesthetic.
- [ ] Header hover reveals both `+` (quick-add) and `X` (close); invisible when not hovered.
- [ ] Bottom-right corner switches between Curled and Flat appearance based on config.
- [ ] Bottom-right resize grip is visible only on corner hover.
- [ ] Right-click displays 6 pastel color swatches matching design system.
- [ ] Keyboard shortcuts format plain text without visual ribbons.
