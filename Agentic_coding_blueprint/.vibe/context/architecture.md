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
    ├── note-001.json        (Individual note content & history)
    └── ...
```

## Components
- **Window Management Subsystem (`platform/`)**:
  - Implements OS-specific window layering:
    - Windows: Hooks behind desktop top-level windows (`HWND_BOTTOM` / Progman worker) with `WS_EX_TOOLWINDOW` to eliminate taskbar and Alt+Tab presence.
    - macOS: Sets `NSWindow.Level = kCGDesktopWindowLevel` and `NSApplicationActivationPolicyAccessory`.
    - Linux: Sets `_NET_WM_WINDOW_TYPE_DESKTOP` or utility window hints.
- **Slint UI Layer (`ui/`)**:
  - `NoteWindow.slint`: Title header with double-click inline editor, hover-reveal 'X', multi-line plain text area, and hover-reveal resize grip.
  - `SettingsWindow.slint`: Clean, tabular preference interface for close behavior, font, default size, and shortcuts.
- **Storage Engine (`storage/`)**:
  - Atomic file writes using temporary files and rename operations to prevent file corruption during sudden shutdown.
  - Debounced auto-save (e.g. 300ms after typing stops).
- **Shortcut Handler (`shortcuts/`)**:
  - Binds OS global hotkeys via `global-hotkey` crate. Dispatches `CreateNote` and `OpenSettings` events.

## Data flow
1. Startup: `sticky-note` reads `config.json` (or initializes defaults), loads `Notes.json`, and spawns each active note window at its last persisted coordinates and dimensions.
2. Typing: Slint triggers text change event → debounced memory update → atomic write to `Notes.json`.
3. Moving/Resizing: Slint window geometry changes → updates note coordinates `(x, y, w, h)` in state → saved to disk.
4. Settings Change: User launches `Sticky Note Settings` → edits preference → writes to `config.json` → sends OS IPC or file-watch event → `sticky-note` reloads config.

## Boundaries
- `sticky-note` UI owns: View rendering, user text capture, hover detection, drag/resize handles.
- `sticky-note-core` owns: Note data models, file I/O, atomic persistence, configuration schema.
- `sticky-note-settings` owns: Configuration editing interface only; never modifies note content directly.

## Invariants
- UI never performs blocking disk I/O on the main render thread.
- Note windows never elevate to always-on-top; they must always respect the desktop layer.
- No network socket or external telemetry may be opened under any circumstance.
- No single note failure may crash the entire manager process.
