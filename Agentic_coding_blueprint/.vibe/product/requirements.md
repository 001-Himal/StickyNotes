# requirements.md

## Functional Requirements
- **FR-1: Window Creation & Management**: Spawn individual frameless note windows on the OS desktop layer (`HWND_BOTTOM` / Progman worker layer on Windows, accessory window with desktop level on macOS, desktop window type on Linux).
- **FR-2: Taskbar & Switcher Concealment**: Invisible in Taskbar, Dock, and Alt+Tab/Cmd+Tab switchers.
- **FR-3: Header & Renaming**: Header displays title (default auto-increment "Untitled Note 1", "Untitled Note 2", ...). Double-click header triggers inline title editing.
- **FR-4: Hover Controls**: 'X' close button reveals only on header hover. Resize grip reveals only on bottom-right hover.
- **FR-5: Drag & Resize**: Dragging header translates window position. Dragging corner grip adjusts window dimensions (min 180x120).
- **FR-6: Local Persistence**: Text and geometry auto-saved atomically to `Sticky Note Notes/Notes.json`.
- **FR-7: Close Button Logic**: Configurable in Settings: `Delete Note`, `Close Note`, or `Hide Note`.
- **FR-8: Settings Application**: Separate executable `Sticky Note Settings` to configure size, font, close behavior, startup, and shortcuts.
- **FR-9: Global Shortcuts**: `Ctrl+Alt+N` creates a new note; `Ctrl+Alt+S` launches settings.

## Non-Functional Requirements
- **NFR-1: Resource Usage**: Total idle RAM <50 MB (target 15-30 MB); idle CPU 0.0%.
- **NFR-2: Startup Time**: <1.0 second from binary execution to window display.
- **NFR-3: Reliability**: Atomic temporary file write ensures zero data loss upon crash or sudden shutdown.
- **NFR-4: Portability**: Self-contained directory without registry dependencies; works on Windows, macOS, and Linux.
