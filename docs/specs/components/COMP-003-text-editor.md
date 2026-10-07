# COMP-003: Text Editor Area (`TextEditor.slint`)

## Purpose
The primary content canvas of the sticky note offering zero-clutter text entry with keyboard formatting.

## Visual & Theme Properties
- Background: 100% transparent surface directly on top of the note body card (Canary Yellow `#FDF1B0`, blue `#C2E6F8`, green `#D8F6C8`, pink `#FCD7E7`, purple `#EAD8FA`, white `#FFFFFF`). Never renders dark input boxes or focus outlines in OS dark mode.
- Text Color: High-contrast ink tone (`#2B2B2B` on yellow/white, `#1A334E` on blue, `#20382B` on green, `#4A1E24` on pink, `#392042` on purple).
- Wrapping: Automatic `word-wrap`, multi-line `single-line: false`.
- Font: Classic Windows 7 handwriting font `Segoe Print` at 15px default (scalable).
- Selection: `#0078D740` ambient highlight with ink foreground.
- Ruled Lines: Optional notebook ruled guidelines spaced every 24px when underline mode is enabled.

## Zero Toolbar Keyboard-Only Formatting
To maintain physical sticky note minimalism, no visual formatting buttons or ribbons are rendered. Instead of corrupting plain text with raw markdown characters (`****`, `_`), formatting triggers visual style toggles:
- `Ctrl + B`: Toggles Bold weight (`700` vs `400`)
- `Ctrl + I`: Toggles Italic styling (`font-italic: true`)
- `Ctrl + U`: Toggles Underline / ruled paper guidelines
- `Ctrl + L`: Toggles real bullet points (`• `) on current line(s) cleanly without markdown hyphens
- `Ctrl + =` / `Ctrl + +`: Increases note text size
- `Ctrl + -`: Decreases note text size
- `Ctrl + N`: Spawns a new sticky note
- `Ctrl + D`: Closes/deletes the note

## Right-Click Context Menu
Right-clicking anywhere in the text margins or header bar opens a minimal floating popup palette displaying 6 authentic pastel swatches:
1. Yellow (Header `#F6E077` / Body `#FDF1B0`)
2. Blue (Header `#8FD1F4` / Body `#C2E6F8`)
3. Green (Header `#B2E89D` / Body `#D8F6C8`)
4. Pink (Header `#F5ABC9` / Body `#FCD7E7`)
5. Purple (Header `#CEA8ED` / Body `#EAD8FA`)
6. White (Header `#DCDCDC` / Body `#FFFFFF`)

Selecting a color instantly updates the note's theme and writes the new `color` property to `Notes.json` and the note's JSON file.
