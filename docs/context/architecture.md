# architecture.md

## Overview
Sticky Note is structured as a modular Cargo workspace comprising two compiled binaries and a shared core library:
1. `sticky-note` (Executable: `Sticky Note`): The long-running, ultra-lightweight desktop widget manager. Renders individual note windows onto the desktop layer, responds to note events, and listens for global hotkeys.
2. `sticky-note-settings` (Executable: `Sticky Note Settings`): An on-demand preferences window that reads and writes `config.json`.
3. `sticky-note-core` (Library): Shared data structures, atomic file storage engine, OS window styling abstractions, and configuration models.

```
Sticky Note/
├── Sticky Note.exe          (Main desktop widget manager)
├── Sticky Note Settings.exe (Preferences window - runs on-demand)
├── config.json              (Shared configuration)
└── Sticky Note Notes/       (Local notes storage)
    ├── Notes.json           (Primary registry & metadata index)
    ├── Note 1.json          (Individual self-contained note document)
    ├── Note 2.json          (Double-clickable in File Explorer)
    └── ...
```

## Components
- **Window Management Subsystem (`platform/`)**:
  - Implements OS-specific window layering and stealth behavior:
    - Windows: Applies `WS_EX_TOOLWINDOW` and strips `WS_EX_APPWINDOW` to omit from Taskbar and Alt+Tab. While focused/active, allows typing; on blur (`WM_KILLFOCUS` / deactivate), sinks window to `HWND_BOTTOM` via `SetWindowPos(hwnd, HWND_BOTTOM, ..., SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE)`. Validates monitor bounds on display change (`WM_DISPLAYCHANGE`) to prevent off-screen or disconnected monitor placement.
    - macOS: Sets `NSWindow.level = NSWindow.Level(Int(CGWindowLevelForKey(.desktopWindow)) + 1)` (above wallpaper/desktop icons), `NSApplicationActivationPolicyAccessory` (hides Dock icon), and `NSWindowCollectionBehaviorCanJoinAllSpaces`.
    - Linux: Under X11, sets `_NET_WM_WINDOW_TYPE_UTILITY` with `_NET_WM_STATE_BELOW`, `_NET_WM_STATE_STICKY`, `_NET_WM_STATE_SKIP_TASKBAR`, and `_NET_WM_STATE_SKIP_PAGER`.
- **Slint UI Layer (`ui/`)**:
  - `NoteWindow.slint`: Title header with double-click inline editor, hover-reveal 'X' and '+', multi-line plain text editing area, authentic curled or flat bottom-right corner, and hover-reveal resize grip.
  - `SettingsWindow.slint`: Clean, pastel card preference interface for close behavior, font, default size, and shortcuts.
- **Storage Engine (`storage/`)**:
  - Dual persistence model: Primary `Notes.json` index (cache) and individual self-contained note files (`Note 1.json`, etc.).
  - Atomic file writes using temporary files in the same directory (`.tmp` -> flush -> `std::fs::rename`) preventing corruption during sudden power cut or crash.
  - Debounced auto-save (300ms after user pauses typing).
  - Robust reconciliation: If `Notes.json` is missing or corrupted, automatically rescans and rebuilds from individual note files.
- **Single-Instance & Local IPC Subsystem (`ipc/`)**:
  - Uses a lightweight local named pipe (`\\.\pipe\StickyNote_IPC` on Windows, Unix domain socket on macOS/Linux).
  - Single-instance enforcement: Primary instance hosts IPC server. A second invocation (e.g. from File Explorer double-click or CLI) acts as client, sends `OPEN_FILE <path>` or `NEW_NOTE`, and exits immediately.
  - Settings notification: When `sticky-note-settings` writes `config.json`, it sends `RELOAD_CONFIG` via the IPC pipe, triggering real-time update in `sticky-note` with zero polling.
- **Shortcut Handler (`shortcuts/`)**:
  - Binds OS global hotkeys via `global-hotkey` crate on the event loop. Dispatches `CreateNote` and `OpenSettings` events.

## Data flow
1. Startup: `sticky-note` acquires single-instance lock/named pipe. Reads `config.json` (or initializes defaults), loads `Notes.json` index (or rebuilds from directory), validates monitor bounds against active displays, and spawns each active note window (`is_closed == false`) at its persisted geometry.
2. Typing: Slint triggers text change event → debounced memory update (300ms) → atomic write to both the note file and `Notes.json`.
3. Moving/Resizing: Slint window geometry changes → updates note coordinates `(x, y, w, h)` in state → saved to disk. When resized, updates `last_used_width` and `last_used_height` in `config.json`.
4. File Launch: User double-clicks a note file in File Explorer → secondary process sends path over IPC to primary process → primary process opens or focuses note window → secondary process exits.
5. Settings Change: User launches `Sticky Note Settings` → edits preference → writes to `config.json` → sends `RELOAD_CONFIG` via IPC → `sticky-note` reloads config instantly with 0% idle CPU.

## Boundaries
- `sticky-note` UI owns: View rendering, user text capture, hover detection, drag/resize handles.
- `sticky-note-core` owns: Note data models, file I/O, atomic persistence, configuration schema.
- `sticky-note-settings` owns: Configuration editing interface only; never modifies note content directly.

## Invariants
- UI never performs blocking disk I/O on the main render thread.
- Note windows never elevate to always-on-top; they must always respect the desktop layer.
- No network socket or external telemetry may be opened under any circumstance.
- No single note failure may crash the entire manager process.
