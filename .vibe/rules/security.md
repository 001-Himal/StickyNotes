# security.md (Desktop Native)

- **Local Path Containment:** All file reading/writing must be strictly contained within the `./Sticky Note Notes/` folder or root `config.json`. Reject path traversal (`../`).
- **Atomic Operations:** Always write to a `.tmp` file and perform an atomic rename to prevent file corruption during system power cuts.
- **Zero Network Operations:** No outbound sockets, no telemetry, no analytics, no external update checkers.
- **Memory Safety:** Audit all `unsafe` blocks required for Windows Win32, Cocoa, or X11 FFI bindings; ensure sound pointer handling.
- **Dependency Vetting:** All third-party crates must be audited via `cargo audit` before addition.
