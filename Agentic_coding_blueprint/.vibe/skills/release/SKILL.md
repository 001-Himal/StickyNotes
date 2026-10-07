# Skill: release

1. Verify `cargo test`, `cargo clippy -- -D warnings`, and `cargo check` all pass.
2. Verify release build with `cargo build --release` produces optimized binaries.
3. Verify memory usage is <50 MB and idle CPU is 0.0%.
4. Run desktop release checklist in `.vibe/checklists/release.md`.
5. Package binaries (`Sticky Note.exe`, `Sticky Note Settings.exe`) with checksums.
