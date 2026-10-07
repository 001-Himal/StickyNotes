# Skill: security-review

Review diff for desktop safety (do NOT modify code):
1. **File Operations:** Are file paths safely sandboxed to `./Sticky Note Notes/`? No directory traversal vulnerabilities?
2. **Atomic Writes:** Are writes performed using `.tmp` files and atomic rename to prevent corruption?
3. **Unsafe FFI Blocks:** Are `unsafe` blocks in Windows/macOS/Linux platform layers properly bounded and sound?
4. **Network Access:** Ensure zero sockets, HTTP requests, or external telemetry connections exist.
5. **Denial of Service:** Is memory properly clamped against huge multi-megabyte clipboard pastes?

Report: findings, severity, affected files, recommended fix.
