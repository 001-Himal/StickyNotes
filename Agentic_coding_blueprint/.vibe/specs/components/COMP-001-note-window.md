# COMP-001: Note Window Shell (`NoteWindow.slint`)

## Purpose
The primary frameless container representing an individual sticky note on the OS desktop.

## Sizing & Dynamic Dimensions Inheritance
- **No Static Size Presets:** The app does not force fixed preset sizes.
- **Dynamic Inheritance:** Whenever a user resizes any note via the corner grip, that new dimension `(width, height)` is stored as the current default size in `config.json`. The next newly spawned note automatically adopts this exact size.
- **Individual Note Geometry:** Each note independently saves its own `(x, y, width, height)` in `Notes.json`.
- **Minimum Clamping:** `min-width: 180px`, `min-height: 120px`. Maximum bounded only by current display workspace.

## 6-Color Pastel Palette Themes
Each note can display one of 6 classic pastel themes (selectable via right-click context menu):

| Color | Body Tone | Header Bar Tone | Text Tone |
|---|---|---|---|
| **Canary Yellow (Default)** | `#FFF8D6` | `#F5E8A9` | `#2B2B2B` |
| **Mint Green** | `#E8F5E9` | `#C8E6C9` | `#20382B` |
| **Sky Blue** | `#E3F2FD` | `#BBDEFB` | `#1A334E` |
| **Lavender** | `#F3E5F5` | `#E1BEE7` | `#392042` |
| **Soft Pink** | `#FFEBEE` | `#FFCDD2` | `#4A1E24` |
| **Crisp White** | `#FFFFFF` | `#F0F0F0` | `#2B2B2B` |

## Visual Structure
- Outer container: Frameless, border-radius 4px, subtle ambient shadow.
- Header bar: Top 28px height (contains `+` quick-add, title label, and `X` close).
- Body editor: Flexible height fill.
- Corner resize handle: Bottom-right 20×20px hover hit area.
