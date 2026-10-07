# TASK-001: Setup Cargo Workspace Structure

- **Status:** Completed
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 1

## Description
Initialize the root `Cargo.toml` workspace and crate subdirectories:
- `crates/sticky-note-core/` (Library)
- `crates/sticky-note/` (Binary)
- `crates/sticky-note-settings/` (Binary)

## Acceptance Criteria
- [x] Root `Cargo.toml` configured with workspace members.
- [x] `cargo check` compiles successfully across all three crates.
- [x] Slint build dependency wired into build scripts.
