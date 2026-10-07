# FEAT-003 — Atomic Local Persistence

## Goal
Store all notes locally within `Sticky Note Notes/Notes.json` with 100% data integrity, debounced saves, and atomic temporary-file writes.

## Schema: `Notes.json` (Index & Metadata Cache)
```json
{
  "version": 1,
  "last_updated": 1728280050,
  "notes": [
    {
      "id": "note-1",
      "filename": "Note 1.json",
      "title": "Note 1",
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

## Schema: Individual Note File (`Sticky Note Notes/Note 1.json`)
```json
{
  "version": 1,
  "id": "note-1",
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
```

## Requirements
1. **Directory Location**: All data stored under `./Sticky Note Notes/` relative to application root.
2. **Per-Note Files & Index**: In addition to `Notes.json` index, save each note as an individual self-contained file (`Note 1.json`, `Note 2.json`, or `{Sanitized_Title}.json`) in `Sticky Note Notes/`.
3. **Filename Sanitization & Collision**: Titles are sanitized for valid OS filenames (stripping `\ / : * ? " < > |`). Duplicate titles append numeric suffixes (e.g. `Groceries (2).json`).
4. **Data Authority & Resiliency**:
   - `Notes.json` provides rapid cold-boot loading of note windows and positions.
   - Individual note files are self-contained: if `Notes.json` is missing or corrupted, `Sticky Note` rebuilds it automatically from scanning all `*.json` files.
   - If a note file was edited externally (file `mtime` > `Notes.json.last_updated`), the individual note file's data takes precedence.
   - If an individual note file is manually deleted in File Explorer, the index marks the note deleted on startup/refresh.
5. **Explorer Double-Click & Single-Instance IPC**: Double-clicking any note file in Windows File Explorer (or invoking `Sticky Note.exe "<path>"`) connects to the primary running instance's named pipe (`\\.\pipe\StickyNote_IPC`), sends `OPEN_FILE <path>`, and exits. The primary process opens/restores and focuses that note.
6. **Debounced Writing**: On user text changes, debounce disk write by 300ms to eliminate unnecessary disk thrashing.
7. **Atomic File Write**: Always write to a `.tmp` file in the same directory, flush to disk (`sync_all`), and atomically rename over destination to prevent corruption during sudden power loss or process kill.
8. **Corrupted File Fallback**: If an individual file contains invalid JSON, back it up as `{filename}.corrupted.bak` and do not crash the application.

## Acceptance Criteria
- [ ] Notes created and edited persist across app restart.
- [ ] Geometry (x, y, width, height) persists accurately.
- [ ] Individual note files exist in `Sticky Note Notes/` (`Note 1.json`, etc.) with matching content.
- [ ] Double-clicking a note file in File Explorer signals running instance via IPC to open that note.
- [ ] Rebuilding `Notes.json` succeeds if the index file is deleted.
- [ ] Writing to disk uses atomic rename.
- [ ] No file corruption occurs even if process is killed mid-session.
