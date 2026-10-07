# TASK-011 — Windows Runtime UX & Rendering Bugfixes

## Status
- **State**: Completed
- **Date**: 2026-10-07
- **Target OS**: Windows 10/11 x64

---

## 1. Problem Statement & User Bug Reports
During real-world testing of the compiled release binaries on Windows, 5 distinct user-facing bugs were identified:

1. **Extraneous Terminal Window**:
   - Double-clicking `Sticky Note.exe` or `Sticky Note Settings.exe` spawned an unnecessary black Windows Command Prompt / terminal console.
   - When the user closed this terminal window, the application abruptly terminated.

2. **Window Layering & Alt+Tab Leaks on Startup**:
   - Newly opened notes were initially spawned in front of all active windows and remained visible in the Windows Alt+Tab application switcher instead of quietly sitting on the desktop wallpaper behind user apps.

3. **Global Shortcuts Inactivity**:
   - Pressing `Ctrl+Alt+N` (new note) or `Ctrl+Alt+S` (open settings) from outside the application failed to trigger note creation or settings launch.

4. **In-Note Formatting Shortcuts Inactivity**:
   - In-editor keyboard shortcuts (`Ctrl+B`, `Ctrl+I`, `Ctrl+U`, `Ctrl+L`) did nothing when pressed inside the note body editor.

5. **Close Button Tofu Box Icon (`[ ]`)**:
   - The note and settings window close button rendered as an unprintable box glyph (`[ ]`) instead of a multiplication 'X' cross.

---

## 2. Root Cause Analysis

1. **Terminal Console Subsystem**:
   - Rust/Cargo binaries default to `/SUBSYSTEM:CONSOLE` unless explicitly instructed otherwise. Without `#![windows_subsystem = "windows"]`, the Windows PE loader automatically attaches/allocates a console host (`conhost.exe` / Windows Terminal).

2. **Window Layering & Alt+Tab State Timing**:
   - `find_window_by_title(&window.get_note_title())` via Win32 `FindWindowW` was racing with Slint's internal window creation before the title property was flushed to the OS window manager, causing `find_window_by_title` to return `0` (null).
   - Furthermore, modifying extended window styles via `SetWindowLongPtrW` with `WS_EX_TOOLWINDOW` requires `SetWindowPos` with `SWP_FRAMECHANGED` for Windows DWM and the Shell to immediately update taskbar/Alt+Tab visibility caches. Without this flag, the window remained in Alt+Tab.

3. **Global Hotkey Thread Loop Isolation**:
   - `GlobalHotKeyManager` on Windows creates a hidden window that listens for `WM_HOTKEY`. In the original architecture, `GlobalHotKeyManager::new()` was initialized on the main thread, while the main thread ran Slint's winit event loop. Depending on message pump filtering, `WM_HOTKEY` events were not pumped or clashed. Running `GlobalHotKeyManager` on a dedicated thread with its own `GetMessageW` / `DispatchMessageW` loop guarantees 100% reliable hotkey capture across the entire OS.

4. **Slint Windows Key Event Encoding**:
   - On Windows, pressing `Ctrl + <Key>` produces standard ASCII control codes:
     - `Ctrl+B` -> `\u{0002}` (STX)
     - `Ctrl+I` -> `\u{0009}` (TAB)
     - `Ctrl+U` -> `\u{0015}` (NAK)
     - `Ctrl+L` -> `\u{000c}` (FF)
   - Checking `event.text == "b"` failed because Slint received the control code `\u{0002}`, not `"b"`.
   - In addition, wrapping logic was one-directional rather than toggling wrapped markdown tags (`**`, `*`, `_`).

5. **Segoe UI Unicode Fallback Failure**:
   - The close button used Unicode `U+2715` (Multiplication X `✕`). Standard Segoe UI on Windows installations lacks this glyph in its primary character set, prompting font fallback failure and rendering the missing glyph box ("tofu").

---

## 3. Implementation Details

### A. Subsystem Configuration (`#![windows_subsystem = "windows"]`)
- Added `#![windows_subsystem = "windows"]` at the top of:
  - `crates/sticky-note/src/main.rs`
  - `crates/sticky-note-settings/src/main.rs`
- Suppresses console creation entirely; runs cleanly as a background GUI process.

### B. Process-Wide Desktop Stealth & Sinking
- Implemented `find_process_windows()` and `stealth_and_sink_all_process_windows()` in `crates/sticky-note-core/src/platform/windows.rs`:
  ```rust
  pub unsafe fn apply_stealth_window_styles(hwnd: HWND) {
      let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
      let new_ex_style = (ex_style | WS_EX_TOOLWINDOW as isize) & !(WS_EX_APPWINDOW as isize);
      SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_ex_style);
      SetWindowPos(
          hwnd,
          HWND_BOTTOM,
          0, 0, 0, 0,
          SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
      );
  }
  ```
- Uses `EnumWindows` with `GetWindowThreadProcessId == std::process::id()` to target all note windows without depending on title string matching.
- Scheduled single-shot timers (50ms and 150ms post-show) to re-sink to `HWND_BOTTOM` after Slint/winit finishes its initial window display transitions.
- Re-sinks to desktop layer upon drag completion (`on_window_moved_completed`).

### C. Dedicated Global Hotkey Message Pump
- In `crates/sticky-note/src/hotkeys.rs`:
  - Created a dedicated background thread specifically to instantiate `GlobalHotKeyManager` and run a standard Win32 `GetMessageW` loop.
  - Decoupled `GlobalHotKeyEvent::receiver()` dispatch to safely invoke `slint::invoke_from_event_loop` back into the main thread context.

### D. Formatting Shortcuts & Toggle Unwrapping
- In `ui/note_window.slint`:
  - Updated `key-pressed` handler in `FocusScope` to match both literal characters and ASCII control codes:
    - Bold: `event.text == "b" || event.text == "B" || event.text == "\u{0002}"`
    - Italic: `event.text == "i" || event.text == "I" || event.text == "\u{0009}"`
    - Underline: `event.text == "u" || event.text == "U" || event.text == "\u{0015}"`
    - Bullet: `event.text == "l" || event.text == "L" || event.text == "\u{000c}"`
- In `crates/sticky-note-core/src/formatting.rs`:
  - Enhanced `apply_text_formatting` with toggle unwrapping:
    - `"**test**"` -> `"test"`
    - `"*test*"` -> `"test"`
    - `"_test_"` -> `"test"`

### E. Vector `Path` Close Icon
- In `ui/note_window.slint` and `ui/settings_window.slint`:
  - Replaced `Text { text: "✕"; }` with resolution-independent Slint vector `Path`:
    ```slint
    Path {
        width: 8px;
        height: 8px;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        stroke: close-touch.has-hover ? #E81123 : root.text-color;
        stroke-width: 1.5px;
        commands: "M 0 0 L 8 8 M 8 0 L 0 8";
    }
    ```
  - Zero font dependency; 100% crisp cross across all DPI scales.

---

## 4. Verification & Results
- **Unit Tests**: All 10 tests in `sticky-note-core` passing (`cargo test -p sticky-note-core`).
- **Linter**: Zero clippy warnings (`cargo clippy -- -D warnings`).
- **Subsystem**: No console window appears when launching binaries.
- **Layering**: Notes immediately sink beneath active windows and are omitted from Alt+Tab.
- **Icons**: Close button displays a sharp, vector-drawn cross.
