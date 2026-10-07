# Project AI Instructions

Read `README.md` first, then `docs/README.md`.

Before modifying code:
1. Read relevant context in `docs/context/` and specs in `docs/specs/`.
2. Follow rules in `docs/rules/` and architectural decisions in `docs/decisions/`.
3. Plan before implementing; adhere strictly to the approved PRD and user choices.
4. Keep changes scoped; respect `docs/config/protected.yaml` and `docs/config/permissions.yaml`.
5. Update tracking files as work progresses (`docs/planning/current.md`, `docs/tasks/`, `docs/decisions/`).
6. Verify: `cargo check`, `cargo clippy -- -D warnings`, `cargo test`; review diffs.
7. Never install web runtimes (Electron/Node/WebViews); maintain desktop-layer widget behavior (`HWND_BOTTOM`, `WS_EX_TOOLWINDOW`).
8. Zero cloud/network dependencies; all persistence remains local and atomic.

