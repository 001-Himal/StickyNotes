# ADR-004 — Desktop-Layer Widget Model & Taskbar Omission

## Status: Accepted
**Date:** 2026-10-07  
**Author:** Himal  

## Context
Standard desktop applications appear in the taskbar/dock and float above all windows. A real sticky note lives attached to the physical workspace. It must not block the user's active applications (Chrome, code editors, games), and should not clutter taskbar or Alt+Tab windows.

## Decision
Configure note windows as desktop widgets using platform native window APIs:
- Windows: `WS_EX_TOOLWINDOW` to suppress taskbar/Alt+Tab presence, and `HWND_BOTTOM` / Progman worker anchoring to stay beneath regular windows.
- macOS: `kCGDesktopWindowLevel` and `NSApplicationActivationPolicyAccessory`.
- Linux: `_NET_WM_WINDOW_TYPE_DESKTOP`.

## Alternatives Considered
1. **Always-On-Top floating notes:** Intrusive and disrupts active work.
2. **Normal application window:** Clutters taskbar and Alt+Tab cycling.

## Consequences
- Authentic Windows 7 Sticky Notes experience.
- Notes are instantly visible upon minimizing or returning to desktop, but never obstruct foreground tasks.
