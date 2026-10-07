# project.md

## What
A minimalist, native desktop sticky note utility inspired by the Windows 7 Sticky Notes experience, built with Rust and Slint. It operates as a digital piece of paper residing directly on the OS desktop layer.

## Why
Modern notes applications have become bloated "productivity ecosystems" with multi-megabyte Chromium runtimes, account logins, cloud sync, telemetry, and heavy UI toolbars. Sticky Note recreates the physical, zero-friction post-it note experience: click, type, drag, resize, close. It uses negligible system resources, stays out of the taskbar, and stays strictly on the desktop behind active work applications.

## Users
- Primary: Users who want instant, lightweight, persistent desktop scratchpads without window clutter.
- Secondary: Minimalists and developers who refuse bloated Electron/browser-wrapped background tools.

## Scope
- In:
  - Multi-window lightweight sticky note widgets.
  - Native desktop layer behavior (visible on desktop, hidden under active applications).
  - Clean borderless UI with hover-only controls (corner close 'X', top-left '+', bottom-right single-direction resize handle).
  - Header title auto-incrementing ("Note 1", "Note 2", etc.) with placeholder ("Write note here...") when empty, inline-renamable.
  - Authentic 6 pastel palettes (two-tone header and body) and curled/bended paper corner (togglable to flat in settings).
  - Dynamic sizing inheritance (resizing any note sets default size for new notes).
  - Keyboard-only text formatting (`Ctrl+B`, `Ctrl+I`, `Ctrl+U`, `Ctrl+L`).
  - Completely local persistence with individual note files in `Sticky Note Notes/` (`Note 1.json`, etc.) and `Notes.json` index.
  - Windows File Explorer direct launch: double-clicking note JSON opens it directly in `Sticky Note.exe`.
  - Independent `Sticky Note Settings` executable with shared `config.json` styled in matching sticky paper aesthetic.
  - Global system shortcuts (`Ctrl+Alt+N` for new note, `Ctrl+Alt+S` for settings).
  - Cross-platform support (Windows first-class, macOS, Linux).
  - Strict performance target: <50 MB total RAM, ~0% idle CPU, <1s startup.
- Out:
  - Cloud synchronization, accounts, user logins.
  - Rich text WYSIWYG toolbars, markdown preview panes, or complex formatting palettes.
  - Telemetry, background analytics, network requests.
  - Web views, Electron, Node.js runtimes.

## Success criteria
- [x] Process memory idle below 50 MB (preferably 10-30 MB).
- [x] Idle CPU usage ~0.0%.
- [x] Notes do not appear in Windows Taskbar, macOS Dock, or Alt+Tab/Cmd+Tab app switcher.
- [x] Notes stay on desktop layer beneath normal applications.
- [x] Changes auto-save reliably to local JSON with atomic writes (zero corruption).
- [x] Two clean separate binaries: `Sticky Note` and `Sticky Note Settings`.

