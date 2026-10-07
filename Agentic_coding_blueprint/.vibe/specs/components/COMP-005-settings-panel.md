# COMP-005: Settings Panel (`SettingsWindow.slint`)

## Purpose
The independent preferences dialog rendered by `Sticky Note Settings.exe`.

## Structure & Sections
1. **Window Size:** Fixed compact utility window (440 × 360 px).
2. **Sections:**
   - **General:** Startup toggle ("Start with Windows"), Delete confirmation prompt.
   - **Default Note Size:** Dropdown selector for presets (Compact 200×140, Standard 300×200, Large 420×320, or Custom).
   - **Appearance:** Font family selection, font size slider (12px–20px), default color swatch.
   - **Behavior:** Cross button action radio choices (`Delete Note`, `Close Note`, `Hide Note`).
   - **Shortcuts:** Global key combination display/editor (`Ctrl+Alt+N`, `Ctrl+Alt+S`).
3. **Actions:** "Save & Close", "Restore Defaults".
