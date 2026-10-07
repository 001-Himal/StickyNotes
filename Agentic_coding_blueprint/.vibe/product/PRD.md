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

### 4.1. Note Window Behavior & Sizing

| Feature | Specification |
|---|---|
| **Layering** | Resides on the desktop level (`HWND_BOTTOM` / Progman worker layer on Windows, `kCGDesktopWindowLevel` on macOS, Desktop type on Linux). Never floats above active applications. |
| **Window Chrome** | Frameless/borderless window with subtle soft shadow. No OS caption bar. |
| **Taskbar / Switcher** | Excluded from Windows Taskbar, macOS Dock, and Alt+Tab / Cmd+Tab app switchers (`WS_EX_TOOLWINDOW`). |
| **Movement** | Dragging from anywhere on the header moves the window smoothly across monitors. |
| **Resizing & Dynamic Inheritance** | Hovering over the bottom-right corner reveals the resize grip. Dragging resizes the note (min: 180×120). **Crucial Sizing Rule:** Whenever a user resizes any note, that new `(width, height)` is automatically remembered as the default dimensions for all subsequently spawned notes. No static size presets in settings. |
| **Bottom-Right Corner Appearance** | Features an authentic **bended / curled paper corner** (dog-ear curl look with soft drop-shadow) simulating physical paper, or clean **flat** corner. Togglable in `Sticky Note Settings`. |
| **Per-Note Geometry Persistence** | Each note saves its own exact `(x, y, width, height)` in `Notes.json` so every note reopens exactly as placed. |

### 4.2. Header, Quick-Add & Title Behavior

- **Hover Header Controls:** Both the top-left `+` button and top-right `X` button are hidden by default, smoothly fading into view only when hovering over the header area.
- **Top-Left `+` Button:** Clicking `+` instantly creates and cascades a new sticky note directly next to the current one.
- **Top-Right `X` Button (Configurable):**
  - Configurable via `Sticky Note Settings`:
    1. **Delete Note:** Instantly removes the note from disk.
    2. **Close & Save:** Hides the note window while preserving its contents safely in `Notes.json`.
    3. **Ask on Click:** Displays a tiny, themed confirmation prompt: *"Delete note or just close?"*
- **Automatic Sequential Naming:** Newly created notes are titled `"Untitled Note 1"`, incrementing to `"Untitled Note 2"`, etc., if already taken.
- **Double-Click Inline Rename:** Double-clicking the header replaces the title text with an inline input field. Pressing `Enter` commits the new title; `Escape` cancels.

### 4.3. Text Area, 6-Color Palette & Keyboard Formatting

- **Zero Clutter:** Pure text editing surface. Zero permanent toolbars, buttons, or ribbons.
- **Right-Click 6-Color Pastel Palette:** Right-clicking anywhere on the note opens a compact context menu offering 6 classic Windows 7 palettes (darker header, lighter body):
  1. 🟦 **Blue:** Light blue header (`#8FD1F4`) with pale blue main body (`#C2E6F8`)
  2. 🟩 **Green:** Soft green header (`#B2E89D`) with pale green main body (`#D8F6C8`)
  3. 🌸 **Pink:** Lavender/pink header (`#F5ABC9`) with pale pink main body (`#FCD7E7`)
  4. 🟪 **Purple:** Purple header (`#CEA8ED`) with pale purple main body (`#EAD8FA`)
  5. ⬜ **White:** Light gray header (`#DCDCDC`) with crisp white main body (`#FFFFFF`)
  6. 🟨 **Yellow (Default):** Muted yellow header (`#F6E077`) with light yellow main body (`#FDF1B0`), with `+` top-left and `x` top-right
  *The chosen color is saved individually with that note in `Notes.json`.*
- **Keyboard-Only Formatting (Zero Toolbars):**
  - `Ctrl + B`: Bold text toggle
  - `Ctrl + I`: Italic text toggle
  - `Ctrl + U`: Underline text toggle
  - `Ctrl + L`: Bullet list / alignment toggle
- **Auto-Save:** Keystrokes are buffered and debounced (300ms) before committing atomically to `Notes.json`.

### 4.4. Global Shortcuts

Registered system-wide so users can invoke actions even while working inside other applications:
- **`Ctrl + Alt + N` (or `Cmd + Alt + N`):** Instantly spawns a new Sticky Note on the desktop.
- **`Ctrl + Alt + S` (or `Cmd + Alt + S`):** Opens `Sticky Note Settings`.

### 4.5. Settings Utility (`Sticky Note Settings`)

A minimal, small popup window designed with the **exact same pastel paper aesthetic** as the sticky notes (not a standard gray dialog!):

1. **Close Button ('X') Action:**
   - ○ Delete note permanently
   - ○ Close and keep saved in `Notes.json`
   - ● Ask each time on click (*"Delete or just close?"*)
2. **General:**
   - Launch on system startup (Toggle).
3. **Appearance:**
   - **Corner Style:** Curled / Bended corner (Classic) vs. Flat corner (Modern).
   - Font family and font size.
   - Default note color choice.
4. **Shortcuts:**
   - Display & edit global shortcuts for New Note and Settings.
*(Note: No static size presets; no trash recovery engine).*

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