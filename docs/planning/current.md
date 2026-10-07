# current.md

## Current Focus
- Session: Polish & Packaging (Windows Release Ready).
- Status: All core Windows milestones (M1 through M5) are completed. All 10 tasks implemented and verified with zero warnings and passing tests.
- Active Task: None (All Windows release tasks completed).

## Completed
- [x] TASK-001: Cargo Workspace Initialization (`sticky-note-core`, `sticky-note`, `sticky-note-settings`, Slint build integration).
- [x] TASK-002: Core models, 6 authentic pastel palettes, atomic storage engine with reconciliation, and local single-instance IPC channel.
- [x] TASK-003: Design and implement Slint Note Window UI (`ui/note_window.slint` with two-tone pastel cards, hover-only `+`/`X` controls, curled vs. flat corner, 6-color right-click palette, and keyboard formatting).
- [x] TASK-004: Windows native desktop layer (`HWND_BOTTOM`), stealth windowing (`WS_EX_TOOLWINDOW` to omit from Taskbar & Alt+Tab), and multi-monitor bounds validation.
- [x] TASK-005: Window Dragging, Resizing and Dynamic Size Inheritance (`last_used_width`/`height` in `config.json`).
- [x] TASK-006: Header Controls, Quick Add cascade, sequential `Note 1/2` naming, double-click inline rename with placeholder & Escape cancellation, configurable close actions (delete, close, ask modal).
- [x] TASK-007: Global Hotkey Integration (`Ctrl+Alt+N` creates note, `Ctrl+Alt+S` launches settings, idle CPU ~0.0%).
- [x] TASK-008: Settings Executable UI (`ui/settings_window.slint`), config atomic persistence, and IPC `RELOAD_CONFIG` real-time sync.
- [x] TASK-010: CI GitHub Actions Build and Windows Release Workflow (`.github/workflows/ci.yml`, `.github/workflows/release.yml`, release binaries built with LTO and stripping).
- [x] TASK-011: Windows Runtime UX & Rendering Bugfixes (suppressed console window via `#![windows_subsystem = "windows"]`, fixed Alt+Tab leaks & initial z-order via `EnumWindows` and `SWP_FRAMECHANGED`, resolved global hotkeys via dedicated message pump thread, fixed in-editor `Ctrl+B/I/U/L` shortcuts by handling ASCII control codes, replaced tofu close glyph with vector `Path` cross).
- [x] TASK-012: Authentic Handwritten Theme, Seamless Canvas & Dynamic Text Modifiers (eliminated Slint `TextEdit` dark focus box, seamless transparent paper canvas in all themes, default Windows 7 cursive font `Segoe Print`, persistent `is_bold`, `is_italic`, `is_underlined` ruled paper guidelines, clean in-editor shortcuts without markdown string corruption, header right-click color palette, font size scaling shortcuts, settings window `#2B2B2B` high-contrast checkbox/input styling without white-on-yellow contrast leaks, directory-scoped IPC named pipe, and foreground window activation on double-click launch).

## Blocked / Deferred
- [ ] TASK-009: Cross-platform macOS and Linux windowing abstractions (Deferred to focus on Windows native release).

## Next Up
- Package updated binaries to `dist/Sticky Note/` and finalize release verification.







