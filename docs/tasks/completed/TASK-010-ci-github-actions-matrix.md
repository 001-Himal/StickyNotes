# TASK-010: CI GitHub Actions Build and Windows Release Workflow

- **Status:** Completed
- **Priority:** Medium
- **Owner:** Himal
- **Target:** Milestone 3

## Description
Configure `.github/workflows/ci.yml` and `.github/workflows/release.yml` for Windows:
1. Automated CI pipeline on `push` and `pull_request` against `main`.
2. Windows runner (`windows-latest`) with Rust stable toolchain.
3. Automated validation steps:
   - `cargo check --all-targets`
   - `cargo clippy --all-targets -- -D warnings`
   - `cargo test --all-targets`
4. Release workflow building optimized production binaries (`sticky-note.exe`, `sticky-note-settings.exe`) with LTO and stripping enabled.
5. Automated release asset packaging into a clean zip archive.

## Acceptance Criteria
- [x] GitHub Actions CI workflow created and syntactically valid.
- [x] Validates `check`, `clippy`, and `test` on Windows runner.
- [x] Release workflow packages Windows binaries and default configuration.
