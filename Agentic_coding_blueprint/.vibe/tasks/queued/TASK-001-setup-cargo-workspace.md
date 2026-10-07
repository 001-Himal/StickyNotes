# TASK-001: Setup Cargo Workspace Structure

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 1

## Description
Initialize the root `Cargo.toml` workspace and crate subdirectories:
- `crates/sticky-note-core/` (Library)
- `crates/sticky-note/` (Binary)
- `crates/sticky-note-settings/` (Binary)

## Acceptance Criteria
- [ ] Root `Cargo.toml` configured with workspace members.
- [ ] `cargo check` compiles successfully across all three crates.
- [ ] Slint build dependency wired into build scripts.
