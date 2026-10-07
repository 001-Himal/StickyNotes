# Checklist: security (Local Desktop Native)
- [ ] Safe file path resolution: Strictly bounded to `./Sticky Note Notes/` (no directory traversal)
- [ ] Atomic file writes: Temporary file `.tmp` write + atomic rename (no crash corruption)
- [ ] Zero network access: No HTTP/WebSocket sockets, no telemetry, no analytics
- [ ] Input bounds: Memory clamp on extreme text paste sizes
- [ ] Rust safety: Audited `unsafe` blocks in platform FFI (`windows-sys`, `cocoa`, `x11rb`)
- [ ] Dependencies audited via `cargo audit`
