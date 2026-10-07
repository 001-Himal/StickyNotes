# user-flows.md

## Flow 1: Create New Note
- **Via Shortcut:** User presses `Ctrl + Alt + N` from anywhere in the OS.
- **Via Header Button:** User hovers over any note header; `+` button fades in on top-left; user clicks `+`.
- **Result:** Next available title is generated (`"Untitled Note 1"`, `"Untitled Note 2"`, etc.). The new note opens at the last resized dimensions (`last_used_width`, `last_used_height`), placed cascading on the desktop with cursor focused.

## Flow 2: Move Note Across Desktop
1. User hovers cursor over note header bar.
2. Mouse cursor changes to Move / Hand.
3. User presses mouse down and drags across desktop.
4. Window moves in real time; coordinates `(x, y)` updated in memory.
5. On mouse release, final position committed to debounced save queue.

## Flow 3: Resize Note & Dynamic Size Inheritance
1. User hovers over bottom-right 20×20px corner; diagonal grip fades in; cursor becomes `nwse-resize`.
2. User drags to resize the note.
3. The note clamps at min 180×120px and resizes smoothly.
4. On mouse release, this note's dimensions are saved to `Notes.json`.
5. Simultaneously, `last_used_width` and `last_used_height` in `config.json` update so any *subsequently created note* defaults to this exact size.

## Flow 4: Double-Click Rename Note Header
1. User double-clicks header text.
2. Title switches to inline input box.
3. User types new title (e.g. "Groceries List").
4. User presses `Enter` or clicks away (focus lost).
5. Title updated and saved to `Notes.json`.

## Flow 5: Dismiss / Delete Note
1. User hovers cursor over header bar; 'X' close button smoothly fades into view in top-right corner.
2. User clicks 'X'.
3. App reads `close_action` from `config.json`:
   - If `delete`: Note permanently removed from `Notes.json`.
   - If `close`: Window closes, but note stays saved in `Notes.json` marked `is_closed: true`.
   - If `ask`: A small themed prompt appears: *"Delete note or just close?"* User chooses `Delete` or `Close`.

## Flow 6: Change Note Color via Right-Click
1. User right-clicks anywhere inside the note.
2. A compact floating palette appears showing 6 pastel swatches (Yellow, Green, Blue, Purple, Pink, White).
3. User clicks a color swatch.
4. Note instantly transitions to the new color theme; `color` saved to `Notes.json`.

## Flow 7: Keyboard Text Formatting
1. User selects text or places cursor in a word.
2. User presses `Ctrl + B` (Bold), `Ctrl + I` (Italic), `Ctrl + U` (Underline), or `Ctrl + L` (Bullet list / alignment).
3. Text formats inline immediately with zero UI toolbars. Auto-saved debounced (300ms).

## Flow 8: Open & Edit Settings
1. User presses `Ctrl + Alt + S` (or opens via Start Menu).
2. `Sticky Note Settings` launches as a minimal, pastel sticky-note-styled popup.
3. User changes settings (e.g. switches close button action to "Ask each time", toggles Start with Windows).
4. Changes written immediately to `config.json`.
5. User closes settings window; process terminates, releasing all memory.
