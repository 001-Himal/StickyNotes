# product.md

## Vision
Sticky Note is a digital piece of paper attached directly to your desktop. It is not an application to manage; it is an ambient surface for quick thoughts, tasks, and reminders that lives quietly in the background and respects your machine's resources.

## Problem
Modern note-taking tools demand too much mental and system overhead. They load multi-hundred-megabyte runtimes, require sign-in accounts, open giant dashboard windows, and position themselves over work windows like demanding applications. The classic simplicity of the Windows 7 Sticky Note widget—where a note was simply there on the desktop wallpaper—has been lost to cloud and Electron bloat.

## Solution
A native, binary-distributed desktop pair:
1. `Sticky Note`: Runs as a lightweight desktop widget manager. Notes sit on the desktop layer, hide beneath normal application windows, have no taskbar presence, and reveal subtle controls only upon hovering.
2. `Sticky Note Settings`: A tiny auxiliary utility to tweak preferences (close button action, default note size, font, shortcuts) that runs on-demand and closes immediately.

## Key features (v1)
- **Ambient Desktop Placement**: Resides on the desktop layer. Hidden beneath Chrome, VS Code, and other apps. Never interrupts workflow.
- **Taskbar & Switcher Stealth**: Does not pollute the taskbar, system dock, or Alt+Tab switcher.
- **Micro-Interaction Chrome**: Borderless aesthetic. Title and text field always visible. Close ('X') button reveals on header hover; resize handle reveals on bottom-right hover.
- **Header Rename**: Double-click header to rename directly. Default titles sequentially auto-increment ("Untitled Note 1", "Untitled Note 2", etc.).
- **Self-Contained Local Persistence**: Automatically saves notes to `Sticky Note Notes/Notes.json` (and individual JSON files).
- **Global Shortcuts**: Instant note creation (`Ctrl+Alt+N`) and settings access (`Ctrl+Alt+S`).
- **Resource Footprint**: <50 MB RAM, 0% idle CPU.
