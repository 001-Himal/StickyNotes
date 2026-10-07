# Sticky Note 🟨

> **A sticky note should behave like physical paper on a desk, not like a computer application.**

Sticky Note is a minimalist, ultra-lightweight, cross-platform desktop utility built with **Rust** and **Slint**. Inspired by the classic Windows 7 native Sticky Notes widget, it lives directly on your desktop wallpaper layer, disappears quietly behind active work applications (Chrome, VS Code), and stays completely out of your taskbar and Alt+Tab switcher.

---

## ✨ Features

- **Ambient Desktop Citizen:** Resides on the desktop layer (`HWND_BOTTOM`). Never covers foreground applications; always waiting when you return to the desktop.
- **Stealth Windowing:** Omitted from the Windows Taskbar, macOS Dock, and Alt+Tab / Cmd+Tab switchers (`WS_EX_TOOLWINDOW`).
- **6 Authentic Pastel Palettes:** Classic Windows 7 two-tone notes (darker header, lighter body):
  - 🟨 Canary Yellow (Default)
  - 🟦 Light Blue
  - 🟩 Soft Green
  - 🌸 Soft Pink
  - 🟪 Pale Purple
  - ⬜ Crisp White
- **Hover-Only Chrome:** Both the top-left `+` (quick-add) button and top-right `X` (close/delete) button remain **100% invisible** until you hover over the header.
- **Curled / Bended Corner:** Authentic 3M post-it dog-ear corner peel in the bottom-right corner, togglable to flat clean mode in settings.
- **Single-Direction Resizing:** Resizing stretches outward to the right and downward from the bottom-right corner while keeping the top-left corner anchored.
- **Dynamic Sizing Inheritance:** Whenever you resize any note, all newly created notes automatically inherit that exact size.
- **Keyboard-Only Formatting:** Zero visual ribbons or toolbars:
  - `Ctrl + B` (Bold), `Ctrl + I` (Italic), `Ctrl + U` (Underline), `Ctrl + L` (Bullet list / alignment).
- **Direct File Explorer Access:** Each note is saved as an individual file in `Sticky Note Notes/` (`Note 1.json`, `Note 2.json`, ...). Double-clicking any note file in Windows File Explorer restores and displays that note on the desktop!
- **Minimal Sticky-Themed Settings Popup:** `Sticky Note Settings.exe` is styled as a matching pastel paper card, not an OS dialog.

---

## 🏗️ Architecture

```
Sticky Note/
├── Sticky Note.exe           # Main desktop widget manager (Rust + Slint)
├── Sticky Note Settings.exe  # On-demand preferences popup (exits immediately when closed)
├── config.json               # Shared application preferences
└── Sticky Note Notes/        # Local data storage
    ├── Notes.json            # Primary index
    ├── Note 1.json           # Individual note file (double-clickable!)
    └── ...
```

---

## ⚡ Performance Targets

| Metric | Target |
|---|---|
| **Idle Memory (RAM)** | < 50 MB total (typically 15–25 MB) |
| **Idle CPU** | 0.0% |
| **Cold Startup Time** | < 1.0 second |
| **Network Requests** | Exactly 0 (100% offline) |

---

## 🛠️ Development

```bash
# Build the workspace
cargo build

# Run main sticky note manager
cargo run --bin sticky-note

# Run settings utility
cargo run --bin sticky-note-settings

# Run tests and linter
cargo test
cargo clippy -- -D warnings
```

---

## 📖 Specifications & Architecture
All project specifications, PRD, component designs, and roadmap are maintained in [`docs/`](file:///d:/Projects/StickyNotes/docs):
- [PRD.md](file:///d:/Projects/StickyNotes/docs/product/PRD.md) — Product Requirements Document
- [design-system.md](file:///d:/Projects/StickyNotes/docs/product/design-system.md) — 6 Pastel Palettes & Corner Curl Specs
- [roadmap.md](file:///d:/Projects/StickyNotes/docs/planning/roadmap.md) — Implementation Milestones
- [tasks/queued/](file:///d:/Projects/StickyNotes/docs/tasks/queued/) — Granular tasks from TASK-001 to TASK-010

