# stack.md

| Layer | Technology | Version | Notes |
|---|---|---|---|
| Core Language | Rust | 1.80+ (2021 edition) | Memory safe, zero garbage collection, near-zero CPU |
| GUI Framework | Slint | 1.8+ | Native GPU/software renderer, low memory, cross-platform |
| Window Management | Platform APIs (`windows-sys`, `cocoa`, `x11rb`) | Native | Desktop-level widget placement, toolwindow flags, no taskbar |
| Global Shortcuts | `global-hotkey` | 0.5+ | System-wide keyboard listeners (Ctrl+Alt+N, Ctrl+Alt+S) |
| Serialization | `serde` + `serde_json` | 1.0+ | Robust, atomic JSON serialization & deserialization |
| Data Storage | Local Filesystem | JSON format | Portable, human-readable in `Sticky Note Notes/` |
| System Tray | `tray-icon` | 0.17+ | Optional minimal background access to Settings & New Note |
| Build System | Cargo (Workspace) | Latest Stable | Two binaries: `sticky-note` & `sticky-note-settings` |
| CI/CD | GitHub Actions | Modern | Automated multi-platform cross-compilation matrix |

Hard rules: no new dep without approval; no stack swaps for popularity; zero network or telemetry dependencies.
