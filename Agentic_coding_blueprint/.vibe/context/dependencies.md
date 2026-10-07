# dependencies.md

| Crate / Package | Version | Purpose | Security / Performance Impact |
|---|---|---|---|
| `slint` | 1.8+ | Native cross-platform desktop UI framework | Zero browser engine, native GPU/software rendering, ~15MB RAM |
| `slint-build` | 1.8+ | Build script helper compiling `.slint` files to Rust code | Build-time only |
| `serde` | 1.0+ | Serialization framework (`derive` feature) | Zero runtime overhead |
| `serde_json` | 1.0+ | Reading and writing JSON files | Fast, lightweight serializer |
| `global-hotkey` | 0.5+ | Registering system-wide global key shortcuts | Low-level OS hook |
| `windows-sys` (Windows) | 0.52+ | Win32 API calls (`HWND_BOTTOM`, `WS_EX_TOOLWINDOW`, Progman) | Zero-cost FFI bindings |
| `tray-icon` | 0.17+ | System tray icon for preferences and quick new note | Native OS tray hook |
| `tempfile` | 3.10+ | Temporary files for atomic disk writes | Safety against partial write crashes |
