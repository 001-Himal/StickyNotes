# TASK-002: Implement Core Models, Per-Note Storage & File Launching

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 1

## Description
In `crates/sticky-note-core/`:
1. Define `Note` and `NotesDocument` structs with `serde::Serialize` and `Deserialize` (including `color` and `(width, height)`).
2. Define `Config` struct for application settings (including `last_used_width`, `last_used_height`, `corner_style`, `close_action`).
3. Implement `StorageEngine` with atomic write helper (`.tmp` write followed by rename).
4. Save notes both in `Notes.json` index and as accessible individual files (`Note 1.json`, `Note 2.json`, ...).
5. Implement auto-increment title generator (`Note {N}`) with placeholder fallback.
6. Support CLI file path loading (`Sticky Note.exe "path/to/note.json"`) for Windows File Explorer double-click launching.
7. Add unit tests for serialization, file launching, and atomic persistence.

## Acceptance Criteria
- [ ] Unit tests pass for reading, writing, and atomic renaming.
- [ ] Title generator produces `Note 1`, `Note 2`, `Note 3` sequentially.
- [ ] Individual note files are created and loadable via CLI path argument.
