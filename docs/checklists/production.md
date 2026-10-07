# Checklist: production readiness
- [ ] `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` pass with zero warnings
- [ ] Release binaries compiled with `cargo build --release` (LTO enabled, stripped)
- [ ] Idle memory verified below 50 MB (target: 15–30 MB)
- [ ] Idle CPU measured at 0.0%
- [ ] Cold startup measured <1.0 second
- [ ] Tested on multi-monitor setup and various DPI scaling factors (100%, 125%, 150%)
- [ ] Window layering verified: Notes stay behind foreground apps, invisible in taskbar/Alt+Tab
