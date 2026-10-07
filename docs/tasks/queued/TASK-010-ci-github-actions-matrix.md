# TASK-010: CI GitHub Actions Build and Release Matrix

- **Status:** Queued
- **Priority:** Low
- **Owner:** Himal
- **Target:** Milestone 4

## Description
Configure `.github/workflows/ci.yml` and `release.yml`:
1. Build matrix across `windows-latest`, `macos-latest`, and `ubuntu-latest`.
2. Run `cargo check`, `cargo test`, `cargo clippy`, and `cargo fmt`.
3. Package release binaries (`Sticky Note.exe`, `Sticky Note Settings.exe` etc.).

## Acceptance Criteria
- [ ] GitHub Actions passes automated builds across Windows, macOS, and Linux.
- [ ] Artifacts packaged as clean zip/tarballs.
