# COMP-002: Header Bar (`HeaderBar.slint`)

## Purpose
The draggable title bar of the note providing window movement, inline renaming, and close button disclosure.

## Structure
- Height: Fixed 28px.
- Left padding: 10px. Title text label (`12px semi-bold`, charcoal `#2B2B2B`).
- Right padding: 6px. Close button container (`18×18px`).
- Background: `#F5E8A9` (subtle contrast to `#FFF8D6` body).

## Interactions
1. **Window Drag:** Mouse down on non-button header area starts OS window movement.
2. **Double-Click:** Swaps title text to `TextInput` with auto-focus and pre-selected text.
3. **Commit Title:** Pressing `Enter` or focus-loss commits title to note state and `Notes.json`. Cancel with `Escape`.
4. **Hover Close Button:** Top-right 'X' icon fades in on header hover (`opacity: 1.0`, transition 120ms).
