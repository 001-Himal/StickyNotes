# FEAT-003 — Atomic Local Persistence

## Goal
Store all notes locally within `Sticky Note Notes/Notes.json` with 100% data integrity, debounced saves, and atomic temporary-file writes.

## Schema: `Notes.json`
```json
{
  "version": 1,
  "last_updated": 1728280000,
  "notes": [
    {
      "id": "note-001",
      "title": "Note 1",
      "content": "Meeting notes at 3pm\nBuy groceries",
      "x": 420,
      "y": 180,
      "width": 300,
      "height": 220,
      "color": "yellow",
      "created_at": 1728280000,
      "updated_at": 1728280050,
      "is_closed": false
    }
  ]
}
```

## Requirements
1. **Directory Location**: All data stored under `./Sticky Note Notes/` relative to application root.
2. **Per-Note Files & Index**: In addition to `Notes.json` index, save each note as an individual file (e.g. `Note 1.json`, `Note 2.json`, ...) in `Sticky Note Notes/`.
3. **Explorer Double-Click Launch**: Double-clicking any note file in Windows File Explorer (or passing it via CLI `Sticky Note.exe "Note 1.json"`) signals `Sticky Note` to immediately open/restore that note on the desktop. If already running, signals the existing background process via single-instance IPC.
4. **Debounced Writing**: On user text changes, debounce disk write by 300ms to eliminate unnecessary disk thrashing.
5. **Atomic File Write**: Always write to `.tmp` files and perform an atomic rename/replace to prevent corruption on sudden power loss or process kill.
6. **Resilience**: If files are corrupted or absent, back them up as `.bak` and reinitialize gracefully.

## Acceptance Criteria
- [ ] Notes created and edited persist across app restart.
- [ ] Geometry (x, y, width, height) persists accurately.
- [ ] Individual note files exist in `Sticky Note Notes/` (`Note 1.json`, etc.).
- [ ] Double-clicking a note file in File Explorer opens that note in `Sticky Note`.
- [ ] Writing to disk uses atomic rename.
- [ ] No file corruption occurs even if process is killed mid-session.
