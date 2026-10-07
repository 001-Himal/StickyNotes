# Product Requirements Document (PRD): Sticky Note

**Status:** APPROVED FOR PLANNING  
**Owner:** Himal  
**Version:** 1.0.0  
**Last Updated:** 2026-10-07  

---

## 1. Executive Summary & Core Philosophy

**"A sticky note should behave like a sticky note, not like an application."**

Sticky Note is a minimalist, native, cross-platform desktop utility designed to resurrect the zero-friction experience of physical paper sticky notes and the classic Windows 7 native Sticky Notes widget. 

Modern note-taking tools have become bloated web apps requiring multi-megabyte Chromium runtimes, account logins, sync servers, and complex formatting toolbars. Sticky Note rejects this entirely. It is a digital piece of paper attached directly to your desktop wallpaper: click, type, drag anywhere, resize, or dismiss. It consumes almost zero system resources, never steals focus, never pollutes the taskbar or Alt+Tab switcher, and quietly stays behind active work applications.

---

## 2. Key Product Principles

1. **Ambient Desktop Citizen:** Lives on the OS desktop layer. Never appears on top of working applications (Chrome, VS Code, games); automatically reveals itself when returning to the desktop.
2. **Invisible Utility Chrome:** No titlebars, no status bars, no ribbon menus, no formatting toolbars. The 'X' (close) and bottom-right resize grip remain completely invisible until the cursor hovers directly over their respective trigger areas.
3. **Decoupled Settings Machinery:** The note is the product; settings are merely the background machinery. Settings live in a separate, lightweight on-demand executable (`Sticky Note Settings`) that does not clutter the note interface.
4. **Local-First, Zero-Bullshit:** 100% offline, self-contained local JSON files. Zero accounts, zero cloud sync, zero telemetry, zero database servers.
5. **Near-Zero Footprint:** Strict resource constraints: <50 MB total RAM, ~0% idle CPU, and <1 second cold startup.

---

## 3. Architecture & File Layout

All binaries, configuration, and data live self-contained in the application's root directory:

```
Sticky Note/
├── Sticky Note.exe          # Main desktop widget manager
├── Sticky Note Settings.exe # Preferences & control center
├── config.json              # Shared preferences file
└── Sticky Note Notes/       # Local notes storage
    ├── Notes.json           # Primary metadata index
    └── note-*.json          # Optional individual note backups
```

### Component Roles
- **`Sticky Note` (Main Widget Process):** Runs continuously in the background (or system tray). Manages note windows on the desktop layer, handles mouse drag/resize events, auto-saves text, and registers system-wide global shortcuts.
- **`Sticky Note Settings` (Control Center Process):** An independent utility launched on-demand via global shortcut (`Ctrl+Alt+S`) or Start Menu. Reads and modifies `config.json`, notifies the main process, and exits when closed.

---

## 4. Feature Specifications

### 4.1. Note Window Behavior & Lifecycle

| Feature | Specification |
|---|---|
| **Layering** | Resides on the desktop level (`HWND_BOTTOM` / Progman worker layer on Windows, `kCGDesktopWindowLevel` on macOS, Desktop type on Linux). Never floats above active applications. |
| **Window Chrome** | Frameless/borderless window with subtle soft shadow. No OS caption bar. |
| **Taskbar / Switcher** | Excluded from Windows Taskbar, macOS Dock, and Alt+Tab / Cmd+Tab app switchers (`WS_EX_TOOLWINDOW`). |
| **Movement** | Dragging from anywhere on the header moves the window smoothly across monitors. |
| **Resizing** | Hovering over the bottom-right corner reveals the resize grip; dragging resizes the note (minimum dimensions: 180×120). |
| **Position Memory** | Note coordinates `(x, y)` and dimensions `(width, height)` persist across reboots. |

### 4.2. Header & Title Behavior

- **Automatic Sequential Naming:** Newly created notes are automatically titled `"Untitled Note 1"`. If that title already exists in `Notes.json`, it increments to `"Untitled Note 2"`, `"Untitled Note 3"`, etc.
- **Double-Click Inline Rename:** Double-clicking the header replaces the title text with an inline input field. Pressing `Enter` or clicking outside commits the new title.
- **Hover Close Button ('X'):**
  - Completely invisible during normal display.
  - Reveals instantly when hovering over the header's top-right region.
  - Behavior when clicked follows user preference configured in Settings (Delete vs. Close vs. Hide).

### 4.3. Text Area & Editing

- **Zero Clutter:** Pure text editing surface. Click anywhere to position cursor and type.
- **Auto-Save:** Keystrokes are buffered and debounced (300ms) before committing atomically to `Notes.json`. No manual save dialogs or Ctrl+S required.
- **Default Appearance:** Classic pastel sticky paper tones (warm yellow `#FFF7D1` default, configurable palette) with legible native typography (Segoe UI, SF Pro, Inter).

### 4.4. Global Shortcuts

Registered system-wide so users can invoke actions even while working inside other applications:
- **`Ctrl + Alt + N` (or `Cmd + Alt + N`):** Instantly spawns a new Sticky Note centered or cascading on the desktop.
- **`Ctrl + Alt + S` (or `Cmd + Alt + S`):** Opens `Sticky Note Settings`.

### 4.5. Settings Utility (`Sticky Note Settings`)

A compact, native preferences window containing:
1. **General:**
   - Launch on system startup (Toggle).
   - Confirm before deleting note (Toggle).
2. **Note Appearance:**
   - Default note dimensions (Width × Height, default 300 × 200).
   - Font family and font size.
   - Default sticky note color.
3. **Behavior:**
   - Close button ('X') action: `Delete Note` | `Close Note` | `Hide Note`.
   - Auto-save debounce interval.
4. **Shortcuts:**
   - Custom keybinding rebinding for New Note and Open Settings.

---

## 5. Non-Functional & Performance Targets

| Metric | Target | Rationale |
|---|---|---|
| **Idle Memory (RAM)** | <50 MB (Target: 15–30 MB) | Must not hog system memory while remaining open permanently. |
| **Idle CPU** | 0.0% | Must not cycle battery or CPU clocks when idle. |
| **Cold Startup Time** | <1.0 second | Must load notes immediately without loading spinners. |
| **Disk Footprint** | <20 MB binary bundle | Pure native compiled binary, no web runtimes. |
| **Network & Telemetry** | Absolute Zero (0 packets) | 100% offline security, private, zero data leakage. |

---

## 6. Approved Tech Stack

- **Language:** Rust (2021 Edition) — ultra-low memory, zero GC pauses, strict safety.
- **GUI Framework:** Slint 1.8+ — native compiled desktop UI framework with negligible footprint.
- **Window Management:** Platform-specific APIs (`windows-sys` on Windows; `cocoa` on macOS; `x11rb`/Wayland on Linux).
- **Shortcut Handler:** `global-hotkey` crate.
- **Serialization:** `serde` + `serde_json` with atomic `.tmp` swap writes.

---

## 7. Out of Scope (Strictly Prohibited)

- ❌ Electron, Node.js, WebViews, or Chromium wrappers.
- ❌ User accounts, passwords, OAuth, or cloud sync.
- ❌ Heavy markdown preview renderers or rich-text WYSIWYG bars.
- ❌ Relational databases (SQLite/PostgreSQL) — JSON is sufficient and human-readable.
- ❌ Always-on-top window layering that obscures other application windows.