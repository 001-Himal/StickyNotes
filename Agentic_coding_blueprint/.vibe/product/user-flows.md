# user-flows.md

## Flow 1: Create New Note
- **Via Shortcut:** User presses `Ctrl + Alt + N` from anywhere in the OS.
- **Via Header Button:** User hovers over any note header; `+` button fades in on top-left; user clicks `+`.
- **Result:** Next available title is generated (`"Note 1"`, `"Note 2"`, etc.). The new note opens at the last resized dimensions (`last_used_width`, `last_used_height`), placed cascading on the desktop with cursor focused.

## Flow 2: Move Note Across Desktop
1. User hovers cursor over note header bar.
2. Mouse cursor changes to Move / Hand.
3. User presses mouse down and drags across desktop.
4. Window moves in real time; coordinates `(x, y)` updated in memory.
5. On mouse release, final position committed to debounced save queue.

## Flow 3: Single-Direction Resize & Dynamic Size Inheritance
1. User hovers over bottom-right 22×22px corner; the corner indicator fades in; cursor becomes `nwse-resize`. (Indicator is **100% invisible** when not hovered).
2. User drags outward: pulling right increases width; pulling down increases height. The top-left corner stays firmly anchored on the desktop.
3. The note clamps at min 180×120px and resizes smoothly.
4. On mouse release, this note's dimensions are saved to `Notes.json`.
5. Simultaneously, `last_used_width` and `last_used_height` in `config.json` update so any *subsequently created note* defaults to this exact size.

## Flow 4: Double-Click Rename & Placeholder Logic
1. User double-clicks header text; swaps to inline input box.
2. User types new title (e.g. "Groceries List").
3. If user clears the title or enters only whitespace, it displays the placeholder `"Write title here..."` or safely reverts to `"Note N"`.
4. User presses `Enter` or clicks away (focus lost) to commit; `Escape` to cancel.
5. Title updated and saved to `Notes.json`.

## Flow 5: Dismiss / Delete Note
1. User hovers cursor over header bar; 'X' close button smoothly fades into view in top-right corner. (Invisible when not hovered).
2. User clicks 'X'.
3. App reads `close_action` from `config.json`:
   - If `delete`: Note permanently removed from `Notes.json` and disk.
   - If `close`: Window closes, but note stays saved in `Notes.json` marked `is_closed: true`.
   - If `ask`: A small themed prompt appears: *"Delete note or just close?"* User chooses `Delete` or `Close`.

## Flow 6: Change Note Color via Right-Click
1. User right-clicks anywhere inside the note.
2. A compact floating palette appears showing 6 pastel swatches (Blue, Green, Pink, Purple, White, Yellow).
3. User clicks a color swatch.
4. Note instantly transitions to the new color theme; `color` saved to `Notes.json`.

## Flow 7: Keyboard Text Formatting
1. User selects text or places cursor in a word.
2. User presses `Ctrl + B` (Bold), `Ctrl + I` (Italic), `Ctrl + U` (Underline), or `Ctrl + L` (Bullet list / alignment).
3. Text formats inline immediately with zero UI toolbars. Auto-saved debounced (300ms).

## Flow 8: Open & Edit Settings
1. User presses `Ctrl + Alt + S` (or opens via Start Menu).
2. `Sticky Note Settings` launches as a minimal, pastel sticky-note-styled popup.
3. User changes settings (e.g. switches close button action, toggles Corner Style between Curled and Flat, toggles Start with Windows).
4. Changes written immediately to `config.json`.
5. User closes settings window; process terminates, releasing all memory.

## Flow 9: Direct File Access & Explorer Double-Click Launch
1. User opens `Sticky Note Notes/` folder in Windows File Explorer.
2. User sees individual note files (`Note 1.json`, `Note 2.json`, ...).
3. User double-clicks any note file.
4. If `Sticky Note.exe` is running in the background, it receives an IPC signal and immediately unhides/restores that note window on the desktop. If not running, it launches and displays that note.
