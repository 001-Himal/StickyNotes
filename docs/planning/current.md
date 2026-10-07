# current.md

## Current Focus
- Session: Implementation Kickoff (Milestone 1).
- Status: TASK-001, TASK-002, TASK-003, and TASK-004 completed. Currently activating TASK-005: Window Dragging, Resizing and Dynamic Size Inheritance.
- Active Task: `docs/tasks/queued/TASK-005-note-drag-resize-and-geometry.md`

## Completed
- [x] TASK-001: Cargo Workspace Initialization (`sticky-note-core`, `sticky-note`, `sticky-note-settings`, Slint build integration).
- [x] TASK-002: Core models, 6 authentic pastel palettes, atomic storage engine with reconciliation, and local single-instance IPC channel.
- [x] TASK-003: Design and implement Slint Note Window UI (`ui/note_window.slint` with two-tone pastel cards, hover-only `+`/`X` controls, curled vs. flat corner, 6-color right-click palette, and keyboard formatting).
- [x] TASK-004: Windows native desktop layer (`HWND_BOTTOM`), stealth windowing (`WS_EX_TOOLWINDOW` to omit from Taskbar & Alt+Tab), and multi-monitor bounds validation.

## Next Up
- Execute TASK-005: Note drag, resize, and per-note geometry persistence with dynamic size inheritance.
- Execute TASK-006: Header rename and hover controls lifecycle.

