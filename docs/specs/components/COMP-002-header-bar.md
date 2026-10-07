# COMP-002: Header Bar (`HeaderBar.slint`)

## Purpose
The draggable title bar of the note providing window movement, inline renaming, quick-add (`+`), and close (`X`) disclosure.

## Structure
- Height: Fixed 28px.
- Left edge: Hover-revealed `+` button (`18×18px`, top-left corner).
- Center: Title text label (`12px semi-bold`, charcoal `#2B2B2B`).
- Right edge: Hover-revealed `X` button (`18×18px`, top-right corner).
- Background: Matches the note's header accent tone (e.g. `#F6E077` for yellow note).

## Hover-Only Disclosure
- Both the `+` icon and `X` icon are **100% invisible** (`opacity: 0.0`) during normal ambient desktop display.
- Only when the user moves the mouse cursor over the header bar do both icons smoothly fade into view (`opacity: 1.0`, transition 120ms).

## Title Naming & Placeholder Logic
- Default title automatically increments: `"Note 1"`, `"Note 2"`, `"Note 3"`, etc. (No "Untitled" prefix).
- Double-clicking header swaps text to an inline `TextInput`.
- If the user types nothing, presses only spaces, or clears the field, it preserves the placeholder `"Write title here..."` or safely reverts to `"Note N"`.
- `Enter` commits title; `Escape` cancels; blur (focus lost) commits.

## Click Interactions
1. **Window Drag:** Mouse down on header area (excluding buttons) initiates smooth OS window dragging.
2. **`+` Button Click:** Spawns and cascades a new note adjacent to the active note.
3. **`X` Button Click:** Executes configured close behavior (`delete`, `close`, or `ask` prompt dialog).
