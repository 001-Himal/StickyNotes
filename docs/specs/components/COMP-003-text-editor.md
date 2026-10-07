# COMP-003: Text Editor Area (`TextEditor.slint`)

## Purpose
The primary content canvas of the sticky note offering zero-clutter text entry with keyboard formatting.

## Visual & Theme Properties
- Background: Matches note body color (default Canary Yellow `#FDF1B0`, or green `#D8F6C8`, blue `#C2E6F8`, pink `#FCD7E7`, purple `#EAD8FA`, white `#FFFFFF`).
- Padding: `8px 12px 12px 12px`.
- Wrapping: Automatic `WordWrap`.
- Font: Native sans-serif (`Segoe UI`, `SF Pro`, `Roboto`) at 14px default.

## Zero Toolbar Keyboard-Only Formatting
To maintain physical sticky note minimalism, no visual formatting buttons or ribbons are rendered. Formatting operates on lightweight plain UTF-8 text via keyboard shortcuts:
- `Ctrl + B`: Wrap selection in bold markers (`**`)
- `Ctrl + I`: Wrap selection in italic markers (`*`)
- `Ctrl + U`: Wrap selection in underline markers (`_`)
- `Ctrl + L`: Prepend bullet list prefix (`- `) to the current line

## Right-Click Context Menu
Right-clicking anywhere in the text area opens a minimal floating popup palette displaying 6 authentic pastel swatches:
1. Yellow (Header `#F6E077` / Body `#FDF1B0`)
2. Blue (Header `#8FD1F4` / Body `#C2E6F8`)
3. Green (Header `#B2E89D` / Body `#D8F6C8`)
4. Pink (Header `#F5ABC9` / Body `#FCD7E7`)
5. Purple (Header `#CEA8ED` / Body `#EAD8FA`)
6. White (Header `#DCDCDC` / Body `#FFFFFF`)

Selecting a color instantly updates the note's theme and writes the new `color` property to `Notes.json` and the note's JSON file.
