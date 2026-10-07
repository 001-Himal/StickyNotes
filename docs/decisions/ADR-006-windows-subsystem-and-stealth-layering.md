# ADR-006 — Windows Subsystem, Stealth Layering, and Event Pipeline Architecture

## Context
When deploying Sticky Note as a production desktop utility on Windows, five platform-specific integration issues arose:
1. Binaries compiled under Cargo's default subsystem allocate an unwanted console/terminal window host (`conhost.exe`).
2. Window stealth styling (`WS_EX_TOOLWINDOW`) and desktop sinking (`HWND_BOTTOM`) failed to detach from the Alt+Tab switcher when relying on window title matching and unforced frame updates.
3. System-wide global hotkeys (`RegisterHotKey`) clashed with Slint's winit event loop when registered on the primary UI thread without a dedicated Win32 message pump.
4. Slint key event dispatching on Windows translates `Ctrl + <Key>` to ASCII control characters (`\u{0002}`, `\u{0009}`, `\u{0015}`, `\u{000c}`), causing standard character checks to fail.
5. Unicode glyph `U+2715` (Multiplication X `✕`) experienced font fallback failure under Segoe UI on standard Windows configurations, displaying an unprintable box / tofu (`[ ]`).

## Decision

### 1. GUI Subsystem Enforcement
Set `#![windows_subsystem = "windows"]` in both `sticky-note` and `sticky-note-settings` entry points. This instructs the linker to set PE Subsystem to `IMAGE_SUBSYSTEM_WINDOWS_GUI`, preventing Windows from attaching or creating a console window.

### 2. PID-Based Enumeration & `SWP_FRAMECHANGED`
Instead of brittle title-based matching (`FindWindowW`), enumerate all top-level windows belonging to `std::process::id()` via `EnumWindows`.
Apply `WS_EX_TOOLWINDOW` and strip `WS_EX_APPWINDOW`, then immediately trigger:
```c
SetWindowPos(hwnd, HWND_BOTTOM, 0, 0, 0, 0, SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);
```
`SWP_FRAMECHANGED` forces the Desktop Window Manager (DWM) and Windows Shell to re-evaluate window chrome and immediately unregister the window from the Taskbar and Alt+Tab switcher.
Single-shot timers (50ms and 150ms) are scheduled post-show to enforce bottom z-order after winit finishes initial display events.

### 3. Dedicated Hotkey Thread & Message Pump
Run `GlobalHotKeyManager` on an isolated worker thread with its own `GetMessageW` / `TranslateMessage` / `DispatchMessageW` loop. Forward hotkey triggers to the Slint event loop using thread-safe `slint::invoke_from_event_loop`. This completely decouples global shortcut interception from UI rendering and modal dialog states.

### 4. Control Code Keyboard Handling
In Slint event handlers, match both printable characters and ASCII control codes:
- Bold: `"b"`, `"B"`, `"\u{0002}"`
- Italic: `"i"`, `"I"`, `"\u{0009}"`
- Underline: `"u"`, `"U"`, `"\u{0015}"`
- Bullet: `"l"`, `"L"`, `"\u{000c}"`
Formatting functions toggle wrapped tags on/off.

### 5. Vector Path Close Glyph
Replace font-dependent text glyphs with a native Slint `Path` cross (`M 0 0 L 8 8 M 8 0 L 0 8`). This eliminates all font dependencies and guarantees pixel-perfect rendering across all DPI settings.

## Consequences
- **Positive**: Zero console window popups; notes never steal focus or leak into Alt+Tab; global and editor shortcuts operate reliably; close button is crisp without glyph boxes.
- **Negative**: Win32 platform code requires maintaining unsafe FFI calls against `windows-sys`.
