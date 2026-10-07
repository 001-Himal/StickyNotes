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
      "title": "Untitled Note 1",
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
2. **Debounced Writing**: On user text changes, debounce disk write by 300ms to eliminate unnecessary disk thrashing.
3. **Atomic File Write**: Always write to `Notes.json.tmp` and perform an atomic rename/replace to prevent corruption on sudden power loss or process kill.
4. **Resilience**: If `Notes.json` is corrupted or absent, back it up as `Notes.json.bak` and reinitialize gracefully.

## Acceptance Criteria
- [ ] Notes created and edited persist across app restart.
- [ ] Geometry (x, y, width, height) persists accurately.
- [ ] Writing to disk uses atomic rename.
- [ ] No file corruption occurs even if process is killed mid-session.
