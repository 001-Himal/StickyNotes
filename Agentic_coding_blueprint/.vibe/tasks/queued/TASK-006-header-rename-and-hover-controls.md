# TASK-006: Header Double-Click Renaming & Configurable Close Actions

- **Status:** Queued
- **Priority:** High
- **Owner:** Himal
- **Target:** Milestone 2

## Description
1. Implement double-click event on header label to toggle into inline text editor.
2. Commit renamed title on `Enter` or focus-out.
3. Wire up close button ('X') click event to execute action specified in `config.json` (`delete`, `close`, or `hide`).
4. If `confirm_before_delete` is enabled in config, show quick confirmation dialog before deleting.

## Acceptance Criteria
- [ ] Double clicking header renames note; saved to `Notes.json`.
- [ ] Close button action obeys user setting.
