# COMP-003: Text Editor Area (`TextEditor.slint`)

## Purpose
The primary content canvas of the sticky note offering zero-clutter text entry with keyboard formatting.

## Visual & Theme Properties
- Background: Matches note body color (default Canary Yellow `#FFF8D6`, or green, blue, purple, pink, white).
- Padding: `8px 12px 12px 12px`.
- Wrapping: Automatic `WordWrap`.
- Font: Native sans-serif (`Segoe UI`, `SF Pro`, `Roboto`) at 14px default.

## Zero Toolbar Keyboard-Only Formatting
To maintain physical sticky note minimalism, no visual formatting buttons or ribbons are rendered. Formatting is controlled entirely via keyboard shortcuts:
- `Ctrl + B`: Toggle bold text on selection / current word.
- `Ctrl + I`: Toggle italic text on selection / current word.
- `Ctrl + U`: Toggle underline text on selection.
- `Ctrl + L`: Toggle bullet list item or text alignment.

## Right-Click Context Menu
Right-clicking anywhere in the text area opens a minimal floating popup palette displaying 6 color swatches:
1. Yellow (`#FFF8D6`)
2. Green (`#E8F5E9`)
3. Blue (`#E3F2FD`)
4. Purple (`#F3E5F5`)
5. Pink (`#FFEBEE`)
6. White (`#FFFFFF`)

Selecting a color instantly updates the note's theme and writes the new `color` property to `Notes.json`.
