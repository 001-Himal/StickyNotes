# TASK-002: Implement Core Models and Atomic Storage

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 1

## Description
In `crates/sticky-note-core/`:
1. Define `Note` and `NotesDocument` structs with `serde::Serialize` and `Deserialize`.
2. Define `Config` struct for application settings.
3. Implement `StorageEngine` with atomic write helper (`.tmp` write followed by rename).
4. Implement auto-increment title generator (`Untitled Note {N}`).
5. Add unit tests for serialization, corruption recovery, and atomic persistence.

## Acceptance Criteria
- [ ] Unit tests pass for reading, writing, and atomic renaming.
- [ ] Corrupted files fail safely and recover.
- [ ] Title generator produces sequential titles based on existing notes.
