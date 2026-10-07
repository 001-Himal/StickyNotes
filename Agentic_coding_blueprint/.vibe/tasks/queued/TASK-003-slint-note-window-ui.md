# TASK-003: Design and Implement Slint Note Window UI

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 1

## Description
Develop `ui/note_window.slint` with the physical sticky note design system:
1. Warm paper background with subtle ambient shadow and 6-color pastel theme support (Yellow, Green, Blue, Purple, Pink, White).
2. Header component with title label, double-click inline editor, hover `+` quick-add button, and hover `X` close button.
3. Multi-line plain text editing area with smooth word wrapping, right-click context color picker, and zero-toolbar keyboard formatting (`Ctrl+B`, `Ctrl+I`, `Ctrl+U`, `Ctrl+L`).
4. Bottom-right corner resize grip that reveals on hover.

## Acceptance Criteria
- [ ] Slint UI renders cleanly with pastel paper aesthetic.
- [ ] Header hover reveals both `+` (quick-add) and `X` (close).
- [ ] Bottom-right grip is visible only on corner hover.
- [ ] Right-click displays 6 pastel color swatches.
