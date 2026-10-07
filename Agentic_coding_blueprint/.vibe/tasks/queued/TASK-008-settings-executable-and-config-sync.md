# TASK-008: Settings Executable and Config Sync

- **Status:** Queued
- **Priority:** Medium
- **Owner:** Himal
- **Target:** Milestone 3

## Description
1. Create `crates/sticky-note-settings/` application.
2. Build `ui/settings_window.slint` with tabs/sections for General, Note Appearance, Behavior, and Shortcuts.
3. Read `config.json` on launch, write atomic update on user change.
4. Notify running `sticky-note` process (or use file watch/polling fallback) to apply new preferences.
5. Exit immediately when closed.

## Acceptance Criteria
- [ ] `Sticky Note Settings.exe` runs independently.
- [ ] Saves user preferences cleanly to `config.json`.
- [ ] Closes cleanly without residual background processes.
