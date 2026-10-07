# COMP-001: Note Window Shell (`NoteWindow.slint`)

## Purpose
The primary frameless container representing an individual sticky note on the OS desktop.

## Geometry & Multi-Size Presets
The window must seamlessly accommodate varying dimensions, from compact scratchpads to expansive workspace notes:

| Size Preset | Dimensions (W × H) | Best Use Case |
|---|---|---|
| **Compact / Tiny** | 200 × 140 px | Single reminders, quick phone numbers, one-liners |
| **Standard / Medium (Default)** | 300 × 200 px | Daily to-do lists, short paragraphs, meeting snippets |
| **Expanded / Large** | 420 × 320 px | Detailed instructions, multi-step checklists, extensive notes |
| **Freeform Resizing** | Min: 180 × 120 px, Max: Display bounds | User-adjusted via bottom-right grip |

## Visual Structure
- Outer container: Frameless, border radius 4px, subtle ambient shadow.
- Header region (top 28px): Contains title and hover 'X'.
- Body region: Fills remaining vertical height.
- Footer corner: 20×20px interactive resize zone.

## States
- `Normal`: Ambient display on desktop, header and body text visible.
- `HeaderHover`: 'X' close button opacity transitions from 0.0 to 1.0.
- `CornerHover`: Resize grip opacity transitions from 0.0 to 1.0; cursor changes to `nwse-resize`.
- `EditingTitle`: Header title swaps to inline input field.
- `Focused`: Text caret active in body area.
