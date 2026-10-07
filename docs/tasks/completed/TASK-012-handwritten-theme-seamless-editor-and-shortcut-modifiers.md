# TASK-012 — Authentic Handwritten Theme, Seamless Canvas & Dynamic Text Modifiers

## Status
- **State**: Completed
- **Date**: 2026-10-07
- **Target OS**: Windows 10/11 x64

---

## 1. Problem Statement & User Bug Reports
Real-world usage of the desktop sticky note application identified 4 critical usability and styling regressions:

1. **Dark Focus Overlay & Contrast Conflict**:
   - In OS Dark Mode, Slint's standard widget `TextEdit` from `std-widgets.slint` automatically rendered an opaque dark-gray (`#2D2D2D`) input box with an active focus underline whenever the note was clicked or focused.
   - When unfocused, text rendered in white (`#FFFFFF`) directly over light pastel yellow (`#FDF1B0`), resulting in near-zero contrast and illegible content.
   - The user demanded a seamless paper experience: yellow should remain yellow everywhere with no dark box or color changes upon focus.

2. **Generic System Sans-Serif Font**:
   - The default font was configured as generic system `Segoe UI` rather than the iconic, warm handwriting typography characteristic of classic Windows 7 Sticky Notes.

3. **Destructive Keyboard Shortcut Behavior**:
   - Pressing in-editor keyboard shortcuts (`Ctrl+B`, `Ctrl+I`, `Ctrl+U`, `Ctrl+L`) inserted raw Markdown syntax strings (`****`, `**`, `__`, `- `) directly into the plain text area, corrupting content rather than applying visual formatting.

4. **Missing Font Size & Window Shortcuts**:
   - No keyboard shortcuts existed to scale font sizes (`Ctrl + =` / `Ctrl + -`) or quickly create notes (`Ctrl + N`) or close notes (`Ctrl + D`) directly from the keyboard.

---

## 2. Root Cause Analysis

1. **Slint Standard Widget Dark Theme Injection**:
   - `TextEdit` from `std-widgets.slint` adheres to Fluent/Material system theme tokens. In Windows dark mode, it injects dark input backgrounds and white text colors, overriding ambient note card background colors.
   - `TextInput` is the underlying low-level primitive which has no built-in background, border, or OS theme overrides, rendering completely transparent over the note's pastel canvas.

2. **Windows 7 Typography Disconnect**:
   - Windows 7 native Sticky Notes specifically utilized Microsoft's pre-installed cursive script font **"Segoe Print"** (`segoepr.ttf`), shipped by default on 100% of Windows installations.

3. **Markdown Syntax Injection vs. Visual Styling**:
   - Shortcuts previously routed through `apply_text_formatting`, which injected raw Markdown symbols (`****`, `_..._`) into the plain string buffer rather than modifying visual rendering attributes.

---

## 3. Implementation Details

### A. Seamless Editor Replacement (`ui/note_window.slint`)
- Replaced `TextEdit` from `std-widgets.slint` with a clean `ScrollView` containing a primitive `TextInput`:
  ```slint
  ScrollView {
      x: 10px;
      y: 4px;
      width: parent.width - 20px;
      height: parent.height - 8px;

      editor := TextInput {
          width: parent.width;
          wrap: word-wrap;
          single-line: false;
          text <=> root.note-content;
          color: root.text-color;
          selection-background-color: #0078D740;
          selection-foreground-color: root.text-color;
          font-family: root.font-family;
          font-size: root.font-size;
          font-weight: root.is-bold ? 700 : 400;
          font-italic: root.is-italic;

          edited => {
              root.content-changed(self.text);
          }
      }
  }
  ```
- Guaranteed 100% transparent text area so the note paper (`root.body-bg`) remains identical when focused, typing, or idle.
- Dark ink color (`root.text-color` = `#2B2B2B` on yellow/white, `#1A334E` on blue) ensures constant high-contrast readability (contrast ratio > 11:1).

### B. Authentic Windows 7 Handwritten Typography
- Default font family updated from `"Segoe UI"` to **`"Segoe Print"`** with 15px default size.
- Updated `crates/sticky-note-core/src/models.rs`, `config.example.json`, and `dist/Sticky Note/config.json`.
- Forwarded `font_family` and `font_size` in `create_and_show_note_window`.

### C. Visual Modifiers & Notebook Ruled Guidelines
- Added persistent boolean properties `is-bold`, `is-italic`, and `is-underlined` to `NoteWindow`, `Note`, and `NoteMetadata`.
- Underline mode (`Ctrl+U`) renders authentic horizontal notebook ruled guidelines across the paper:
  ```slint
  if root.is-underlined: Rectangle {
      width: 100%;
      height: 100%;
      for line-idx in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]: Rectangle {
          y: line-idx * 24px + 6px;
          width: 100%;
          height: 1px;
          background: root.text-color.transparentize(0.85);
      }
  }
  ```

### D. Clean Keyboard Formatting & Navigation Shortcuts
- Updated `key-pressed` in `FocusScope` to handle:
  - `Ctrl + B`: Toggles `is_bold` (700 weight).
  - `Ctrl + I`: Toggles `is_italic`.
  - `Ctrl + U`: Toggles `is_underlined` (ruled guidelines).
  - `Ctrl + L`: Toggles real bullet points (`• `) cleanly per line without markdown hyphens.
  - `Ctrl + =` / `Ctrl + +`: Increases font size dynamically.
  - `Ctrl + -`: Decreases font size dynamically.
  - `Ctrl + N`: Quickly spawns a new note.
  - `Ctrl + D`: Closes/deletes the current note.
  - Right-click on header bar or margins opens the 6-color pastel palette.

### E. Settings Window Bottom Text Contrast Fix
- Completely eliminated `std-widgets.slint` dependencies from `ui/settings_window.slint`.
- Replaced standard `CheckBox` with custom styled check box: label text `"Start Sticky Note automatically on Windows boot"` is explicitly set to `#2B2B2B` (dark charcoal ink) instead of defaulting to white in OS dark mode.
- Replaced `LineEdit` shortcut fields with custom white input cards containing dark `#2B2B2B` text and subtle borders.

### F. Directory-Scoped IPC & Foreground Launch Activation
- Replaced static `StickyNote_IPC` socket name with executable directory hash (`StickyNote_IPC_<hash>`) to eliminate cross-directory process hijacking.
- Added `bring_all_process_windows_to_front()` on startup and IPC commands (`NewNote`, `OpenFile`) so double-clicking `Sticky Note.exe` opens the note directly in front of active windows.
- Refactored `apply_stealth_window_styles()` with `SWP_NOZORDER` to maintain taskbar/Alt+Tab stealth (`WS_EX_TOOLWINDOW`) without burying the window underneath File Explorer.

---

## 4. Verification & Results
- **Unit Tests**: 100% passing across workspace (`cargo test`).
- **Linter**: Zero warnings on all targets (`cargo clippy --all-targets -- -D warnings`).
- **Visuals**: Seamless pastel paper across all states, zero dark boxes, genuine handwriting typography, and functional styling shortcuts.
- **Settings Contrast**: Checkbox text and shortcut inputs in settings are dark charcoal (`#2B2B2B`), high-contrast, and never washed out on yellow card background.
- **Double-Click Launch**: Double-clicking `Sticky Note.exe` instantly displays and activates note in foreground without being buried.
- **Persistence**: `is_bold`, `is_italic`, and `is_underlined` serialize atomically into individual note JSON files.
