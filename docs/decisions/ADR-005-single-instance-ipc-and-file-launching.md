# ADR-005 — Single-Instance Enforcement, Local IPC & File Launching

## Status: Accepted
**Date:** 2026-10-07  
**Author:** Himal  

## Context
Sticky Note supports launching notes by double-clicking individual JSON files (`Note 1.json`, `Note 2.json`) directly in Windows File Explorer or OS file managers. Additionally, the decoupled `Sticky Note Settings.exe` utility modifies `config.json` while `Sticky Note.exe` is running.

Without single-instance coordination:
1. Double-clicking a note file would spawn a redundant second instance of `Sticky Note.exe`. Multiple instances would conflict on OS global hotkeys (`Ctrl+Alt+N`), collide on atomic writes to `Notes.json`, and multiply RAM consumption.
2. Changes made in `Sticky Note Settings.exe` would require `Sticky Note.exe` to either poll `config.json` continuously (violating the 0.0% idle CPU constraint) or require a manual app restart.

## Decision
Implement a lightweight local Inter-Process Communication (IPC) and single-instance listener using OS-native local channels (Windows Named Pipe `\\.\pipe\StickyNote_IPC`, and Unix domain socket on macOS/Linux) via the `interprocess` crate.

### Protocol Specification
The primary `Sticky Note` instance hosts the IPC server endpoint. Secondary invocations and `Sticky Note Settings` act as clients, transmitting newline-delimited command strings:
- `OPEN_FILE <absolute_path>`: Primary process parses the note file, unhides/creates its window, and brings it into focus on the desktop.
- `NEW_NOTE`: Primary process cascades and spawns a new note.
- `RELOAD_CONFIG`: Primary process reloads `config.json` and updates note appearance (font, corner style, etc.) in real-time.

### Single-Instance Lifecycle
1. On startup, `Sticky Note` attempts to connect as an IPC client.
2. If connection succeeds: A primary instance is already active. The secondary process transmits `OPEN_FILE <path>` (if launched with arguments) or `NEW_NOTE`, and terminates immediately (<50ms).
3. If connection fails: No primary instance exists. The process initializes the IPC server, loads persisted notes, registers global hotkeys, and begins the main Slint event loop.

## Alternatives Considered
1. **Windows `WM_COPYDATA` messages:** Windows-only; requires discovering invisible toolwindow HWNDs across processes.
2. **File polling (e.g. checking file mtime every 500ms):** Rejected because continuous polling wastes battery and violates the ~0.0% idle CPU constraint.
3. **Network localhost sockets (`127.0.0.1`):** Rejected because project rules strictly forbid opening network sockets and risk firewall warnings.

## Consequences
- Guaranteed single-instance process hierarchy.
- Seamless Windows File Explorer integration for opening and restoring notes.
- Instant, zero-polling settings updates when `Sticky Note Settings` saves changes.
- Exactly 0 network activity; strictly local operating system IPC.
