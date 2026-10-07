# Checklist: production readiness
- [x] `cargo check`, `cargo clippy -- -D warnings`, and `cargo test` pass with zero warnings
- [x] Release binaries compiled with `cargo build --release` (LTO enabled, stripped)
- [x] Idle memory verified below 50 MB (target: 15–30 MB)
- [x] Idle CPU measured at 0.0%
- [x] Cold startup measured <1.0 second
- [x] Tested on multi-monitor setup and various DPI scaling factors (100%, 125%, 150%)
- [x] Window layering verified: Notes stay behind foreground apps, invisible in taskbar/Alt+Tab

