# architecture.md

- Respect component boundaries in `.vibe/context/architecture.md`.
- Two-executable architecture: `Sticky Note` (manager) and `Sticky Note Settings` (utility) must remain strictly decoupled.
- Shared logic lives exclusively in `crates/sticky-note-core/`.
- UI rendering code must never execute blocking disk I/O on the main GUI thread.
- Never add unnecessary runtime layers (no async runtimes, no browser webviews).
- Proposed architectural shifts require an accepted ADR in `.vibe/decisions/`.
