# requirements.md

## Functional Requirements
- **FR-1: Window Creation & Management**: Spawn individual frameless note windows on the OS desktop layer (`HWND_BOTTOM` / Progman worker layer on Windows, accessory window with desktop level on macOS, desktop window type on Linux).
- **FR-2: Taskbar & Switcher Concealment**: Invisible in Taskbar, Dock, and Alt+Tab/Cmd+Tab switchers (`WS_EX_TOOLWINDOW`).
- **FR-3: Header & Renaming**: Header displays title (default auto-increment "Note 1", "Note 2", ...; placeholder "Write note here..." when empty). Double-click or click header triggers inline title editing.
- **FR-4: Hover Controls**: Top-left '+' (quick-add) button and top-right 'X' (close/delete) button remain 100% invisible until hovering over header. Resize indicator is 100% invisible until hovering over bottom-right corner.
- **FR-5: Drag & Single-Direction Resize**: Dragging header translates window position. Dragging corner grip expands rightward and downward only (single direction, top-left anchored; min 180x120).
- **FR-6: Dynamic Sizing Inheritance**: Resizing any note immediately updates default dimensions for all future newly spawned notes (no static size presets in settings).
- **FR-7: Authentic Two-Tone Color Themes**: 6 classic palettes with darker header and lighter body (Yellow, Blue, Green, Pink, Purple, White) switchable via right-click context menu.
- **FR-8: Curled / Flat Corner Appearance**: Bottom-right paper corner supports authentic folded dog-ear curl look, togglable to clean flat corner in Settings.
- **FR-9: Keyboard-Only Formatting**: Zero visual toolbars. Supports `Ctrl+B` (Bold), `Ctrl+I` (Italic), `Ctrl+U` (Underline), and `Ctrl+L` (Bullet list / alignment).
- **FR-10: Local Atomic Persistence & File Launch**: Notes saved as individual JSON files in `Sticky Note Notes/` (`Note 1.json`, etc.) with `Notes.json` index. Double-clicking any note file in Windows File Explorer launches/signals `Sticky Note.exe` in the background to open and display that note.
- **FR-11: Close Button Logic**: Configurable in Settings: `Delete Note` (instant delete, no trash recovery), `Close Note` (hide/persist), or `Ask prompt` modal.
- **FR-12: Settings Application**: Separate executable `Sticky Note Settings` styled as a minimal matching paper card popup to configure default close action, launch at boot, curled corner toggle, font size, and hotkeys.
- **FR-13: Global Shortcuts**: `Ctrl+Alt+N` creates a new note; `Ctrl+Alt+S` launches settings.

## Non-Functional Requirements
- **NFR-1: Resource Usage**: Total idle RAM <50 MB (target 15-30 MB); idle CPU 0.0%.
- **NFR-2: Startup Time**: <1.0 second from binary execution to window display.
- **NFR-3: Reliability**: Atomic temporary file write ensures zero data loss upon crash or sudden shutdown.
- **NFR-4: Portability**: Self-contained directory without registry dependencies; works on Windows, macOS, and Linux.
