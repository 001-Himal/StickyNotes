# TASK-007: Global Hotkey Integration

- **Status:** Completed
- **Priority:** Medium
- **Owner:** Himal
- **Target:** Milestone 2

## Description
1. Integrate `global-hotkey` crate in `crates/sticky-note/src/hotkeys.rs`.
2. Register `Ctrl+Alt+N` for new note creation.
3. Register `Ctrl+Alt+S` to spawn `Sticky Note Settings.exe`.
4. Run event listener without causing CPU wake-ups when idle.

## Acceptance Criteria
- [x] Pressing `Ctrl+Alt+N` creates a new note while another application has focus.
- [x] Pressing `Ctrl+Alt+S` launches settings.
- [x] Idle CPU remains ~0.0%.
