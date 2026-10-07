# TASK-002: Implement Core Models, Per-Note Storage & File Launching

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 1

## Description
In `crates/sticky-note-core/`:
1. Define `Note` and `NotesDocument` structs with `serde::Serialize` and `Deserialize` (including `id`, `filename`, `title`, `content`, `geometry`, `color`, timestamps, and `is_closed`).
2. Define `Config` struct for application settings (including `last_used_width`, `last_used_height`, `corner_style`, `close_action`, `font_family`, `font_size`, `default_color`).
3. Implement `StorageEngine` with atomic write helper (`.tmp` write in same directory, `sync_all`, followed by atomic rename).
4. Save notes both in `Notes.json` index (startup cache) and as accessible individual files (`Note 1.json`, `Note 2.json`, or `{Sanitized_Title}.json`) in `Sticky Note Notes/`.
5. Implement filename sanitization (stripping illegal OS characters `\ / : * ? " < > |`) and collision handling (`Title (2).json`).
6. Implement resilient reconciliation: rebuild `Notes.json` if missing from scanning individual files, and honor external file edits if file `mtime` > index `updated_at`.
7. Implement single-instance lock and local IPC listener (`\\.\pipe\StickyNote_IPC` via `interprocess` crate):
   - Secondary instance sends `OPEN_FILE <path>` or `NEW_NOTE` and exits immediately.
   - Primary instance handles commands to open, restore, or create notes.
8. Add unit tests for models, atomic writing, title sanitization, reconciliation, and IPC command parsing.

## Acceptance Criteria
- [ ] Unit tests pass for reading, writing, atomic renaming, and corrupted file fallback.
- [ ] Title generator produces `Note 1`, `Note 2`, `Note 3` sequentially and sanitizes file paths.
- [ ] Individual note files are created and loadable via CLI path argument and IPC.
- [ ] Secondary process cleanly hands off file path to primary instance and exits.
