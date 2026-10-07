# user-flows.md

## Flow 1: Create New Note via Shortcut
1. User presses `Ctrl + Alt + N` (or `Cmd + Alt + N`) from any application.
2. `sticky-note` process catches global shortcut.
3. Next available title determined (`"Untitled Note 1"`, `"Untitled Note 2"`, etc.).
4. New note spawned cascading on the desktop layer.
5. Text cursor immediately focused in the body editor.

## Flow 2: Move Note Across Desktop
1. User hovers cursor over note header bar.
2. Mouse cursor changes to Move / Hand.
3. User presses mouse down and drags across desktop.
4. Window moves in real time; coordinates `(x, y)` updated in memory.
5. On mouse release, final position committed to debounced save queue.

## Flow 3: Rename Note Header
1. User double-clicks header text.
2. Title switches to inline input box.
3. User types new title (e.g. "Groceries List").
4. User presses `Enter` or clicks away (focus lost).
5. Title updated and saved to `Notes.json`.

## Flow 4: Dismiss / Delete Note
1. User hovers cursor over header bar.
2. 'X' close button smoothly fades into view.
3. User clicks 'X'.
4. App reads `config.json` close behavior:
   - If `Delete`: Note removed from `Notes.json` (prompt confirmation if configured) and window closed.
   - If `Close` / `Hide`: Note marked hidden in `Notes.json` and window dismissed.

## Flow 5: Open & Edit Settings
1. User presses `Ctrl + Alt + S` (or opens via Start Menu / System Tray).
2. `Sticky Note Settings` window launches independently.
3. User adjusts preferences (e.g. change Close button behavior to "Delete Note", change default size to 320x240).
4. Changes written immediately to `config.json`.
5. User closes settings window; settings process terminates, freeing all memory.
