# anti-feature-creep.md

## Strict Anti-Bloat Manifesto for Sticky Note

Any proposed feature or dependency must pass all five filters before being considered:

1. **Does a physical sticky note do this?** If physical sticky notes don't have it (e.g. cloud accounts, collaborative real-time sync cursors, kanban boards), default answer is NO.
2. **Does it add background CPU or RAM?** If it increases idle memory above 50 MB or causes idle CPU ticks, REJECT IT.
3. **Does it clutter the note interface?** The note must have zero permanent buttons or formatting ribbons. If it needs a button on the note, REJECT IT.
4. **Does it require network access?** Absolutely zero outbound network connections. If a feature needs the internet, REJECT IT.
5. **Can it live in Sticky Note Settings instead?** If it's configuration or machinery, keep it out of the note window.

### Specifically Banned Features
- 🚫 Markdown rendering panes / WYSIWYG toolbars
- 🚫 User authentication / cloud backup
- 🚫 Webview / Chromium / Electron runtime embedding
- 🚫 Always-on-top window pin buttons (violates the desktop-layer widget principle)
- 🚫 Complex relational databases (SQLite / Postgres)
- 🚫 Telemetry / analytics tracking
