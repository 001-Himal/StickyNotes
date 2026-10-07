# COMP-005: Settings Panel (`SettingsWindow.slint`)

## Purpose
The preferences dialog rendered by `Sticky Note Settings.exe`.

## Visual Design: Sticky Note Paper Aesthetic
Rather than appearing as a harsh grey system dialog, the Settings window is styled as a minimal, compact sticky note card:
- Background: Warm pastel paper tone (`#FFF8D6`).
- Header: Subtle pastel bar (`#F5E8A9`) with title "Sticky Note Settings" and top-right 'X'.
- Dimensions: Compact fixed popup (380 × 320 px).
- Border: Soft rounded corners (6px), subtle ambient shadow.

## Preferences Managed
1. **Close Button ('X') Action:**
   - ( ) Delete note permanently
   - ( ) Close note (keeps saved in `Notes.json`)
   - (•) Ask each time on click (*"Delete or just close?"*)
2. **General:**
   - [x] Launch Sticky Note on Windows/OS startup
3. **Appearance:**
   - **Corner Style:** Curled / Bended corner (Classic) vs. Flat corner (Modern)
   - Font family dropdown (System native sans-serif fonts)
   - Font size slider (12px to 20px)
   - Default note color picker (6 pastel swatches)
4. **Shortcuts:**
   - Global New Note keybinding (`Ctrl+Alt+N`)
   - Global Open Settings keybinding (`Ctrl+Alt+S`)

## Design Invariants
- ❌ No static size presets (new notes dynamically inherit the last resized note's dimensions).
- ❌ No trash recovery engine (strictly minimalist).
- Exits immediately upon closing, releasing all resources.
