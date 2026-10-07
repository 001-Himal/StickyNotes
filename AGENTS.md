# Project AI Instructions

Read `README.md` first, then `.vibe/README.md`.

Before modifying code:
1. Read relevant context in `.vibe/context/` and specs in `.vibe/specs/`.
2. Follow rules in `.vibe/rules/` and architectural decisions in `.vibe/decisions/`.
3. Plan before implementing; adhere strictly to the approved PRD and user choices.
4. Keep changes scoped; respect `.vibe/config/protected.yaml` and `.vibe/config/permissions.yaml`.
5. Update tracking files as work progresses (`planning/current.md`, `tasks/`, `decisions/`).
6. Verify: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`; review diffs.
7. Never install web runtimes (Electron/Node/WebViews); maintain desktop-layer widget behavior (`HWND_BOTTOM`, `WS_EX_TOOLWINDOW`).
8. Zero cloud/network dependencies; all persistence remains local and atomic.

